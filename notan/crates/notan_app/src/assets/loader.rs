use super::storage::AssetStorage;
use crate::app::App;
use downcast_rs::{impl_downcast, Downcast};

/// Represents a custom asset loader
/// It's used to define how an asset is going to be created from a buffer of bytes
pub trait AssetLoader
where
    Self: Send + Sync + Downcast,
{
    /// Sets an asset's defaults version to be used while the file is loading
    fn set_default(&self, id: &str, storage: &mut AssetStorage);

    /// Once the file is loaded this is used to create a new asset from them and update it on the storage
    fn load(&self, id: &str, data: Vec<u8>, app: &mut App, storage: &mut AssetStorage);

    /// Returns the file extensions that will use this loader
    fn extensions(&self) -> &[&str];
}

impl_downcast!(AssetLoader);
