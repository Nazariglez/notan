use futures::future::Future;
use futures::Async;
use std::cell::RefCell;
use std::rc::Rc;

/// Resource data in charge of load and store the buffer
pub struct ResourceData {
    data: Vec<u8>,
    future: Option<Box<dyn Future<Item = Vec<u8>, Error = String>>>,
}

impl ResourceData {
    /// Create a new resource data storing the future in charge of load the file
    pub fn new(future: Box<dyn Future<Item = Vec<u8>, Error = String>>) -> Self {
        Self {
            data: vec![],
            future: Some(future),
        }
    }

    /// Create a new ResourceData and wraps it inside a Rc<RefCell>>
    pub fn rc(future: Box<dyn Future<Item = Vec<u8>, Error = String>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(future)))
    }

    /// Executed once the file is loaded, this drops the future
    pub fn set_loaded(&mut self) {
        self.future = None;
    }

    /// Returns if the file is already loaded
    pub fn is_loaded(&self) -> bool {
        self.future.is_none()
    }

    /// Return the raw buffer
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/// This trait is used to construct the resources from the App
pub trait ResourceConstructor {
    /// Create a new Resource from a file
    fn new(path: &str) -> Self;
}

/// Represent an Asset
pub trait Resource {
    /// Dispatched when the resource buffer is loaded, used to process and store the data ready to be consumed
    fn on_load(&mut self) -> Result<(), String>;

    /// Check if the asset is loaded on memory
    fn is_loaded(&self) -> bool;

    /// Get the resource data on charge of load the file
    fn resource_data(&self) -> &Rc<RefCell<ResourceData>>;

    /// Execute the future in charge of loading the file
    fn try_load(&mut self) -> Result<(), String> {
        if self.resource_data().borrow().is_loaded() {
            return Ok(());
        }

        let rd = self.resource_data().clone();
        let mut rd_mut = rd.borrow_mut();

        match &mut rd_mut.future {
            Some(fut) => fut.poll().map(|s| {
                if let Async::Ready(buff) = s {
                    rd_mut.data = buff;
                    rd_mut.future = None;
                    drop(rd_mut);

                    self.on_load()?;
                }
                Ok(())
            })?,
            _ => {
                //Already loaded
                unreachable!();
            }
        }
    }
}
