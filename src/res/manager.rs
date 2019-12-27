use super::loader::load_file;
use nae_core::resources::*;
//use super::resource::*;
use futures::{Async, Future};
use nae_core::BaseSystem;
use backend::System;

type ResourceLoader = (
    Box<dyn BaseResource>,
    Box<dyn Future<Item = Vec<u8>, Error = String>>,
);

pub(crate) struct ResourceLoaderManager {
    to_load: Vec<ResourceLoader>,
}

impl ResourceLoaderManager {
    pub fn new() -> Self {
        Self { to_load: vec![] }
    }

    pub fn add<T>(&mut self, file: &str) -> Result<T, String>
    where
        T: BaseResource + Resource + ResourceConstructor + Clone,
    {
        let fut = load_file(file);
        let asset = T::new(file);
        self.to_load.push((Box::new(asset.clone()), Box::new(fut)));
        Ok(asset)
    }

    pub fn add_from_memory<T>(&mut self, data: &[u8]) -> Result<T, String>
    where
        T: Resource + ResourceConstructor + Clone,
    {
        unimplemented!()
    }

    pub fn try_load(&mut self) -> Result<Option<Vec<(Vec<u8>, Box<dyn BaseResource>)>>, String> {
        if self.to_load.len() == 0 {
            return Ok(None);
        }

        let mut loaded = vec![];
        let mut not_loaded = vec![];

        while let Some(mut asset_loader) = self.to_load.pop() {
            if let AssetState::Done(data) = try_load_asset(&mut asset_loader)? {
                loaded.push((data, asset_loader.0));
            } else {
                not_loaded.push(asset_loader);
            }
        }

        self.to_load = not_loaded;

        Ok(Some(loaded))
    }
}

#[derive(Eq, PartialEq)]
pub(crate) enum AssetState {
    OnProgress,
    AlreadyLoaded,
    Done(Vec<u8>),
}

fn try_load_asset(loader: &mut ResourceLoader) -> Result<AssetState, String> {
    let (_, ref mut future) = loader;
    return future.poll().map(|s| {
        if let Async::Ready(buff) = s {
            AssetState::Done(buff.to_vec())
        } else {
            AssetState::OnProgress
        }
    });
}
