use crate::BaseApp;

/// Represent a resource
pub trait Resource {
    /// Dispatched when the resource is loaded on memory
    fn parse<T: BaseApp>(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String>;

    /// Should return true if the resource is ready to use it
    fn is_loaded(&self) -> bool;
}

/// Represent a resource that can be created from the trait
pub trait ResourceConstructor {
    /// Create a new resource
    fn new(file: &str) -> Self;
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

pub trait BaseTexture: Resource {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
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
    fn text_size<T: BaseApp, F: BaseFont>(app: &mut T, font: &F, text: &str, size: f32) -> (f32, f32);
    fn text_size_ext<T: BaseApp, F: BaseFont>(
        app: &mut T,
        font: &F,
        text: &str,
        size: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        max_width: Option<f32>,
    ) -> (f32, f32);
}