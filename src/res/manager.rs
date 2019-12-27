use super::loader::load_file;
use nae_core::resources::*;
//use super::resource::*;
use backend::System;
use futures::{Async, Future};
use nae_core::BaseSystem;

//https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4da80a3618ce54c5b4003d3eb424ae83
//https://github.com/Rufflewind/_urandom/blob/da71b8b23bc90407c371d651a09d3128654c76d7/rust/dyn_trait_hierarchy.rs
//http://idubrov.name/rust/2018/06/16/dynamic-casting-traits.html
//https://crates.io/crates/mopa
//https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=a13a0ddcc11fe83d8adb967d75187332

pub trait ResourceParser {
    fn parse_res(&mut self, app: &mut System, data: Vec<u8>) -> Result<(), String>;
    fn already_loaded(&mut self) -> bool;
}

impl ResourceParser for backend::Texture {
    fn parse_res(&mut self, sys: &mut System, data: Vec<u8>) -> Result<(), String> {
        self.parse(sys, data)
    }

    fn already_loaded(&mut self) -> bool {
        self.is_loaded()
    }
}

impl ResourceParser for backend::Font {
    fn parse_res(&mut self, sys: &mut System, data: Vec<u8>) -> Result<(), String> {
        self.parse(sys, data)
    }

    fn already_loaded(&mut self) -> bool {
        self.is_loaded()
    }
}

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
        T: ResourceParser + Resource + ResourceConstructor + Clone + 'a,
    {
        let fut = load_file(file);
        let asset = T::new(file);
        self.to_load.push((Box::new(asset.clone()), Box::new(fut)));
        Ok(asset)
    }

    pub fn add_from_memory<T>(&mut self, data: &[u8]) -> Result<T, String>
    where
        T: ResourceParser + Resource + ResourceConstructor + Clone + 'a,
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
