use crate::{BaseApp, BaseGfx, BaseSystem};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFormat {
    Rgba,
    Red,
    R8,
    Depth,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFilter {
    Linear,
    Nearest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

/// Represents an external resource
pub trait Resource<T>: Clone {
    /// Create a empty resource ready to be loaded
    fn prepare(app: &mut T, file: &str) -> Result<Self, String>;

    /// Parse byte data to create to fill the resource
    fn set_data(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String>;
}
