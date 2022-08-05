use super::asset::Asset;
use super::utils::{AssetLoadTracker, DoneSignal, LoadWrapper};
use futures::prelude::*;
use hashbrown::HashMap;
use std::any::TypeId;

#[cfg(all(target_arch = "wasm32", feature = "drop_files"))]
use crate::DroppedFile;

/// Store the assets while they are loading
#[derive(Default)]
pub struct AssetStorage {
    to_load: HashMap<String, LoadWrapper>,
    pub(crate) tracker: AssetLoadTracker,
}

impl AssetStorage {
    pub(crate) fn register(&mut self, id: &str, type_id: TypeId) -> DoneSignal {
        #[allow(clippy::unnecessary_to_owned)]
        let fut = Box::pin(platter2::load_file(id.to_string()).map_err(|e| e.to_string()));
        let state = LoadWrapper::new(id, fut, type_id);
        let loaded = state.loaded.clone();
        log::info!("to load -> {} {:?}", id, state.type_id);
        self.to_load.insert(id.to_string(), state);
        loaded
    }

    #[cfg(all(target_arch = "wasm32", feature = "drop_files"))]
    pub(crate) fn register_wasm_dropped_file(
        &mut self,
        id: &str,
        file: &DroppedFile,
        type_id: TypeId,
    ) -> Result<DoneSignal, String> {
        let f = file
            .file
            .as_ref()
            .ok_or_else(|| "File not available".to_string())?;
        let fut = Box::pin(
            wasm_bindgen_futures::JsFuture::from(f.array_buffer()).map(|res| match res {
                Ok(buffer) => Ok(js_sys::Uint8Array::new(&buffer).to_vec()),
                Err(e) => Err(format!("{:?}", e)),
            }),
        );

        let state = LoadWrapper::new(id, fut, type_id);
        let loaded = state.loaded.clone();
        log::info!("to load -> {} {:?}", id, state.type_id);
        self.to_load.insert(id.to_string(), state);
        Ok(loaded)
    }

    /// Parse an asset with the loaded one
    pub fn parse<A>(&mut self, id: &str, asset: A) -> Result<(), String>
    where
        A: Send + Sync + 'static,
    {
        self.get::<A>(id, false)
            .map(|mut stored_asset| {
                *stored_asset.inner.write() = Some(asset);
                stored_asset.loaded.done();
            })
            .map_err(|e| {
                log::error!("{}", e);
                e
            })
    }

    pub(crate) fn get<A>(&self, id: &str, claim: bool) -> Result<Asset<A>, String>
    where
        A: Send + Sync + 'static,
    {
        match self.to_load.get(id) {
            Some(state) => {
                if TypeId::of::<A>() == state.type_id {
                    let loaded = state.loaded.clone();
                    let asset = if claim {
                        self.tracker.claim_asset(id, loaded.clone())
                    } else {
                        self.tracker.get_asset(id, loaded.clone())
                    };

                    asset.map(|inner| Asset {
                        id: id.to_string(),
                        loaded,
                        inner,
                    })
                } else {
                    Err("Invalid asset type".to_string())
                }
            }
            None => Err("Invalid asset id".to_string()),
        }
    }

    #[inline]
    pub(crate) fn try_load(&mut self) -> Option<Vec<(String, Vec<u8>)>> {
        if self.to_load.is_empty() {
            return None;
        }

        let loaded = self
            .to_load
            .iter_mut()
            .filter_map(|(id, state)| state.try_load().map(|data| (id.clone(), data)))
            .collect::<Vec<_>>();

        Some(loaded)
    }

    #[inline]
    pub(crate) fn clean_asset(&mut self, id: &str) -> Result<(), String> {
        let tracker = self
            .to_load
            .remove(id)
            .ok_or_else(|| format!("Asset '{}' not found.", id))?;

        if !tracker.is_loaded() {
            return Err(format!(
                "The loader of '{}' should call 'storage.parse({}, asset)' before it ends.",
                id, id
            ));
        }

        Ok(())
    }

    #[inline]
    pub(crate) fn clean_ready_assets(&mut self) {
        self.tracker.clean();
    }
}
