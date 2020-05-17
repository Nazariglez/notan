use super::ResourceParser;
use crate::app::App;
use crate::resource_parser;
use backend::{Draw, Graphics};
use nae_core::{BaseApp, BaseSystem};
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

    pub fn is_loaded(&self) -> bool {
        self.inner.borrow().len() != 0
    }

    pub fn parse_data<T, S>(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem,
    {
        *self.inner.borrow_mut() = data;
        Ok(())
    }

    /// Create a new blob from bytes
    pub fn from_bytes<T, S>(app: &mut T, data: &[u8]) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics, Draw = Draw>,
    {
        let blob = Blob {
            inner: Rc::new(RefCell::new(data.to_vec())),
        };

        Ok(blob)
    }
}

resource_parser!(Blob, App);
