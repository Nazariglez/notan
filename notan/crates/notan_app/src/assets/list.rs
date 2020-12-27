use super::asset::{Asset, Asset2};
use super::storage::{AssetLoadTracker, DoneSignal, LoadTracker};
use hashbrown::{HashMap, HashSet};
use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct AssetList2 {
    count: usize,
    load_tracker: HashMap<String, DoneSignal>,
    assets: HashMap<TypeId, HashMap<String, Arc<dyn Any + Send + Sync>>>,
    claimed: HashSet<String>,
    tracker: AssetLoadTracker,
}

impl AssetList2 {
    pub(crate) fn new(tracker: AssetLoadTracker) -> Self {
        Self {
            count: 0,
            assets: Default::default(),
            load_tracker: Default::default(),
            tracker,
            claimed: Default::default(),
        }
    }

    pub(crate) fn insert(&mut self, id: &str, loader: DoneSignal) {
        self.load_tracker.insert(id.to_string(), loader);
        self.count += 1;
    }

    /// Returns true if all the assets were loaded
    pub fn is_loaded(&self) -> bool {
        let still_loading = self.load_tracker.values().any(|loaded| !loaded.is_done());

        !still_loading
    }

    /// Returns the total count of assets
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns true if this list doesn't contains any asset
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns a value between 0.0 and 1.0 meaning 0.0 nothing has been loaded and 1.0 everything is loaded
    pub fn progress(&self) -> f32 {
        if self.load_tracker.is_empty() {
            return 1.0;
        }

        let loaded =
            self.load_tracker.values().fold(
                0,
                |acc, loaded| {
                    if loaded.is_done() {
                        acc + 1
                    } else {
                        acc
                    }
                },
            );
        loaded as f32 / self.count as f32
    }

    /// Returns the [Asset]
    pub fn get<A>(&mut self, id: &str) -> Result<Asset2<A>, String>
    where
        A: Send + Sync + 'static,
    {
        let type_id = TypeId::of::<A>();
        let loaded = self
            .load_tracker
            .get(id)
            .ok_or_else(|| "Invalid asset id".to_string())?
            .clone();

        if !self.claimed.contains(id) {
            let asset = self.tracker.claim_asset::<A>(id, loaded.clone())?;
            let list = self.assets.entry(type_id).or_insert(HashMap::new());
            list.insert(id.to_string(), asset);
        }

        let list = match self.assets.get(&type_id) {
            Some(map) => map,
            _ => return Err("Invalid asset type".to_string()),
        };

        list.get(id)
            .ok_or_else(|| "Invalid asset id".to_string())
            .map(|asset| Asset2 {
                id: id.to_string(),
                loaded,
                res: asset.clone().downcast::<RwLock<Option<A>>>().unwrap(),
            })
    }
}

#[derive(Default, Clone)]
/// A list of loading assets
pub struct AssetList {
    count: usize,
    assets: HashMap<TypeId, HashMap<String, LoadTracker>>,
}

impl AssetList {
    pub(crate) fn insert(&mut self, type_id: TypeId, id: &str, tracker: LoadTracker) {
        let list = self.assets.entry(type_id).or_insert(HashMap::new());
        list.insert(id.to_string(), tracker);
        self.count += 1;
    }

    /// Returns true if all the assets were loaded
    pub fn is_loaded(&self) -> bool {
        let still_loading = self
            .assets
            .values()
            .any(|list| list.values().any(|tracker| !tracker.is_loaded()));

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

        loaded as f32 / self.count as f32
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
            .ok_or_else(|| "Invalid asset id".to_string())
            .map(|tracker| Asset {
                id: id.to_string(),
                loaded: tracker.loaded.0.clone(),
                res: tracker.asset.clone().downcast::<RwLock<A>>().unwrap(),
            })
    }

    /// Returns and remove from this list the [Asset]
    pub fn remove<A>(&mut self, id: &str) -> Result<Asset<A>, String>
    where
        A: Send + Sync + 'static,
    {
        let storage = match self.assets.get_mut(&TypeId::of::<A>()) {
            Some(map) => map,
            _ => return Err("Invalid asset type".to_string()),
        };

        let asset = storage
            .remove(id)
            .ok_or_else(|| "Invalid asset id".to_string())
            .map(|tracker| Asset {
                id: id.to_string(),
                loaded: tracker.loaded.0.clone(),
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
