use crate::app::App;
use crate::res::loader::load_file;
use futures::future::Future;
use futures::Async;
use std::cell::RefCell;
use std::rc::Rc;

/// Represent a resource
pub trait Resource {
    /// Dispatched when the resource is loaded on memory
    fn parse(&mut self, app: &mut App, data: Vec<u8>) -> Result<(), String>;

    /// Should return true if the resource is ready to use it
    fn is_loaded(&self) -> bool;
}

/// Represent a resource that can be created from the trait
pub trait ResourceConstructor {
    /// Create a new resource
    fn new(file: &str) -> Self;
}
