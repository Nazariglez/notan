use super::loader::load_file;
use backend::System;
use futures::{Async, Future};
use nae_core::BaseSystem;
use nae_core::*;

pub trait ResourceParser {
    fn parse_res(&mut self, app: &mut System, data: Vec<u8>) -> Result<(), String>;
    fn already_loaded(&mut self) -> bool;
}

#[macro_export]
macro_rules! resource_parser {
    ($type:ty, $system:ty) => {
        impl ResourceParser for $type {
            fn parse_res(&mut self, sys: &mut $system, data: Vec<u8>) -> Result<(), String> {
                self.parse(sys, data)
            }

            fn already_loaded(&mut self) -> bool {
                self.is_loaded()
            }
        }
    };
}

resource_parser!(backend::Texture, System);
resource_parser!(backend::Font, System);

type ResourceLoader<'a> = (
    Box<dyn ResourceParser + 'a>,
    Box<dyn Future<Item = Vec<u8>, Error = String>>,
);

pub(crate) struct ResourceLoaderManager<'a> {
    to_load: Vec<ResourceLoader<'a>>,
}

impl<'a> ResourceLoaderManager<'a> {
    pub fn new() -> Self {
        Self { to_load: vec![] }
    }

    pub fn len(&self) -> usize {
        self.to_load.len()
    }

    pub fn add<T>(&mut self, file: &str) -> Result<T, String>
    where
        T: ResourceParser + Resource + Clone + 'a,
    {
        let fut = load_file(file);
        let asset = T::new(file);
        self.to_load.push((Box::new(asset.clone()), Box::new(fut)));
        Ok(asset)
    }

    pub fn add_from_memory<T>(&mut self, data: &[u8]) -> Result<T, String>
    where
        T: ResourceParser + Resource + Clone + 'a,
    {
        unimplemented!()
    }

    pub fn try_load(
        &mut self,
    ) -> Result<Option<Vec<(Vec<u8>, Box<dyn ResourceParser + 'a>)>>, String> {
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
