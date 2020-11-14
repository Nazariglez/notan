use crate::app::App;
use downcast_rs::{impl_downcast, Downcast};
use futures::prelude::*;
use futures::Future;
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

struct LoadState {
    fut: Box<dyn Future<Output = Result<Vec<u8>, String>>>,
    loaded: Arc<AtomicBool>,
    asset: Arc<dyn Any + Send + Sync>,
}

impl LoadState {
    fn new<A>(asset: A, fut: Box<dyn Future<Output = Result<Vec<u8>, String>>>) -> Self
    where
        A: Send + Sync + 'static,
    {
        Self {
            fut,
            loaded: Arc::new(AtomicBool::new(false)),
            asset: Arc::new(RwLock::new(asset)),
        }
    }

    fn tracker(&self) -> LoadTracker {
        LoadTracker {
            loaded: self.loaded.clone(),
            asset: self.asset.clone(),
        }
    }
}

#[derive(Clone)]
struct LoadTracker {
    loaded: Arc<AtomicBool>,
    asset: Arc<dyn Any + Send + Sync>,
}

impl LoadTracker {
    fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::SeqCst)
    }
}

#[derive(Default)]
pub struct AssetStorage {
    list: Option<AssetList>,
    assets: HashMap<TypeId, HashMap<String, LoadState>>,
}

impl AssetStorage {
    fn store<A>(&mut self, id: &str, asset: A)
    where
        A: Send + Sync + 'static,
    {
        let type_id = TypeId::of::<A>();
        let fut = platter::load_file(id.to_string()).map_err(|e| e.to_string());
        let state = LoadState::new(asset, Box::new(fut));

        // In case that exists a list append the state to it
        if let Some(loader) = &mut self.list {
            loader.insert(type_id, id, state.tracker());
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

    fn try_load(&mut self) {
        self.assets.retain(|_, list| {
            list.retain(|_, state| try_load(state));
            list.len() != 0
        });
    }
}

fn try_load(state: &mut LoadState) -> bool {
    // match state.fut.as_ref().poll() {
    //     _ => true
    // }

    true
}

/// Assets and loaders can be set and get from this struct
pub struct AssetManager {
    loaders: HashMap<TypeId, Arc<AssetLoader>>,
    extensions: HashMap<String, TypeId>,
    storage: AssetStorage,
}

impl AssetManager {
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

    fn tick(&mut self) {
        self.storage.try_load();
    }

    /// Starts loading a list of [Asset]s and return an [AssetList] to get them and check the progress
    pub fn load(&mut self, paths: &[&str]) -> Result<AssetList, String> {
        self.storage.list = Some(Default::default());

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

        let asset_list = self
            .storage
            .list
            .take()
            .ok_or("AssetList cannot be extracted from AssetManager".to_string())?;

        Ok(asset_list)
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

///
#[derive(Default, Clone)]
pub struct AssetList {
    count: usize,
    assets: HashMap<TypeId, HashMap<String, LoadTracker>>,
}

impl AssetList {
    fn insert(&mut self, type_id: TypeId, id: &str, tracker: LoadTracker) {
        let mut list = self.assets.entry(type_id).or_insert(HashMap::new());
        list.insert(id.to_string(), tracker);
        self.count += 1;
    }

    /// Returns true if all the assets were loaded
    pub fn is_loaded(&self) -> bool {
        let still_loading = self
            .assets
            .values()
            .find(|list| list.values().find(|tracker| !tracker.is_loaded()).is_some())
            .is_some();

        !still_loading
    }

    /// Returns the total count of assets
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns a value between 0.0 and 1.0 meaning 0.0 nothing has been loaded and 1.0 everything is loaded
    pub fn progress(&self) -> f32 {
        let loaded = self.assets.values().fold(0, |acc, list| {
            let loaded = list.values().fold(
                0,
                |acc, tracker| if tracker.is_loaded() { acc + 1 } else { acc },
            );

            acc + loaded
        });

        (loaded as f32 / self.count as f32)
    }

    /// Returns the [Asset]
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
            .map(|tracker| Asset {
                id: id.to_string(),
                loaded: tracker.loaded.clone(),
                res: tracker.asset.clone().downcast::<RwLock<A>>().unwrap(),
            })
    }

    /// Returns and remove from this list the [Asset]
    pub fn remove<A>(&mut self, id: &str) -> Result<Asset<A>, String>
    where
        A: Send + Sync + 'static,
    {
        let mut storage = match self.assets.get_mut(&TypeId::of::<A>()) {
            Some(map) => map,
            _ => return Err("Invalid asset type".to_string()),
        };

        let asset = storage
            .remove(id)
            .ok_or("Invalid asset id".to_string())
            .map(|tracker| Asset {
                id: id.to_string(),
                loaded: tracker.loaded.clone(),
                res: tracker.asset.clone().downcast::<RwLock<A>>().unwrap(),
            })?;

        self.count -= 1;
        Ok(asset)
    }

    /// Returns true if this list doesn't contains any asset
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
