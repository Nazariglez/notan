use std::rc::Rc;
use std::cell::RefCell;

struct InnerTexture {}

pub struct Texture {
    inner: Rc<RefCell<InnerTexture>>,
}
