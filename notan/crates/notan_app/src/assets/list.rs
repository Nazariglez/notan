use super::asset::Asset;
use super::storage::LoadTracker;
use hashbrown::HashMap;
use parking_lot::RwLock;
use std::any::TypeId;

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
        let storage = match self.assets.get_mut(&TypeId::of::<A>()) {
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
