use crate::{BaseApp, BaseGfx, BaseSystem};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFormat {
    Rgba,
    Red,
    R8,
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

pub trait Resource: Clone {
    type Graphics: BaseGfx;
    /// Create a new empty resource
    fn empty<T, S>(app: &mut T) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Self::Graphics>;

    /// Parse byte data to create to fill the resource
    fn parse_data<T, S>(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Self::Graphics>;

    /// Returns if the texture is ready to render
    fn is_loaded(&self) -> bool;
}
