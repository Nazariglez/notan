use crate::graphics::BaseContext2d;
use crate::{BaseApp, BaseSystem};

/// Represent a resource
pub trait Resource {
    type Context2d: BaseContext2d;

    /// Create a new resource
    fn new(file: &str) -> Self;

    /// Dispatched when the resource is loaded on memory
    fn parse<T, S>(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Context2d>;

    /// Should return true if the resource is ready to use it
    fn is_loaded(&self) -> bool;
}

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

pub trait BaseTexture: Resource
where
    Self: Sized,
{
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn from_size<T, S>(app: &mut T, width: i32, height: i32) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Context2d>;
    fn from<T, S>(
        app: &mut T,
        width: i32,
        height: i32,
        internal_format: TextureFormat,
        format: TextureFormat,
        min_filter: TextureFilter,
        mag_filter: TextureFilter,
    ) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Context2d>;
    fn format(&self) -> TextureFormat;
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

pub trait BaseFont: Resource {
    type Kind;

    fn text_size<T>(app: &mut T, font: &Self::Kind, text: &str, size: f32) -> (f32, f32)
    where
        T: BaseSystem<Context2d = Self::Context2d>;

    fn text_size_ext<T>(
        app: &mut T,
        font: &Self::Kind,
        text: &str,
        size: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        max_width: Option<f32>,
    ) -> (f32, f32)
    where
        T: BaseSystem<Context2d = Self::Context2d>;
}
