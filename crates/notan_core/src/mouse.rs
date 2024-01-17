use crate::window::WindowId;

#[derive(Copy, Clone, Debug)]
pub struct MouseEvent {
    pub window_id: WindowId,
    pub action: MouseAction,
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug)]
pub enum MouseAction {
    Wheel { delta_x: f32, delta_y: f32 },
    ButtonPressed { button: MouseButton },
    ButtonReleased { button: MouseButton },
    Move { relative_x: f32, relative_y: f32 },
    Enter,
    Left,
}

/// Represents a button of a mouse
#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}
