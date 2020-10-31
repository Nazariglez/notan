use crate::backend::Backend;
use crate::builder::{AppBuilder, BuildConfig};

/// Builder configuration for the window options
pub struct WindowConfig {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
    pub min_size: Option<(i32, i32)>,
    pub max_size: Option<(i32, i32)>,
    pub maximized: bool,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("Notan App"),
            width: 800,
            height: 600,
            fullscreen: true,
            min_size: None,
            max_size: None,
            maximized: false,
            resizable: false,
        }
    }
}

impl WindowConfig {
    /// Create a new instance using default values
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the window's title
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the window's width and height
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Enable fullscreen mode
    pub fn fullscreen(mut self) -> Self {
        self.fullscreen = true;
        self
    }

    /// Sets the window's minimum size
    pub fn min_size(mut self, width: i32, height: i32) -> Self {
        self.min_size = Some((width, height));
        self
    }

    /// Sets the window's maximum size
    pub fn max_size(mut self, width: i32, height: i32) -> Self {
        self.max_size = Some((width, height));
        self
    }

    /// Starts the window maximized
    pub fn maximized(mut self) -> Self {
        self.maximized = true;
        self
    }

    /// Allow the window to be resizable
    pub fn resizable(mut self) -> Self {
        self.resizable = true;
        self
    }
}

impl<S, B> BuildConfig<S, B> for WindowConfig
where
    B: Backend,
{
    fn apply(self, mut builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        builder.window = self;
        builder
    }
}
