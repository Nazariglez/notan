use crate::config::WindowConfig;
use crate::{App, EventIterator};
use downcast_rs::{impl_downcast, Downcast};
use futures::prelude::*;
use futures::Future;
use notan_graphics::DeviceBackend;

#[cfg(feature = "audio")]
use notan_audio::AudioBackend;

#[cfg(feature = "audio")]
use std::cell::RefCell;

#[cfg(feature = "audio")]
use std::rc::Rc;

/// Closure returned from the backend's initialize method
pub type InitializeFn<S, R> = dyn FnOnce(App, S, R) -> Result<(), String>;

/// Closure used to load files
pub type LoadFileFn = Box<dyn Fn(String) -> Box<dyn Future<Output = Result<Vec<u8>, String>>>>;

/// Represents the backend implementation
pub trait Backend: Downcast {
    /// Returns the window implementation
    fn window(&mut self) -> &mut dyn WindowBackend;

    /// Returns an iterator that contains the backend events
    fn events_iter(&mut self) -> EventIterator;

    /// Closes the application
    fn exit(&mut self);

    /// Return the system timestamp
    fn system_timestamp(&self) -> u64;

    /// Open a link on a browser if the platform supports it
    fn open_link(&self, url: &str, new_tab: bool);
}

impl_downcast!(Backend);

/// Indicate to the backend if the frame was skipped or if it ended
pub enum FrameState {
    End,
    Skip,
}

/// Backend initialization run
pub trait BackendSystem: Backend {
    /// Returns a closure where the backend is initialized and the application loops is managed
    fn initialize<S, R>(&mut self, window: WindowConfig) -> Result<Box<InitializeFn<S, R>>, String>
    where
        Self: Backend,
        S: 'static,
        R: FnMut(&mut App, &mut S) -> Result<FrameState, String> + 'static;

    /// Returns a function that load files
    fn get_file_loader(&self) -> LoadFileFn {
        Box::new(|path| Box::new(platter2::load_file(path).map_err(|e| e.to_string())))
    }

    /// Returns the graphics backend implementation
    fn get_graphics_backend(&self) -> Box<dyn DeviceBackend>;

    #[cfg(feature = "audio")]
    /// Return the audio backend implementation
    fn get_audio_backend(&self) -> Rc<RefCell<dyn AudioBackend>>;
}

/// Represent mouse cursor icon
/// They are same as egui::CursorIcon because this is mostly to give support to egui
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum CursorIcon {
    Default,
    None,
    ContextMenu,
    Help,
    PointingHand,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    AllScroll,
    ResizeHorizontal,
    ResizeNeSw,
    ResizeNwSe,
    ResizeVertical,
    ZoomIn,
    ZoomOut,
    ResizeEast,
    ResizeSouthEast,
    ResizeSouth,
    ResizeSouthWest,
    ResizeWest,
    ResizeNorthWest,
    ResizeNorth,
    ResizeNorthEast,
    ResizeColumn,
    ResizeRow,
}

/// Represents a window
pub trait WindowBackend {
    /// Sets the window's size
    fn set_size(&mut self, width: i32, height: i32);

    /// Returns the window's size
    fn size(&self) -> (i32, i32);

    /// Enable or disable the fullscreen mode
    fn set_fullscreen(&mut self, enabled: bool);

    /// Returns true if the window is in fullscreen mode
    fn is_fullscreen(&self) -> bool;

    /// Set window to be drawn above others or not
    fn set_always_on_top(&mut self, enabled: bool);

    /// Returns true if window is drawn above others
    fn is_always_on_top(&self) -> bool;

    /// Window's width
    fn width(&self) -> i32 {
        self.size().0
    }

    /// Window's height
    fn height(&self) -> i32 {
        self.size().1
    }

    /// Screen's DPI
    fn dpi(&self) -> f64;

    /// Enable or disable the lazy mode for the app's loop
    fn set_lazy_loop(&mut self, lazy: bool);

    /// Returns true if the lazy mode is enabled
    fn lazy_loop(&self) -> bool;

    /// Request next frame
    fn request_frame(&mut self);

    /// Sets the mouse cursor icon
    fn set_cursor(&mut self, cursor: CursorIcon);

    /// Returns the current cursor icon
    fn cursor(&self) -> CursorIcon;

    /// Confine the mouse on the app
    fn set_capture_cursor(&mut self, capture: bool);

    /// Returns if the mouse is confined in the app
    fn capture_cursor(&self) -> bool;

    /// Hide or show the window
    fn set_visible(&mut self, visible: bool);

    /// Returns if the window is visible
    fn visible(&self) -> bool;
}
