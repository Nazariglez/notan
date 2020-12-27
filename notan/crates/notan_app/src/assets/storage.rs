use super::asset::Asset;
use super::list::AssetList;
use super::waker::DummyWaker;

use super::utils::{AssetLoadTracker, DoneSignal, LoadWrapper};
use futures::prelude::*;
use hashbrown::HashMap;
use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Store the assets while they are loading
#[derive(Default)]
pub struct AssetStorage {
    pub(crate) list: Option<AssetList>,
    to_load: HashMap<String, LoadWrapper>,
    pub(crate) tracker: AssetLoadTracker,
}

impl AssetStorage {
    pub(crate) fn register(&mut self, id: &str, type_id: TypeId) -> DoneSignal {
        let fut = Box::pin(platter::load_file(id.to_string()).map_err(|e| e.to_string()));
        let state = LoadWrapper::new(fut, type_id);
        let loaded = state.loaded.clone();
        notan_log::info!("to load -> {} {:?}", id, state.type_id);
        self.to_load.insert(id.to_string(), state);
        loaded
    }

    /// Parse an asset with the loaded one
    pub fn parse<A>(&mut self, id: &str, asset: A) -> Result<(), String>
    where
        A: Send + Sync + 'static,
    {
        self.get::<A>(id, false)
            .map(|mut stored_asset| {
                *stored_asset.res.write() = Some(asset);
                stored_asset.loaded.done();
            })
            .map_err(|e| {
                notan_log::error!("{}", e);
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
                        self.tracker.get_asset(id, loaded.0.clone())
                    };

                    asset.map(|res| Asset {
                        id: id.to_string(),
                        loaded,
                        res,
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
            .filter_map(|(id, state)| match state.try_load() {
                Some(data) => Some((id.clone(), data)),
                _ => None,
            })
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
}
