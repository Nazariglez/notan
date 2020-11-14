use crate::app::App;
use downcast_rs::{impl_downcast, Downcast};
use hashbrown::HashMap;
use indexmap::{IndexMap, IndexSet};
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Read-Only representation of an asset loaded from a file
#[derive(Clone, Debug)]
pub struct Asset<A>
where
    A: Send + Sync,
{
    id: String,
    loaded: Arc<AtomicBool>,
    res: Arc<RwLock<A>>,
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

#[derive(Clone)]
struct LoadState {
    loaded: Arc<AtomicBool>,
    asset: Arc<dyn Any + Send + Sync>,
}

impl LoadState {
    fn new<A>(asset: A) -> Self
    where
        A: Send + Sync + 'static,
    {
        Self {
            loaded: Arc::new(AtomicBool::new(false)),
            asset: Arc::new(RwLock::new(asset)),
        }
    }
}

#[derive(Default)]
pub struct AssetStorage {
    loader: Option<Loader>,
    assets: HashMap<TypeId, HashMap<String, LoadState>>,
}

impl AssetStorage {
    fn store<A>(&mut self, id: &str, asset: A)
    where
        A: Send + Sync + 'static,
    {
        let type_id = TypeId::of::<A>();
        let state = LoadState::new(asset);

        // In case that exists a loader append the state to it
        if let Some(loader) = &mut self.loader {
            let mut list = loader.assets.entry(type_id).or_insert(HashMap::new());
            list.insert(id.to_string(), state.clone());
        }

        let mut list = self.assets.entry(type_id).or_insert(HashMap::new());
        list.insert(id.to_string(), state);
    }

    fn get<A>(&self, id: &str) -> Result<Asset<A>, String>
    where
        A: Send + Sync + 'static,
    {
        let storage = match self.assets.get(&TypeId::of::<A>()) {
            Some(map) => map,
            _ => return Err("Invalid asset type".to_string()),
        };

        storage
            .get(id)
            .ok_or("Invalid asset id".to_string())
            .map(|state| Asset {
                id: id.to_string(),
                loaded: state.loaded.clone(),
                res: state.asset.clone().downcast::<RwLock<A>>().unwrap(),
            })
    }
}

/// Assets and loaders can be set and get from this struct
pub struct LoadManager {
    loaders: HashMap<TypeId, Arc<AssetLoader>>,
    extensions: HashMap<String, TypeId>,
    storage: AssetStorage,
}

impl LoadManager {
    /// Returns a new manager
    pub fn new() -> Self {
        let mut manager = Self {
            loaders: HashMap::new(),
            extensions: HashMap::new(),
            storage: AssetStorage::default(),
        };

        manager.add_loader::<BlobLoader>();

        manager
    }

    /// Adds a new [AssetLoader]
    pub fn add_loader<L>(&mut self)
    where
        L: AssetLoader + Default + 'static,
    {
        let loader = L::default();
        let type_id = TypeId::of::<L>();

        for ext in loader.extensions() {
            self.extensions.insert(ext.to_string(), type_id);
        }

        self.loaders.insert(type_id, Arc::new(loader));
    }

    /// Starts loading a file and returns an [Asset]
    pub fn load_asset<A>(&mut self, path: &str) -> Result<Asset<A>, String>
    where
        A: Send + Sync + 'static,
    {
        let _ = self.load(&[path])?;
        self.get(path)
    }

    pub fn get<A>(&self, path: &str) -> Result<Asset<A>, String>
    where
        A: Send + Sync + 'static,
    {
        self.storage.get(path)
    }

    /// Starts loading a list of [Asset]s and return a [Loader] to get them and check the progress
    pub fn load(&mut self, paths: &[&str]) -> Result<Loader, String> {
        self.storage.loader = Some(Default::default());

        paths.iter().for_each(|p| {
            let ext = Path::new(p)
                .extension()
                .map(|ext| ext.to_str().unwrap())
                .unwrap_or("blob");

            let loader = match self.get_loader(ext) {
                Ok(loader) => loader,
                Err(err) => {
                    notan_log::warn!("Asset: {} -> {} -> loading as Blob", p, err);
                    self.get_loader("blob").unwrap()
                }
            }
            .clone();

            loader.load(p, &mut self.storage);
        });

        let loader = self
            .storage
            .loader
            .take()
            .ok_or("Loader cannot be extracted from LoadManager".to_string())?;

        Ok(loader)
    }

    fn get_loader(&self, ext: &str) -> Result<&Arc<AssetLoader>, String> {
        let type_id = match self.extensions.get(ext) {
            Some(type_id) => type_id,
            _ => return Err("Invalid extension".to_string()),
        };

        let loader = match self.loaders.get(type_id) {
            Some(loader) => loader,
            _ => return Err("Invalid asset type".to_string()),
        };

        Ok(loader)
    }
}

pub trait AssetLoader
where
    Self: Send + Sync + Downcast,
{
    fn load(&self, id: &str, storage: &mut AssetStorage);
    // fn from_bytes(data: &[u8]) -> Result<Self, String> where Self: Sized;
    fn extensions(&self) -> &[&str];
}

impl_downcast!(AssetLoader);

#[derive(Debug, Default)]
pub struct Blob(pub Vec<u8>);

impl Deref for Blob {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
pub struct BlobLoader;
impl AssetLoader for BlobLoader {
    fn load(&self, id: &str, storage: &mut AssetStorage) {
        storage.store(id, Blob(vec![]));
    }
    fn extensions(&self) -> &[&str] {
        &["blob"]
    }
}

#[derive(Default, Clone)]
pub struct Loader {
    assets: HashMap<TypeId, HashMap<String, LoadState>>,
}

impl Loader {
    pub fn is_loaded(&self) -> bool {
        unimplemented!()
    }

    pub fn progress(&self) -> f32 {
        unimplemented!() //todo 0.0 to 1.0
    }

    //pub fn get() {} //Returns Asset<A>
    //pub fn take() {} //Returns Asset<A> and remove it from the list
}
