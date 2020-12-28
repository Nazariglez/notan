use super::waker::*;
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

pub(crate) struct LoadWrapper {
    fut: LocalBoxFuture<'static, Result<Vec<u8>, String>>,
    pub loaded: DoneSignal,
    pub type_id: TypeId,
}

impl LoadWrapper {
    pub fn new(fut: LocalBoxFuture<'static, Result<Vec<u8>, String>>, type_id: TypeId) -> Self {
        Self {
            fut,
            loaded: DoneSignal::new(),
            type_id,
        }
    }

    pub fn try_load(&mut self) -> Option<Vec<u8>> {
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

    #[inline(always)]
    pub fn is_loaded(&self) -> bool {
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

impl ClaimTracker {
    fn is_ready(&self) -> bool {
        self.tracker.is_loaded() && self.claim
    }
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
        loaded: DoneSignal,
    ) -> Result<Arc<RwLock<Option<A>>>, String>
    where
        A: Send + Sync + 'static,
    {
        self.insert_if_necessary::<A>(id, loaded);
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

    #[inline]
    pub fn clean(&mut self) {
        self.assets.write().retain(|_, tracker| !tracker.is_ready());
    }
}
