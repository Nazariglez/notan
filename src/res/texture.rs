use std::rc::Rc;
use std::cell::RefCell;
use futures::future::Future;
use super::resource::*;
use super::loader::load_file;

#[derive(Debug, Clone)]
pub struct TextureData {
    pub tex: glow::WebTextureKey,
    pub width: i32,
    pub height: i32,
    pub raw: Vec<u8>,
}


#[derive(Clone)]
pub struct Texture {
    pub inner: Rc<RefCell<Option<Vec<u8>>>>,
    pub(crate) fut: Rc<RefCell<Box<Future<Item = Vec<u8>, Error = String>>>>,
    pub(crate) tex: Option<TextureData>,
}

impl ResourceConstructor for Texture {
    fn new(file: &str) -> Self {
        Self {
            inner: Rc::new(RefCell::new(None)),
            fut: Rc::new(RefCell::new(Box::new(load_file(file)))),
            tex: None,
        }
    }
}

impl Resource for Texture {
    fn future(&mut self) -> Rc<RefCell<Future<Item = Vec<u8>, Error = String>>> {
        self.fut.clone()
    }

    fn set_asset(&mut self, asset: Vec<u8>) {
        *self.inner.borrow_mut() = Some(asset);
    }

    fn is_loaded(&self) -> bool {
        self.inner.borrow().is_some()
    }
}
