use super::loader::AssetLoader;
use super::storage::AssetStorage;
use crate::app::App;

#[derive(Default)]
pub struct BytesLoader;

impl AssetLoader for BytesLoader {
    fn set_default(&self, id: &str, storage: &mut AssetStorage) {
        storage.set_default::<Vec<u8>>(id, vec![]);
    }

    fn load(&self, id: &str, data: Vec<u8>, _app: &mut App, storage: &mut AssetStorage) {
        storage.update(&id, data);
    }

    fn extensions(&self) -> &[&str] {
        &["blob"]
    }
}
