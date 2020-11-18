use super::asset::Asset;
use super::list::AssetList;
use super::waker::DummyWaker;
use futures::future::LocalBoxFuture;
use futures::prelude::*;
use futures::task::{Context, Poll};
use hashbrown::HashMap;
use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Store the assets while they are loading
#[derive(Default)]
pub struct AssetStorage {
    pub(crate) list: Option<AssetList>,
    assets: HashMap<TypeId, HashMap<String, LoadWrapper>>,
}

impl AssetStorage {
    /// Sets a default version of an asset that could be used while the real one is loading
    pub fn set_default<A>(&mut self, id: &str, asset: A)
    where
        A: Send + Sync + 'static,
    {
        let type_id = TypeId::of::<A>();
        let fut = Box::pin(platter::load_file(id.to_string()).map_err(|e| e.to_string()));
        let state = LoadWrapper::new(asset, fut);

        // In case that exists a list append the state to it
        if let Some(loader) = &mut self.list {
            loader.insert(type_id, id, state.tracker());
        }

        let list = self.assets.entry(type_id).or_insert(HashMap::new());
        list.insert(id.to_string(), state);
    }

    /// Update a default asset with the loaded one
    pub fn update<A>(&mut self, id: &str, asset: A)
    where
        A: Send + Sync + 'static,
    {
        match self.get::<A>(id) {
            Ok(stored_asset) => {
                *stored_asset.res.write() = asset;
                stored_asset.loaded.store(true, Ordering::SeqCst);
            }
            Err(err) => notan_log::error!("{}", err),
        }
    }

    pub(crate) fn get<A>(&self, id: &str) -> Result<Asset<A>, String>
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

    pub(crate) fn try_load(&mut self) -> Option<Vec<(String, Vec<u8>)>> {
        if self.assets.len() == 0 {
            return None;
        }

        let loaded = self
            .assets
            .iter_mut()
            .flat_map(|(_, list)| {
                list.iter_mut()
                    .filter_map(|(id, state)| match try_load(state) {
                        Some(data) => Some((id.clone(), data)),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Some(loaded)
    }

    pub(crate) fn clean(&mut self, ids: &[String]) {
        self.assets.retain(|_, list| {
            list.retain(|path_id, _| !ids.contains(&path_id));
            list.len() != 0
        });
    }
}

fn try_load(state: &mut LoadWrapper) -> Option<Vec<u8>> {
    let waker = DummyWaker.into_task_waker();
    let mut ctx = Context::from_waker(&waker);
    match state.fut.as_mut().poll(&mut ctx) {
        Poll::Ready(r_buff) => match r_buff {
            Ok(buff) => Some(buff),
            Err(err) => {
                notan_log::error!("{}", err);
                None
            }
        },
        _ => None,
    }
}

struct LoadWrapper {
    fut: LocalBoxFuture<'static, Result<Vec<u8>, String>>,
    loaded: Arc<AtomicBool>,
    asset: Arc<dyn Any + Send + Sync>,
}

impl LoadWrapper {
    fn new<A>(asset: A, fut: LocalBoxFuture<'static, Result<Vec<u8>, String>>) -> Self
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
pub(crate) struct LoadTracker {
    pub loaded: Arc<AtomicBool>,
    pub asset: Arc<dyn Any + Send + Sync>,
}

impl LoadTracker {
    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::SeqCst)
    }
}
