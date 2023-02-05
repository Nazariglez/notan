use super::utils::DoneSignal;
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use std::sync::Arc;

/// Read-Only representation of an asset loaded from a file
#[derive(Debug)]
pub struct Asset<A>
where
    A: Send + Sync,
{
    pub(crate) id: String,
    pub(crate) loaded: DoneSignal,
    pub(crate) inner: Arc<RwLock<Option<A>>>,
}

impl<A> Asset<A>
where
    A: Send + Sync,
{
    /// Returns the id of this asset, used to loaded
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Give access to the inner asset if it's already loaded
    pub fn lock(&self) -> Option<MappedRwLockReadGuard<'_, A>> {
        let opt_asset = self.inner.read();
        if opt_asset.is_none() {
            return None;
        }

        Some(RwLockReadGuard::map(opt_asset, |asset| {
            asset.as_ref().unwrap()
        }))
    }

    /// Consume the asset and returns the inner asset if it's already loaded and exists just one reference to it
    pub fn try_unwrap(self) -> Result<A, String> {
        if !self.is_loaded() {
            return Err(format!(
                "Asset: '{}' cannot be unwrapped because is still loading...",
                self.id
            ));
        }

        let id = self.id.clone();
        Arc::try_unwrap(self.inner)
            .map_err(|_| {
                format!(
                    "Asset: '{id}' cannot be unwrapped because exists more than one reference to it.",
                )
            })
            .map(|asset_lock| asset_lock.into_inner().unwrap())
    }

    /// Returns true if the asset is already loaded
    #[inline]
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
            inner: Arc::new(RwLock::new(data)),
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

impl<A> Clone for Asset<A>
where
    A: Send + Sync,
{
    fn clone(&self) -> Self {
        Asset {
            id: self.id.clone(),
            loaded: self.loaded.clone(),
            inner: self.inner.clone(),
        }
    }
}
