use crate::mouse::*;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// Application events usually received from the user
pub enum Event {
    /// When the app is about to close
    Exit,

    /// Reoresents the current window's position after it was moved
    WindowMove { x: i32, y: i32 },

    /// Represents the current window's size after it was resized
    WindowResize { width: i32, height: i32 },

    /// Represents a change on the screen aspect ration
    ScreenAspectChange { ratio: f32 },

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

    /// Mouse cursos has left the window's app
    MouseLeft { x: i32, y: i32 },

    /// Keyboard's key is down
    KeyDown { key: KeyCode },

    /// Keyboard's key was released
    KeyUp { key: KeyCode },

    /// Unicode char pressed
    ReceivedCharacter(char),
}

#[derive(Debug, Clone, Default)]
/// Event iterator queue
pub struct EventIterator(VecDeque<Event>);

impl EventIterator {
    pub fn new() -> Self {
        Default::default()
    }

    /// Remove and return the last element on the queue
    pub fn pop(&mut self) -> Option<Event> {
        self.0.pop_front()
    }

    /// Add an event at the beginning of the list
    pub fn push(&mut self, evt: Event) {
        self.0.push_back(evt);
    }

    /// Return the events and clear the list
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

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
/// KeyCode from winit adding 'unknown'
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
    NumpadAdd,
    NumpadDivide,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadMultiply,
    NumpadSubtract,

    AbntC1,
    AbntC2,
    Apostrophe,
    Apps,
    Asterisk,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
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
    Mute,
    MyComputer,
    // also called "Next"
    NavigateForward,
    // also called "Prior"
    NavigateBackward,
    NextTrack,
    NoConvert,
    OEM102,
    Period,
    PlayPause,
    Plus,
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
