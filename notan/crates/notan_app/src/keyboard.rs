use crate::Event;
use hashbrown::{HashMap, HashSet};

#[derive(Default)]
/// Represent the keyboard data
pub struct Keyboard {
    /// pressed keys
    pub pressed: HashSet<KeyCode>,
    /// down keys
    pub down: HashMap<KeyCode, f32>,
    /// released keys
    pub released: HashSet<KeyCode>,
}

impl Keyboard {
    /// returns true if the key was released on the last frame
    pub fn was_released(&self, key: KeyCode) -> bool {
        self.released.contains(&key)
    }

    /// returns true if the key is still down
    pub fn is_down(&self, key: KeyCode) -> bool {
        self.down.contains_key(&key)
    }

    /// returns the total ime that this key is down
    pub fn down_delta(&self, key: KeyCode) -> f32 {
        *self.down.get(&key).unwrap_or(&0.0)
    }

    /// returns true if the key was pressed on the last frame
    pub fn was_pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub(crate) fn clear(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }

    #[inline]
    pub(crate) fn process_events(&mut self, evt: &Event, delta: f32) {
        match evt {
            Event::KeyUp { key } => {
                self.down.remove(key);
                self.pressed.remove(key);
                self.released.insert(*key);
            }

            Event::KeyDown { key } => {
                if let Some(t) = self.down.get_mut(key) {
                    *t += delta;
                } else {
                    self.down.insert(*key, 0.0);
                    self.pressed.insert(*key);
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
/// KeyCode represents the symbolic name of the keyboard keys pressed
/// This enum code comes from `winit` just adding the Unknown key for non-compatible keys between platforms
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
    Add,
    Divide,
    Decimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    Multiply,
    Subtract,

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
