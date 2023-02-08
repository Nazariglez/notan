use std::path::PathBuf;

use crate::backend::Backend;
use crate::builder::{AppBuilder, BuildConfig};

/// Builder configuration for the window options
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WindowConfig {
    /// Window's title
    /// `Web: no-op`
    pub title: String,

    /// Window's width
    pub width: u32,

    /// Window's height
    pub height: u32,

    /// Start window in fullscreen mode
    /// `Web: no-op`
    pub fullscreen: bool,

    /// Minimum resizable window's size
    pub min_size: Option<(u32, u32)>,

    /// Maximum resizable window's size
    pub max_size: Option<(u32, u32)>,

    /// Start the window maximized
    /// `Web: The canvas will fill the size of the parent of the HtmlCanvasElement`
    pub maximized: bool,

    /// Allow to resize the window
    /// `Web: The canvas will resize along with the parent of the HtmlCanvasElement`
    pub resizable: bool,

    /// Enable V-Sync
    /// `Web: no-op`
    pub vsync: bool,

    /// Antialias nultisamples level
    /// `Web: WebGL will use this as antialias = false if the value is 0 or true otherwise`
    pub multisampling: u8,

    /// Enable High DPI viewport and drawing if the device pixel ratio is higher than 1
    /// This is `false` by default, enable it could consume more resources and require
    /// a custom way of drawing things. The advice is using it if you know what you're doing
    pub high_dpi: bool,

    /// Inner loop will run only after an input event
    pub lazy_loop: bool,

    /// Background as transparent
    pub transparent: bool,

    /// Window will be drawn above others
    pub always_on_top: bool,

    /// Enable decorations
    /// `Web: Does nothing`
    pub decorations: bool,

    /// Hide the windows
    pub visible: bool,

    // Whether mouse events will pass through the window, useful for overlays
    pub mouse_passthrough: bool,

    /// Use or create the canvas with this id. Only Web.
    pub canvas_id: String,

    /// Window icon file
    pub window_icon_path: Option<PathBuf>,

    /// Task bar icon file
    pub taskbar_icon_path: Option<PathBuf>,
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
            high_dpi: false,
            lazy_loop: false,
            transparent: false,
            always_on_top: false,
            decorations: true,
            visible: true,
            mouse_passthrough: false,
            canvas_id: String::from("notan_canvas"),
            window_icon_path: None,
            taskbar_icon_path: None,
        }
    }
}

impl WindowConfig {
    /// Create a new instance using default values
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the window's title
    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Inner loop will run only after an input event
    pub fn set_lazy_loop(mut self, lazy: bool) -> Self {
        self.lazy_loop = lazy;
        self
    }

    /// Sets the window's width and height
    pub fn set_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Enable fullscreen mode
    pub fn set_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    /// Sets the window's minimum size
    pub fn set_min_size(mut self, width: u32, height: u32) -> Self {
        self.min_size = Some((width, height));
        self
    }

    /// Sets the window's maximum size
    pub fn set_max_size(mut self, width: u32, height: u32) -> Self {
        self.max_size = Some((width, height));
        self
    }

    /// Starts the window maximized
    pub fn set_maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    /// Allow the window to be resizable
    pub fn set_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Enable vsync
    pub fn set_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    /// Enabled multisampling aliasing (opengl)
    pub fn set_multisampling(mut self, samples: u8) -> Self {
        self.multisampling = samples;
        self
    }

    /// Enable High DPI
    pub fn set_high_dpi(mut self, enabled: bool) -> Self {
        self.high_dpi = enabled;
        self
    }

    /// Set the background as transparent
    pub fn set_transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    /// Set the background as transparent
    pub fn set_always_on_top(mut self, always_on_top: bool) -> Self {
        self.always_on_top = always_on_top;
        self
    }

    /// Enable or disable decorations
    pub fn set_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    /// Hide or show the window
    pub fn set_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Mouse events pass through window
    pub fn set_mouse_passthrough(mut self, mouse_passthrough: bool) -> Self {
        self.mouse_passthrough = mouse_passthrough;
        self
    }

    /// Use or create the canvas with this id. Only Web.
    pub fn set_canvas_id(mut self, canvas_id: &str) -> Self {
        self.canvas_id = canvas_id.to_string();
        self
    }

    /// Window icon path
    pub fn set_window_icon(mut self, window_icon_path: Option<PathBuf>) -> Self {
        self.window_icon_path = window_icon_path;
        self
    }

    /// Task bar icon path
    pub fn set_taskbar_icon(mut self, taskbar_icon_path: Option<PathBuf>) -> Self {
        self.taskbar_icon_path = taskbar_icon_path;
        self
    }
}

impl<S, B> BuildConfig<S, B> for WindowConfig
where
    B: Backend,
{
    fn apply(&self, mut builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        builder.window = self.clone();
        builder
    }
}
