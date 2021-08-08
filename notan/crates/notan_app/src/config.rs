use crate::backend::Backend;
use crate::builder::{AppBuilder, BuildConfig};

/// Builder configuration for the window options
pub struct WindowConfig {
    /// Window's title
    /// `Web: no-op`
    pub title: String,

    /// Window's width
    pub width: i32,

    /// Window's height
    pub height: i32,

    /// Start window in fullscreen mode
    /// `Web: no-op`
    pub fullscreen: bool,

    /// Minimum resizable window's size
    pub min_size: Option<(i32, i32)>,

    /// Maximum resizable window's size
    pub max_size: Option<(i32, i32)>,

    /// Start the window maximized
    /// `Web: no-op`
    pub maximized: bool,

    /// Allow to resize the window
    /// `Web: no-op`
    pub resizable: bool,

    /// Enable V-Sync
    /// `Web: no-op`
    pub vsync: bool,

    /// Antialias nultisamples level
    /// `Web: WebGL will use this as antialias = false if the value is 0 or true otherwise`
    pub multisampling: u16,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("Notan App"),
            width: 800,
            height: 600,
            fullscreen: false,
            min_size: None,
            max_size: None,
            maximized: false,
            resizable: false,
            vsync: false,
            multisampling: 0,
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

    /// Enable vsync
    pub fn vsync(mut self) -> Self {
        self.vsync = true;
        self
    }

    /// Enabled multisampling aliasing (opengl)
    pub fn multisampling(mut self, samples: u16) -> Self {
        self.multisampling = samples;
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
