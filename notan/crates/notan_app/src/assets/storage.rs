use super::asset::{Asset, Asset2};
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

#[derive(Clone, Debug)]
pub(crate) struct DoneSignal(pub Arc<AtomicBool>);
impl DoneSignal {
    #[inline]
    pub fn new() -> Self {
        Self::from_bool(false)
    }

    #[inline]
    pub fn from_bool(value: bool) -> Self {
        Self(Arc::new(AtomicBool::new(value)))
    }

    #[inline]
    pub fn from_atomic(value: Arc<AtomicBool>) -> Self {
        Self(value)
    }

    #[inline]
    pub fn done(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }

    #[inline]
    pub fn is_done(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

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
            .ok_or_else(|| "Invalid asset id".to_string())
            .map(|state| Asset {
                id: id.to_string(),
                loaded: state.loaded.clone(),
                res: state.asset.clone().downcast::<RwLock<A>>().unwrap(),
            })
    }

    #[inline]
    pub(crate) fn try_load(&mut self) -> Option<Vec<(String, Vec<u8>)>> {
        if self.assets.is_empty() {
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

    #[inline(always)]
    pub(crate) fn clean(&mut self, ids: &[String]) {
        self.assets.retain(|_, list| {
            list.retain(|path_id, _| !ids.contains(&path_id));
            !list.is_empty()
        });
    }
}

#[inline]
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
            loaded: DoneSignal::from_atomic(self.loaded.clone()),
            asset: self.asset.clone(),
        }
    }
}

struct LoadWrapper2 {
    fut: LocalBoxFuture<'static, Result<Vec<u8>, String>>,
    loaded: DoneSignal,
    type_id: TypeId,
    // asset: Arc<dyn Any + Send + Sync>,
}

impl LoadWrapper2 {
    // fn new<A>(fut: LocalBoxFuture<'static, Result<Vec<u8>, String>>) -> Self
    //     where A: Send + Sync + 'static
    // {
    //     let asset: Option<A> = None;
    //     Self {
    //         fut,
    //         loaded: Arc::new(AtomicBool::new(false)),
    //         asset: Arc::new(RwLock::new(asset)),
    //     }
    // }

    fn new(fut: LocalBoxFuture<'static, Result<Vec<u8>, String>>, type_id: TypeId) -> Self {
        Self {
            fut,
            loaded: DoneSignal::new(),
            type_id,
        }
    }

    // fn tracker(&self) -> LoadTracker {
    //     LoadTracker {
    //         loaded: self.loaded.clone(),
    //         asset: self.asset.clone(),
    //     }
    // }

    fn try_load(&mut self) -> Option<Vec<u8>> {
        let waker = DummyWaker.into_task_waker();
        let mut ctx = Context::from_waker(&waker);
        match self.fut.as_mut().poll(&mut ctx) {
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

    fn is_loaded(&self) -> bool {
        self.loaded.is_done()
    }
}

#[derive(Clone)]
pub(crate) struct LoadTracker {
    pub loaded: DoneSignal,
    pub asset: Arc<dyn Any + Send + Sync>,
}

impl LoadTracker {
    pub fn is_loaded(&self) -> bool {
        self.loaded.is_done()
    }
}

struct ClaimTracker {
    tracker: LoadTracker,
    claim: bool,
}

#[derive(Default, Clone)]
pub(crate) struct AssetLoadTracker {
    assets: Arc<RwLock<HashMap<String, ClaimTracker>>>,
}

impl AssetLoadTracker {
    pub fn insert_if_necessary<A>(&self, id: &str, loaded: DoneSignal)
    where
        A: Send + Sync + 'static,
    {
        self.assets
            .write()
            .entry(id.to_string())
            .or_insert_with(|| {
                let asset: Arc<RwLock<Option<A>>> = Arc::new(RwLock::new(None));
                let tracker = LoadTracker { loaded, asset };

                ClaimTracker {
                    tracker,
                    claim: false,
                }
            });
    }

    pub fn get_asset<A>(
        &self,
        id: &str,
        loaded: Arc<AtomicBool>,
    ) -> Result<Arc<RwLock<Option<A>>>, String>
    where
        A: Send + Sync + 'static,
    {
        self.insert_if_necessary::<A>(id, DoneSignal::from_atomic(loaded));
        self.assets
            .read()
            .get(id)
            .unwrap()
            .tracker
            .asset
            .clone()
            .downcast::<RwLock<Option<A>>>()
            .map_err(|_| "Invalid asset type".to_string())
    }

    pub fn claim_asset<A>(
        &self,
        id: &str,
        loaded: DoneSignal,
    ) -> Result<Arc<RwLock<Option<A>>>, String>
    where
        A: Send + Sync + 'static,
    {
        self.insert_if_necessary::<A>(id, loaded);
        let mut assets = self.assets.write();
        let tracker = assets.get_mut(id).unwrap();
        tracker.claim = true;
        tracker
            .tracker
            .asset
            .clone()
            .downcast::<RwLock<Option<A>>>()
            .map_err(|_| "Invalid asset type".to_string())
    }
}

/// Store the assets while they are loading
#[derive(Default)]
pub struct AssetStorage2 {
    pub(crate) list: Option<AssetList>,
    to_load: HashMap<String, LoadWrapper2>,
    pub(crate) tracker: AssetLoadTracker,
}

impl AssetStorage2 {
    pub(crate) fn register(&mut self, id: &str, type_id: TypeId) -> DoneSignal {
        let fut = Box::pin(platter::load_file(id.to_string()).map_err(|e| e.to_string()));
        let state = LoadWrapper2::new(fut, type_id);
        let loaded = state.loaded.clone();
        notan_log::info!("to load -> {} {:?}", id, state.type_id);
        self.to_load.insert(id.to_string(), state);
        loaded
    }

    /// Parse an asset with the loaded one
    pub fn parse<A>(&mut self, id: &str, asset: A)
    where
        A: Send + Sync + 'static,
    {
        match self.get::<A>(id, false) {
            Ok(mut stored_asset) => {
                *stored_asset.res.write() = Some(asset);
                stored_asset.loaded.done();
            }
            Err(err) => notan_log::error!("{}", err),
        }
    }

    pub(crate) fn get<A>(&self, id: &str, claim: bool) -> Result<Asset2<A>, String>
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

                    asset.map(|res| Asset2 {
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
