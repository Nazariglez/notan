use crate::app::{App, MyAny};
use downcast_rs::{impl_downcast, Downcast};
use hashbrown::HashMap;
use indexmap::{IndexMap, IndexSet};
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

//https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=573ad7da1d081b586ce1997201674ef7

//Handle == Res
pub struct Asset<A> {
    id: String,
    loaded: Arc<AtomicBool>,
    res: Arc<RwLock<A>>,
}

impl<A> Asset<A>
where
    A: Default,
{
    pub fn lock(&self) -> MappedRwLockReadGuard<'_, A> {
        RwLockReadGuard::map(self.res.read(), |unlocked| unlocked)
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::SeqCst)
    }
}

struct LoadState {
    loaded: Arc<AtomicBool>,
    // asset: Arc<RwLock<AssetLoader>>,
}

#[derive(Default)]
pub struct AssetStorage {
    assets: HashMap<TypeId, HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl AssetStorage {
    pub fn init<A>(&mut self, id: &str, asset: A)
    //init load?
    where
        A: Send + Sync + 'static,
    {
        let type_id = TypeId::of::<A>();
        let mut list = self.assets.entry(type_id).or_insert(HashMap::new());
        list.insert(id.to_string(), Arc::new(RwLock::new(asset)));
    }

    pub fn get<A>(&self, id: &str) -> Result<Asset<A>, String>
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
            .map(|a| Asset {
                id: id.to_string(),
                loaded: Arc::new(AtomicBool::new(false)),
                res: a.clone().downcast::<RwLock<A>>().unwrap(),
            })
    }
}

pub struct LoadManager {
    loaders: HashMap<TypeId, Arc<AssetLoader>>,
    extensions: HashMap<String, TypeId>,
    storage: AssetStorage,
}

impl LoadManager {
    pub fn new() -> Self {
        let mut manager = Self {
            loaders: HashMap::new(),
            extensions: HashMap::new(),
            storage: AssetStorage::default(),
        };

        manager.add_loader::<BlobLoader>();

        manager
    }

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

    pub fn load(&mut self, paths: &[&str]) -> Result<(), String> {
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

        Ok(())
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

#[derive(Default)]
pub struct BlobLoader;
impl AssetLoader for BlobLoader {
    fn load(&self, id: &str, storage: &mut AssetStorage) {
        storage.init(id, Blob(vec![]));
    }
    fn extensions(&self) -> &[&str] {
        &["blob"]
    }
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
struct Loader {
    // assets: IndexMap<TypeId, HashMap<String, Box<AssetLoader>>>
}

impl Loader {
    /// Creates a new Loader
    pub fn new() -> Self {
        Default::default()
    }

    // /// Returns the assets of the type passed
    // pub fn get<T: AssetLoader + 'static>(&self, id: &str) -> Option<&T> {
    //     let map = self.assets
    //         .get(&TypeId::of::<T>());
    //
    //     Some(match map {
    //         Some(list) => match list.get(id) {
    //             Some(asset) => asset.as_any().downcast_ref().unwrap(),
    //             _ => return None,
    //         },
    //         _ => return None
    //     })
    // }
}
