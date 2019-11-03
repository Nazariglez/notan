use std::rc::Rc;
use std::cell::RefCell;
use futures::future::Future;
use futures::Async;

pub trait ResourceConstructor {
    fn new(path: &str) -> Self;
}

/// Represent an Asset
pub trait Resource {
    /// Get the future stored on charge of load the file
    fn future(&mut self) -> Rc<RefCell<Future<Item = Vec<u8>, Error = String>>>;

    /// Dispatched when the asset buffer is loaded, used to process and store the data ready to be consumed
    fn set_asset(&mut self, asset: Vec<u8>);

    /// Check if the asset is loaded on memory
    fn is_loaded(&self) -> bool;

    /// Execute the future in charge of loading the file
    fn try_load(&mut self) -> Result<(), String> {
        if self.is_loaded() {
            return Ok(());
        }

        self.future().borrow_mut().poll().map(|s| {
            Ok(match s {
                Async::Ready(buff) => {
                    self.set_asset(buff);
                }
                _ => {}
            })
        })?
    }
}

