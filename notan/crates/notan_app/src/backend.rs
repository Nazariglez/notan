use crate::config::WindowConfig;
use crate::graphics::GraphicsBackend;
use crate::{App, EventIterator};
use downcast_rs::{impl_downcast, Downcast};
use futures::prelude::*;
use futures::Future;

/// Closure returned from the backend's initialize method
pub type InitializeFn<S, R> = dyn Fn(App, S, R) -> Result<(), String>;

/// Closure used to load files
pub type LoadFileFn = Box<dyn Fn(String) -> Box<dyn Future<Output = Result<Vec<u8>, String>>>>;

/// Represents the backend implementation
pub trait Backend: Send + Sync + Downcast {
    /// Returns the window implementation
    fn window(&mut self) -> &mut dyn WindowBackend;

    /// Returns an iterator that contains the backend events
    fn events_iter(&mut self) -> EventIterator;

    /// Closes the application
    fn exit(&mut self);
}

impl_downcast!(Backend);

/// Backend initialization run
pub trait BackendSystem: Backend {
    /// Returns a closure where the backend is initialized and the application loops is managed
    fn initialize<S, R>(&mut self, window: WindowConfig) -> Result<Box<InitializeFn<S, R>>, String>
    where
        Self: Backend,
        S: 'static,
        R: FnMut(&mut App, &mut S) -> Result<(), String> + 'static;

    /// Returns a function that load files
    fn get_file_loader(&self) -> LoadFileFn {
        Box::new(|path| Box::new(platter::load_file(path).map_err(|e| e.to_string())))
    }

    /// Returns the graphics backend implementation
    fn get_graphics_backend(&self) -> Box<GraphicsBackend>;
}

/// Represents a window
pub trait WindowBackend: Send + Sync {
    /// Sets the window's size
    fn set_size(&mut self, width: i32, height: i32);

    /// Returns the window's size
    fn size(&self) -> (i32, i32);

    /// Enable or disable the fullscreen mode
    fn set_fullscreen(&mut self, enabled: bool);

    /// Returns true if the window is in fullscreen mode
    fn is_fullscreen(&self) -> bool;

    /// Window's width
    fn width(&self) -> i32 {
        self.size().0
    }

    /// Window's height
    fn height(&self) -> i32 {
        self.size().1
    }
}
