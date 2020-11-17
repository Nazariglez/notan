use super::asset::Asset;
use super::bytes::BytesLoader;
use super::list::AssetList;
use super::loader::AssetLoader;
use super::storage::AssetStorage;
use crate::app::App;
use hashbrown::HashMap;
use std::any::{Any, TypeId};
use std::path::Path;
use std::sync::Arc;

/// Assets and loaders can be set and get from this struct
#[derive(Default)]
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

        manager.add_loader::<BytesLoader>();

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

    #[inline(always)]
    /// Starts loading a file and returns an [Asset]
    pub fn load_asset<A>(&mut self, path: &str) -> Result<Asset<A>, String>
    where
        A: Send + Sync + 'static,
    {
        let _ = self.load_list(&[path])?;
        self.storage.get(path)
    }

    #[inline(always)]
    pub(crate) fn tick(&mut self, app: &mut App) {
        match self.storage.try_load() {
            Some(to_update) => self.update_assets_list(app, to_update),
            _ => {}
        }
    }

    #[inline(always)]
    fn update_assets_list(&mut self, app: &mut App, mut to_update: Vec<(String, Vec<u8>)>) {
        let to_clean = to_update
            .drain(..)
            .rev()
            .map(|(id, data)| {
                self.update_asset(app, &id, data);
                id
            })
            .collect::<Vec<_>>();

        self.storage.clean(&to_clean);
    }

    #[inline(always)]
    fn update_asset(&mut self, app: &mut App, id: &str, data: Vec<u8>) {
        let ext = Path::new(&id)
            .extension()
            .map(|ext| ext.to_str().unwrap())
            .unwrap_or("blob");

        let loader = match self.get_loader(ext) {
            Ok(loader) => loader,
            Err(err) => {
                notan_log::warn!("Asset: {} -> {} -> loading as Blob", id, err);
                self.get_loader("blob").unwrap()
            }
        }
        .clone();

        loader.load(&id, data, app, &mut self.storage);
    }

    /// Starts loading a list of [Asset]s and return an [AssetList] to get them and check the progress
    pub fn load_list(&mut self, paths: &[&str]) -> Result<AssetList, String> {
        self.storage.list = Some(Default::default());

        paths.iter().for_each(|id| {
            let ext = Path::new(id)
                .extension()
                .map(|ext| ext.to_str().unwrap())
                .unwrap_or("blob");

            let loader = match self.get_loader(ext) {
                Ok(loader) => loader,
                Err(err) => {
                    notan_log::warn!("Asset: {} -> {} -> loading as Blob", id, err);
                    self.get_loader("blob").unwrap()
                }
            }
            .clone();

            loader.set_default(id, &mut self.storage);
        });

        let asset_list = self
            .storage
            .list
            .take()
            .ok_or("AssetList cannot be extracted from AssetManager".to_string())?;

        Ok(asset_list)
    }

    #[inline(always)]
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
