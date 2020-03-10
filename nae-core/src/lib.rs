mod color;
mod geometry;
mod gfx;
mod pipeline;
mod resources;

pub mod log;
pub mod math;
pub mod window;

pub use color::*;
pub use geometry::*;
pub use gfx::*;
pub use pipeline::*;
pub use resources::*;

pub use rand;
pub use rand_pcg;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct BuilderOpts {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
    pub fps_target: Option<i32>,
    pub min_size: Option<(i32, i32)>,
    pub max_size: Option<(i32, i32)>,
    pub maximized: bool,
    pub resizable: bool,
}

impl Default for BuilderOpts {
    fn default() -> Self {
        Self {
            title: String::from("Nae App"),
            width: 800,
            height: 600,
            fullscreen: false,
            fps_target: None,
            min_size: None,
            max_size: None,
            maximized: false,
            resizable: false,
        }
    }
}

pub trait BaseApp {
    type System: BaseSystem;
    fn system(&mut self) -> &mut Self::System;
}

pub trait BaseSystem {
    type Kind: BaseSystem;
    type Context2d: gfx::BaseContext2d;
    type Graphics: BaseGfx;

    fn new(opts: BuilderOpts) -> Result<Self::Kind, String>;
    fn gfx(&mut self) -> Rc<RefCell<Self::Graphics>>;
    fn ctx2(&mut self) -> &mut Self::Context2d;
    fn events(&mut self) -> &mut EventIterator;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn dpi(&self) -> f32;
    fn set_fullscreen(&mut self, full: bool);
    fn fullscreen(&self) -> bool;
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

    /// Keyboard's key down
    KeyDown { key: KeyCode },

    /// Keyboard's key up
    KeyUp { key: KeyCode },

    /// Unicode char pressed
    ReceivedCharacter(char),
}

//#[cfg(feature = "mouse")]
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
/// Represents a button of a mouse
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

/// Represent a key
/// Enum from winit but added an Unknown key.
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum KeyCode {
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,
    /// The '0' key over the 'O' and 'P' keys.
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1.
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    /// Print Screen/SysRq.
    Snapshot,
    /// Scroll Lock.
    Scroll,
    /// Pause/Break key, next to Scroll lock.
    Pause,

    /// `Insert`, next to Backspace.
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    /// The Backspace key, right over Enter.
    // TODO: rename
    Back,
    /// The Enter key.
    Return,
    /// The space bar.
    Space,

    /// The "Compose" key on Linux.
    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,  // also called "Prior"
    NavigateBackward, // also called "Next"
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,

    Unknown,
}
