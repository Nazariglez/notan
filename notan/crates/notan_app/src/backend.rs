use crate::{App, EventIterator};
use downcast_rs::{impl_downcast, Downcast};

/// Closure returned from the backend's initialize method
pub type InitializeFn<S, R> = dyn Fn(App, S, R) -> Result<(), String>;

/// Represents the backend implementation
pub trait Backend: Send + Sync + Downcast {
    /// Returns the window implementation
    fn window(&mut self) -> &mut WindowBackend;

    /// Returns an iterator that contains the backend events
    fn events_iter(&mut self) -> EventIterator;

    /// Closes the application
    fn exit(&mut self);
}

impl_downcast!(Backend);

pub trait BackendSystem: Backend {
    /// Returns a closure where the backend is initialized and the application loops is managed
    fn initialize<S, R>(&mut self) -> Result<Box<InitializeFn<S, R>>, String>
    where
        Self: Backend,
        S: 'static,
        R: FnMut(&mut App, &mut S) + 'static;
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
}
