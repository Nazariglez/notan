use std::collections::VecDeque;

#[cfg(feature = "drop_files")]
use std::path::PathBuf;

use crate::keyboard::KeyCode;
use crate::mouse::MouseButton;

#[derive(Debug, PartialEq, Clone)]
/// Application events usually received from the user
pub enum Event {
    /// When the app is about to close
    Exit,

    /// Represents the current window's position after it was moved
    WindowMove { x: i32, y: i32 },

    /// Represents the current window's size after it was resized
    WindowResize { width: i32, height: i32 },

    /// Represents a change on the screen aspect ration
    ScreenAspectChange { ratio: f64 },

    /// Represents the current's mouse position after it was moved
    MouseMove { x: i32, y: i32 },

    /// A mouse button is down on this position
    MouseDown { button: MouseButton, x: i32, y: i32 },

    /// A mouse button was released on this position
    MouseUp { button: MouseButton, x: i32, y: i32 },

    /// The mouse wheel was moved with this delta
    MouseWheel { delta_x: f32, delta_y: f32 },

    /// Mouse cursor has entered the window's app
    MouseEnter { x: i32, y: i32 },

    /// Mouse cursor has left the window's app
    MouseLeft { x: i32, y: i32 },

    /// Keyboard's key is down
    KeyDown { key: KeyCode },

    /// Keyboard's key was released
    KeyUp { key: KeyCode },

    /// User's touch the screen
    TouchStart { id: u64, x: f32, y: f32 },

    /// User0s move the touch
    TouchMove { id: u64, x: f32, y: f32 },

    /// Touch event ends
    TouchEnd { id: u64, x: f32, y: f32 },

    /// The System cancelled the touch
    TouchCancel { id: u64, x: f32, y: f32 },

    /// Unicode char pressed
    ReceivedCharacter(char),

    #[cfg(feature = "drop_files")]
    /// The user is dragging a file over the window
    DragEnter {
        path: Option<PathBuf>,
        name: Option<String>,
        mime: String,
    },

    #[cfg(feature = "drop_files")]
    /// The user stops dragging any file over the window
    DragLeft,

    #[cfg(feature = "drop_files")]
    /// A file was dropped into the window
    Drop(DroppedFile),

    #[cfg(feature = "clipboard")]
    /// Text copied to the clipboard
    Copy,

    #[cfg(feature = "clipboard")]
    /// Text cut to the clipboard
    Cut,

    #[cfg(feature = "clipboard")]
    /// Text pasted from the clipboard
    Paste(String),
}

#[cfg(feature = "drop_files")]
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct DroppedFile {
    pub path: Option<PathBuf>,
    pub name: String,
    pub mime: String,

    #[cfg(target_arch = "wasm32")]
    pub file: Option<web_sys::File>,
}

#[derive(Debug, Clone, Default)]
/// Event iterator queue
pub struct EventIterator(VecDeque<Event>);

impl EventIterator {
    pub fn new() -> Self {
        Default::default()
    }

    /// Remove and return the first element on the queue
    pub fn pop_front(&mut self) -> Option<Event> {
        self.0.pop_front()
    }

    /// Add an event at the end of the list
    pub fn push(&mut self, evt: Event) {
        self.0.push_back(evt);
    }

    /// Add an event at the beginning of the list
    pub fn push_front(&mut self, evt: Event) {
        self.0.push_front(evt);
    }

    /// Return the events and clear the list
    pub fn take_events(&mut self) -> EventIterator {
        EventIterator(std::mem::take(&mut self.0))
    }
}

impl Iterator for EventIterator {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}
