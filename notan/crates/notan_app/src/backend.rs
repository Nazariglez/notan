use crate::App;

/// Closure returned from the backend's initilize method
pub type InitializeFn<B, S, R> = Fn(App<B>, S, R) -> Result<(), String>;

/// Represents the backend implementation
pub trait Backend: Send + Sync {
    type Impl: Backend;

    /// Returns the backend forcing the implementation type.
    /// Useful for have access inside the runner callback to the private fields from itself.
    fn get_impl(&mut self) -> &mut <Self as Backend>::Impl;

    /// Returns a closure where the backend is initialized and the application loops is managed
    fn initialize<B, S, R>(&mut self) -> Result<Box<InitializeFn<B, S, R>>, String>
        where
            B: Backend<Impl = Self::Impl> + 'static,
            S: 'static,
            R: FnMut(&mut App<B>, &mut S) + 'static;

    /// Sets the window's size
    fn set_size(&mut self, width: i32, height: i32);

    /// Returns the window's size
    fn size(&self) -> (i32, i32);

    /// Enable or disable the fullscreen mode
    fn set_fullscreen(&mut self, enabled: bool);

    /// Returns true if the window is in fullscreen mode
    fn is_fullscreen(&self) -> bool;

    /// Closes the application
    fn exit(&mut self);
}
