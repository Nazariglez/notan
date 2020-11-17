use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Read-Only representation of an asset loaded from a file
#[derive(Clone, Debug)]
pub struct Asset<A>
where
    A: Send + Sync,
{
    pub(crate) id: String,
    pub(crate) loaded: Arc<AtomicBool>,
    pub(crate) res: Arc<RwLock<A>>,
}

impl<A> Asset<A>
where
    A: Send + Sync + Default,
{
    /// Returns the id of this asset, used to loaded
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns a read-only access to the asset
    pub fn lock(&self) -> MappedRwLockReadGuard<'_, A> {
        RwLockReadGuard::map(self.res.read(), |unlocked| unlocked)
    }

    /// Returns true if the asset is already loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::SeqCst)
    }
}

impl<A> PartialEq for Asset<A>
where
    A: Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<A> Eq for Asset<A> where A: Send + Sync {}
