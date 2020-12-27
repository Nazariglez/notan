use super::utils::DoneSignal;
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use std::sync::Arc;

/// Read-Only representation of an asset loaded from a file
#[derive(Clone, Debug)]
pub struct Asset<A>
where
    A: Send + Sync,
{
    pub(crate) id: String,
    pub(crate) loaded: DoneSignal,
    pub(crate) res: Arc<RwLock<Option<A>>>,
}

impl<A> Asset<A>
where
    A: Send + Sync,
{
    /// Returns the id of this asset, used to loaded
    pub fn id(&self) -> &str {
        &self.id
    }
    //
    // pub fn lock(&self) -> MappedRwLockReadGuard<'_, Option<A>>  {
    //     RwLockReadGuard::map(self.res.read(), |unlocked| unlocked)
    // }

    /// Returns true if the asset is already loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded.is_done()
    }

    /// Create a new asset from custom data
    pub fn from_data(id: &str, data: A) -> Asset<A> {
        Self::from_option(id, Some(data))
    }

    #[inline]
    pub(crate) fn from_option(id: &str, data: Option<A>) -> Asset<A> {
        Asset {
            id: id.to_string(),
            loaded: DoneSignal::from_bool(true),
            res: Arc::new(RwLock::new(data)),
        }
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
