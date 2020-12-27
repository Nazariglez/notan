use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::list::*;
use super::storage::*;
use crate::AppState;
use hashbrown::HashMap;
use std::any::TypeId;
use std::path::Path;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct Loader {
    //change to AssetParser?
    extensions: Vec<String>,
    parser: Option<LoaderCallback>,
    type_id: Option<TypeId>,
}

impl Loader {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn output<A>(mut self) -> Self
    where
        A: Send + Sync + 'static,
    {
        self.type_id = Some(TypeId::of::<A>());
        self
    }

    pub fn from_extension(mut self, ext: &str) -> Self {
        self.extensions.push(ext.to_string());
        self
    }

    pub fn from_extensions(mut self, exts: &[&str]) -> Self {
        for ext in exts {
            self.extensions.push(ext.to_string());
        }
        self
    }

    pub fn use_parser<H, Params>(mut self, handler: H) -> Self
    where
        H: LoaderHandler<Params>,
    {
        self.parser = Some(handler.callback());
        self
    }

    fn apply(self, manager: &mut AssetManager2) -> Result<(), String> {
        let Loader {
            extensions,
            parser,
            type_id,
        } = self;

        if extensions.is_empty() {
            return Err("Loader without extensions associated.".to_string());
        }

        let type_id =
            type_id.ok_or_else(|| "Loader without output type associated.".to_string())?;
        let mut parser = parser.ok_or_else(|| "Loader without parser associated.".to_string())?;
        parser.set_type_id(type_id);

        extensions.iter().for_each(|ext| {
            manager.loaders.insert(ext.to_string(), parser.clone());
        });

        Ok(())
    }
}

#[derive(Clone)]
pub enum LoaderCallback {
    Basic(
        Option<TypeId>,
        Rc<dyn Fn(&str, Vec<u8>, &mut AssetStorage2) -> Result<(), String>>,
    ),
}

pub trait LoaderHandler<Params> {
    fn callback(self) -> LoaderCallback;
}

macro_rules! loader_handler {
    ($variant:expr, $($param:ident),*) => {
        #[allow(unused_parens)]
        impl<F> LoaderHandler<(&str, Vec<u8>, &mut AssetStorage2, $(&mut $param),*)> for F
        where
            F: Fn(&str, Vec<u8>, &mut AssetStorage2, $(&mut $param),*) -> Result<(), String> + 'static
        {
            fn callback(self) -> LoaderCallback {
                $variant(None, Rc::new(self))
            }
        }
    }
}

loader_handler!(LoaderCallback::Basic,);

impl LoaderCallback {
    pub(crate) fn exec(
        &self,
        id: &str,
        data: Vec<u8>,
        storage: &mut AssetStorage2,
    ) -> Result<(), String> {
        use LoaderCallback::*;
        match self {
            Basic(_, cb) => cb(id, data, storage),
        }
    }

    fn set_type_id(&mut self, type_id: TypeId) {
        use LoaderCallback::*;
        let ty = match self {
            Basic(ref mut ty, _) => ty,
        };

        *ty = Some(type_id);
    }

    fn type_id(&self) -> Option<TypeId> {
        use LoaderCallback::*;
        match self {
            Basic(ty, _) => *ty,
        }
    }
}

pub struct AssetManager2 {
    storage: AssetStorage2,
    loaders: HashMap<String, LoaderCallback>,
    byte_loader: LoaderCallback,
}

impl AssetManager2 {
    pub fn new() -> Self {
        let bytes_id = TypeId::of::<Vec<u8>>();
        let byte_loader = LoaderCallback::Basic(
            Some(bytes_id),
            Rc::new(|id, bytes, storage| {
                storage.parse::<Vec<u8>>(id, bytes);
                Ok(())
            }),
        );

        Self {
            loaders: HashMap::new(),
            storage: AssetStorage2::default(),
            byte_loader,
        }
    }

    pub fn tick(&mut self) -> Result<(), String> {
        //TODO pub(crate)
        if let Some(mut to_update) = self.storage.try_load() {
            while let Some((id, data)) = to_update.pop() {
                let ext = Path::new(&id)
                    .extension()
                    .map(|ext| ext.to_str().unwrap())
                    .unwrap_or("");

                let loader = match self.loaders.get(ext) {
                    Some(loader) => loader,
                    None => {
                        notan_log::warn!(
                            "Not found a loader for '{}', loading as bytes (Vec<u8>)",
                            id
                        );
                        &self.byte_loader
                    }
                };

                loader.exec(&id, data, &mut self.storage)?;
                self.storage.clean_asset(&id)?;
            }
        }

        Ok(())
    }

