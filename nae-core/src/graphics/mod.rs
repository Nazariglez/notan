use crate::{BaseApp, BaseSystem};
mod blend;
mod color;
mod geometry;
mod transform;

use crate::resources::{BaseFont, BaseTexture, HorizontalAlign, VerticalAlign};
pub use blend::*;
pub use color::*;
pub use geometry::*;
pub use transform::Transform2d;

pub trait BaseSurface
where
    Self: Sized,
{
    type Texture: BaseTexture;
    type Context2d: BaseContext2d;

    fn from_size<T: BaseSystem<Context2d = Self::Context2d>>(
        app: &mut T,
        width: i32,
        height: i32,
    ) -> Result<Self, String>;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn texture(&self) -> &Self::Texture;
}

pub trait BaseShader
where
    Self: Sized,
{
    type Graphics: BaseContext2d;
    type Buffer;
    type Attr;
    type Kind: BaseShader;

    fn new<T: BaseSystem<Context2d = Self::Graphics>>(
        app: &mut T,
        vertex: &str,
        fragment: &str,
        attributes: Vec<Self::Attr>,
    ) -> Result<Self, String>;
    fn buffer(&self, name: &str) -> Option<Self::Buffer>;
    fn from_image_fragment<T, S>(app: &mut T, fragment: &str) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Graphics>;
    fn from_text_fragment<T, S>(app: &mut T, fragment: &str) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Graphics>;
    fn from_color_fragment<T, S>(app: &mut T, fragment: &str) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Graphics>;

    fn is_equal(&self, shader: &Self::Kind) -> bool;

    //TODO find a way to include this in this trait keeping flexible to do something like fn<T: UniformTrait>(name: &str, value: T); where UniformTrait is defined on the impl not here...
    //    fn set_uniform<T>(&self, name: &str, value: T) -> Result<(), String>;
}

pub struct Vertex {
    pub pos: (f32, f32),
    pub color: Color,
}

impl Vertex {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self { pos: (x, y), color }
    }
}

pub trait BaseContext2d
where
    Self: Sized,
{
    type Device;
    type Shader: BaseShader;
    type Surface: BaseSurface;
    type Texture: BaseTexture;
    type Font: BaseFont;

    fn new(device: &Self::Device) -> Result<Self, String>;
    fn set_shader(&mut self, shader: Option<&Self::Shader>);
    fn update_custom_shader(&mut self, shader: Option<&Self::Shader>);
    fn set_alpha(&mut self, alpha: f32);
    fn set_blend(&mut self, mode: BlendMode);
    fn set_size(&mut self, width: i32, height: i32);
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn set_color(&mut self, color: Color);
    fn transform(&mut self) -> &mut Transform2d;
    fn begin_to_surface(&mut self, surface: Option<&Self::Surface>);
    fn begin(&mut self);
    fn end(&mut self);
    fn clear(&mut self, color: Color);
    fn begin_mask(&mut self);
    fn end_mask(&mut self);
    fn clear_mask(&mut self);
    fn flush(&mut self);
    fn set_font(&mut self, font: &Self::Font);
    fn font(&self) -> &Self::Font;
    fn text(&mut self, text: &str, x: f32, y: f32, size: f32);
    fn text_ext(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        max_width: Option<f32>,
    );
    fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32);
    fn stroke_triangle(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        line_width: f32,
    );
    fn rect(&mut self, x: f32, y: f32, width: f32, height: f32);
    fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32, line_width: f32);
    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, strength: f32);
    fn circle(&mut self, x: f32, y: f32, radius: f32);
    fn rounded_rect(&mut self, x: f32, y: f32, width: f32, height: f32, radius: f32);
    fn stroke_rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        radius: f32,
        line_width: f32,
    );
    fn stroke_circle(&mut self, x: f32, y: f32, radius: f32, line_width: f32);
    fn geometry(&mut self, geometry: &mut Geometry);
    fn image(&mut self, img: &Self::Texture, x: f32, y: f32);
    fn image_crop(
        &mut self,
        img: &Self::Texture,
        x: f32,
        y: f32,
        source_x: f32,
        source_y: f32,
        source_width: f32,
        source_height: f32,
    );
    fn image_ext(
        &mut self,
        img: &Self::Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        source_x: f32,
        source_y: f32,
        source_width: f32,
        source_height: f32,
    );
    fn pattern(
        &mut self,
        img: &Self::Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        offset_x: f32,
        offset_y: f32,
    );
    fn pattern_ext(
        &mut self,
        img: &Self::Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        offset_x: f32,
        offset_y: f32,
        scale_x: f32,
        scale_y: f32,
    );
    fn vertex(&mut self, vertices: &[Vertex]);
    fn image_9slice(&mut self, img: &Self::Texture, x: f32, y: f32, width: f32, height: f32);
    fn image_9slice_ext(
        &mut self,
        img: &Self::Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
    );
}
