use super::asset::Asset;
use super::utils::{AssetLoadTracker, DoneSignal};
use hashbrown::{HashMap, HashSet};
use parking_lot::RwLock;
use std::any::{Any, TypeId};

use std::sync::Arc;

pub struct AssetList {
    count: usize,
    load_tracker: HashMap<String, DoneSignal>,
    assets: HashMap<TypeId, HashMap<String, Arc<dyn Any + Send + Sync>>>,
    claimed: HashSet<String>,
    tracker: AssetLoadTracker,
}

impl AssetList {
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

    /// Returns if the list contains the asset
    pub fn contains(&self, id: &str) -> bool {
        self.load_tracker.contains_key(id)
    }

    /// Create an [Asset] clone and returns it
    pub fn get_clone<A>(&mut self, id: &str) -> Result<Asset<A>, String>
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
            let list = self.assets.entry(type_id).or_default();
            list.insert(id.to_string(), asset);
        }

        let list = match self.assets.get(&type_id) {
            Some(map) => map,
            _ => return Err("Invalid asset type".to_string()),
        };

        list.get(id)
            .ok_or_else(|| "Invalid asset id".to_string())
            .map(|asset| Asset {
                id: id.to_string(),
                loaded,
                inner: asset.clone().downcast::<RwLock<Option<A>>>().unwrap(),
            })
    }

    /// Remove and returns the [Asset] from the list
    pub fn take<A>(&mut self, id: &str) -> Result<Asset<A>, String>
    where
        A: Send + Sync + 'static,
    {
        let asset = self.get_clone::<A>(id)?;
        self.count -= 1;
        self.load_tracker.remove(id);
        self.claimed.remove(id);
        self.tracker.clean();
        if let Some(map) = self.assets.get_mut(&TypeId::of::<A>()) {
            map.remove(id);
        }
        Ok(asset)
    }
}