    pub fn add_loader(&mut self, loader: Loader) {
        if let Err(e) = loader.apply(self) {
            notan_log::error!("{}", e);
        }
    }

    fn load(&mut self, id: &str) -> Result<DoneSignal, String> {
        let ext = Path::new(id)
            .extension()
            .map(|ext| ext.to_str().unwrap())
            .unwrap_or("");

        let loader = match self.loaders.get(ext) {
            Some(loader) => loader,
            None => {
                notan_log::warn!(
                    "Not found a loader for '{}', loading as bytes (Vec<u8>)",
                    id
                );
                &self.byte_loader
            }
        };

        Ok(match loader.type_id() {
            Some(type_id) => self.storage.register(id, type_id),
            None => return Err("Loader without output type id".to_string()),
        })
    }

    pub fn load_asset<A>(&mut self, id: &str) -> Result<Asset2<A>, String>
    where
        A: Send + Sync + 'static,
    {
        let _ = self.load(id)?;
        self.storage.get(id, true)
    }

    pub fn load_list(&mut self, paths: &[&str]) -> Result<AssetList2, String> {
        let mut list = AssetList2::new(self.storage.tracker.clone());
        for id in paths {
            let loaded = self.load(id)?;
            list.insert(id, loaded);
        }
        Ok(list)
    }
}

/// Read-Only representation of an asset loaded from a file
#[derive(Clone, Debug)]
pub struct Asset<A>
where
    A: Send + Sync,
{
    pub(crate) id: String,
    pub(crate) loaded: Arc<AtomicBool>,
    pub(crate) res: Arc<RwLock<A>>,
}

impl<A> Asset<A>
where
    A: Send + Sync + Default,
{
    /// Returns the id of this asset, used to loaded
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns a read-only access to the asset
    pub fn lock(&self) -> MappedRwLockReadGuard<'_, A> {
        RwLockReadGuard::map(self.res.read(), |unlocked| unlocked)
    }

    /// Returns true if the asset is already loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::SeqCst)
    }

    /// Create a new asset from custom data
    pub fn from_data(id: &str, data: A) -> Asset<A> {
        Asset {
            id: id.to_string(),
            loaded: Arc::new(AtomicBool::new(true)),
            res: Arc::new(RwLock::new(data)),
        }
    }
}

impl<A> PartialEq for Asset<A>
where
    A: Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<A> Eq for Asset<A> where A: Send + Sync {}

impl<A> Default for Asset<A>
where
    A: Default + Send + Sync,
{
    fn default() -> Asset<A> {
        let id = std::any::type_name::<A>();
        Asset::from_data(id, Default::default())
    }
}

/// Read-Only representation of an asset loaded from a file
#[derive(Clone, Debug)]
pub struct Asset2<A>
where
    A: Send + Sync,
{
    pub(crate) id: String,
    pub(crate) loaded: DoneSignal,
    pub(crate) res: Arc<RwLock<Option<A>>>,
}

impl<A> Asset2<A>
where
    A: Send + Sync,
{
    /// Returns the id of this asset, used to loaded
    pub fn id(&self) -> &str {
        &self.id
    }
    //
    // pub fn lock(&self) -> MappedRwLockReadGuard<'_, Option<A>>  {
    //     RwLockReadGuard::map(self.res.read(), |unlocked| unlocked)
    // }

    /// Returns true if the asset is already loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded.is_done()
    }

    /// Create a new asset from custom data
    pub fn from_data(id: &str, data: A) -> Asset2<A> {
        Self::from_option(id, Some(data))
    }

    #[inline]
    pub(crate) fn from_option(id: &str, data: Option<A>) -> Asset2<A> {
        Asset2 {
            id: id.to_string(),
            loaded: DoneSignal::from_bool(true),
            res: Arc::new(RwLock::new(data)),
        }
    }
}

impl<A> PartialEq for Asset2<A>
where
    A: Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<A> Eq for Asset2<A> where A: Send + Sync {}
