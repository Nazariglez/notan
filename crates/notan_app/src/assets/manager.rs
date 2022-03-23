use super::asset::Asset;
use super::list::AssetList;
use super::loader::*;
use super::storage::AssetStorage;
use super::utils::DoneSignal;

use hashbrown::HashMap;
use std::any::TypeId;
use std::path::Path;
use std::rc::Rc;

#[cfg(feature = "drop_files")]
use crate::DroppedFile;

pub struct Assets {
    storage: AssetStorage,
    pub(crate) loaders: HashMap<String, LoaderCallback>,
    byte_loader: LoaderCallback,
}

impl Assets {
    pub(crate) fn new() -> Self {
        let bytes_id = TypeId::of::<Vec<u8>>();
        let byte_loader = LoaderCallback::Basic(
            Some(bytes_id),
            Rc::new(|storage, id, bytes| storage.parse::<Vec<u8>>(id, bytes)),
        );

        Self {
            loaders: HashMap::new(),
            storage: AssetStorage::default(),
            byte_loader,
        }
    }

    pub(crate) fn tick<S>(&mut self, mut params: LoaderParams<S>) -> Result<(), String> {
        if let Some(mut to_update) = self.storage.try_load() {
            while let Some((id, data)) = to_update.pop() {
                let ext = Path::new(&id)
                    .extension()
                    .map(|ext| ext.to_str().unwrap())
                    .unwrap_or("");

                let loader = match self.loaders.get(ext) {
                    Some(loader) => loader,
                    None => {
                        log::warn!(
                            "Not found a loader for '{}', loading as bytes (Vec<u8>)",
                            id
                        );
                        &self.byte_loader
                    }
                };

                loader.exec(&id, data, &mut self.storage, &mut params)?;
                self.storage.clean_asset(&id)?;
            }

            self.storage.clean_ready_assets();
        }

        Ok(())
    }

    pub fn add_loader(&mut self, loader: AssetLoader) {
        if let Err(e) = loader.apply(self) {
            log::error!("{}", e);
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
                log::warn!(
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

    #[cfg(all(target_arch = "wasm32", feature = "drop_files"))]
    fn load_wasm_dropped_file(&mut self, file: &DroppedFile) -> Result<DoneSignal, String> {
        let id = file.name.clone();
        let ext = Path::new(&id)
            .extension()
            .map(|ext| ext.to_str().unwrap())
            .unwrap_or("");

        let loader = match self.loaders.get(ext) {
            Some(loader) => loader,
            None => {
                log::warn!(
                    "Not found a loader for '{}', loading as bytes (Vec<u8>)",
                    id
                );
                &self.byte_loader
            }
        };

        Ok(match loader.type_id() {
            Some(type_id) => self
                .storage
                .register_wasm_dropped_file(&id, file, type_id)?,
            None => return Err("Loader without output type id".to_string()),
        })
    }

    pub fn load_asset<A>(&mut self, id: &str) -> Result<Asset<A>, String>
    where
        A: Send + Sync + 'static,
    {
        let _ = self.load(id)?;
        self.storage.get(id, true)
    }

    #[cfg(all(target_arch = "wasm32", feature = "drop_files"))]
    fn load_wasm_dropped_file_asset<A>(&mut self, file: &DroppedFile) -> Result<Asset<A>, String>
    where
        A: Send + Sync + 'static,
    {
        let _ = self.load_wasm_dropped_file(file)?;
        self.storage.get(&file.name, true)
    }

    pub fn load_list(&mut self, paths: &[&str]) -> Result<AssetList, String> {
        let mut list = AssetList::new(self.storage.tracker.clone());
        for id in paths {
            let loaded = self.load(id)?;
            list.insert(id, loaded);
        }
        Ok(list)
    }

    #[cfg(feature = "drop_files")]
    pub fn load_dropped_file<A>(&mut self, file: &DroppedFile) -> Result<Asset<A>, String>
    where
        A: Send + Sync + 'static,
    {
        if let Some(path) = &file.path {
            if let Some(id) = path.to_str() {
                return self.load_asset(id);
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            return self.load_wasm_dropped_file_asset(file);
        }

        #[cfg(not(target_arch = "wasm32"))]
        Err(format!("Can't load the dropped file {}", file.name))
    }
}
