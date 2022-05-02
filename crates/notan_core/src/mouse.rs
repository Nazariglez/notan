#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
/// Represents a button of a mouse
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}
