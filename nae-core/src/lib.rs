pub mod graphics;
pub mod log;
pub mod math;
pub mod resources;
pub mod window;

pub use graphics::*;
pub use resources::*;

pub use rand;
pub use rand_pcg;
use std::collections::VecDeque;

pub struct BuilderOpts {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
}

impl Default for BuilderOpts {
    fn default() -> Self {
        Self {
            title: String::from("Nae App"),
            width: 800,
            height: 600,
            fullscreen: false,
        }
    }
}

pub trait BaseApp {
    type System: BaseSystem;
    fn system(&mut self) -> &mut Self::System;
}

pub trait BaseSystem {
    type Kind: BaseSystem;
    type Context2d: BaseContext2d;

    fn new(opts: BuilderOpts) -> Result<Self::Kind, String>;
    fn ctx2(&mut self) -> &mut Self::Context2d;
    fn events(&mut self) -> &mut EventIterator;
}

#[cfg(target_arch = "wasm32")]
pub fn date_now() -> u64 {
    js_sys::Date::now() as u64
}

#[derive(Debug, Clone)]
pub struct EventIterator(VecDeque<Event>);

impl EventIterator {
    pub fn new() -> Self {
        EventIterator(VecDeque::new())
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.0.pop_front()
    }

    pub fn push(&mut self, evt: Event) {
        self.0.push_back(evt);
    }

    pub fn take_events(&mut self) -> EventIterator {
        EventIterator(std::mem::replace(&mut self.0, VecDeque::new()))
    }
}

impl Iterator for EventIterator {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn date_now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// Input events made by the user
pub enum Event {
    /// Dispatched when the window is about to close
    /// `web` target will ignore this event
    Quit,

    /// Represent the current window's position after it was moved
    /// `web` target will ignore this event
    WindowMove { x: i32, y: i32 },

    /// Represents the current window's size after it was resized
    /// `web` target will dispatch this event when the browser window is resized, not the canvas itself.
    WindowResize { width: i32, height: i32 },

    //#[cfg(feature = "mouse")]
    /// Represent the current mouse's position after it was moved
    /// `mouse` feature must be enabled
    MouseMove { x: i32, y: i32 },

    //#[cfg(feature = "mouse")]
    /// A mouse button is down on this position
    /// `mouse` feature must be enabled
    MouseDown { button: MouseButton, x: i32, y: i32 },

    //#[cfg(feature = "mouse")]
    /// A mouse button was released on this position
    /// `mouse` feature must be enabled
    MouseUp { button: MouseButton, x: i32, y: i32 },

    //#[cfg(feature = "mouse")]
    /// Mouse wheel was moved and this are his delta
    /// `mouse` feature must be enabled
    MouseWheel { delta_x: f32, delta_y: f32 },

    /// Mouse cursor enter to the window's app
    MouseEnter { x: i32, y: i32 },

    /// Mouse cursor has left the window's app
    MouseLeft { x: i32, y: i32 },
}

//#[cfg(feature = "mouse")]
#[derive(Clone, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
/// Represents a button of a mouse
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}
