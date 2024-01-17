use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WindowId(u64);

impl From<u64> for WindowId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<WindowId> for u64 {
    fn from(value: WindowId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub size: Option<(u32, u32)>,
    pub min_size: Option<(u32, u32)>,
    pub max_size: Option<(u32, u32)>,
    pub position: Option<(i32, i32)>,
    pub resizable: bool,
    pub title: String,
    pub fullscreen: bool,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
}

impl WindowConfig {
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.size = Some((width, height));
        self
    }

    pub fn with_min_size(mut self, width: u32, height: u32) -> Self {
        self.min_size = Some((width, height));
        self
    }

    pub fn with_max_size(mut self, width: u32, height: u32) -> Self {
        self.max_size = Some((width, height));
        self
    }

    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.position = Some((x, y));
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            size: Some((800, 600)),
            min_size: None,
            max_size: None,
            position: None,
            resizable: false,
            title: "Notan Window".to_string(),
            fullscreen: false,
            maximized: false,
            visible: true,
            transparent: false,
        }
    }
}

pub trait NotanApp<W: NotanWindow> {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    fn create(&mut self, attrs: WindowConfig) -> Result<WindowId, String>;
    fn close(&mut self, id: WindowId) -> bool;
    fn exit(&mut self);
}

pub trait NotanWindow: HasWindowHandle + HasDisplayHandle + std::marker::Sync {
    fn id(&self) -> WindowId;
    fn physical_size(&self) -> (u32, u32);
    fn size(&self) -> (u32, u32);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_size(&mut self, width: u32, height: u32);
    fn scale(&self) -> f64;
    fn position(&self) -> Result<(i32, i32), String>;
    fn set_position(&mut self, x: i32, y: i32);
    fn title(&self) -> &str;
    fn set_title(&mut self, title: &str);
    fn fullscreen(&self) -> bool;
    fn set_fullscreen(&mut self, fullscreen: bool);
    fn request_focus(&mut self);
    fn has_focus(&self) -> bool;
    fn set_cursor_icon(&mut self, cursor: CursorIcon);
    fn cursor(&self) -> CursorIcon;
    fn set_maximized(&mut self, maximized: bool);
    fn maximized(&self) -> bool;
    fn set_minimized(&mut self, minimized: bool);
    fn minimized(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
    fn visible(&self) -> bool;
    fn set_transparent(&mut self, transparent: bool);
    fn transparent(&self) -> bool;
    fn set_resizable(&mut self, resizable: bool);
    fn resizable(&self) -> bool;
    fn set_min_size(&mut self, width: u32, height: u32);
    fn min_size(&self) -> Option<(u32, u32)>;
    fn set_max_size(&mut self, width: u32, height: u32);
    fn max_size(&self) -> Option<(u32, u32)>;
    fn request_redraw(&mut self);
}

/// Window's event
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WindowEvent {
    pub id: WindowId,
    pub action: WindowAction,
}

/// Window's event type
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowAction {
    /// A new window was created
    Init,

    /// Window's position after it was moved
    Moved { x: i32, y: i32 },

    /// Window's size after it was resized
    Resized {
        width: u32,
        height: u32,
        scale_factor: f64,
    },

    /// The window was minimized
    Minimized,

    /// The window was maximized
    Maximized,

    /// The window did gain the focus
    FocusGained,

    /// The window did lost the focus
    FocusLost,

    /// The window was closed
    Close,
}

/// Represent mouse cursor icon
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
