use super::ResourceParser;
use crate::app::App;
use crate::resource_parser;
use backend::{Draw, Graphics};
use nae_core::{BaseApp, BaseSystem, Resource};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

/// Represent raw data
#[derive(Clone)]
pub struct Blob {
    inner: Rc<RefCell<Vec<u8>>>,
}

impl Blob {
    //https://stackoverflow.com/questions/29401626/how-do-i-return-a-reference-to-something-inside-a-refcell-without-breaking-encap
    /// Return a reference to the inner data
    pub fn data(&self) -> Ref<Vec<u8>> {
        Ref::map(self.inner.borrow(), |data| data)
    }

    /// Return a mutable referece to the inner data
    pub fn data_mut(&mut self) -> RefMut<Vec<u8>> {
        RefMut::map(self.inner.borrow_mut(), |data| data)
    }

    /// Create a new blob from bytes
    pub fn from_bytes<T, S>(app: &mut T, data: &[u8]) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics>,
    {
        let blob = Blob {
            inner: Rc::new(RefCell::new(data.to_vec())),
        };

        Ok(blob)
    }

    /// Returns if the resource is already loaded
    pub fn is_loaded(&self) -> bool {
        self.inner.borrow().len() != 0
    }
}

impl Resource<App> for Blob {
    fn prepare(app: &mut App, file: &str) -> Result<Self, String> {
        Self::from_bytes(app, &[])
    }

    fn set_data(&mut self, app: &mut App, data: Vec<u8>) -> Result<(), String> {
        *self.inner.borrow_mut() = data;
        Ok(())
    }
}

resource_parser!(Blob, App);
