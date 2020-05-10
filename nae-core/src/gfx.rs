use super::math::Mat3;
use crate::resources::{BaseFont, BaseTexture, HorizontalAlign, VerticalAlign};
use crate::{BaseApp, BaseSystem, BlendMode, ClearOptions, Color, Geometry, PipelineOptions};

pub trait BaseSurface
where
    Self: Sized,
{
    type Texture: BaseTexture;
    type Context2d: BaseContext2d;

    fn from_size<T, S>(app: &mut T, width: i32, height: i32) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Context2d>;
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

    fn push_matrix(&mut self, matrix: &Mat3);
    fn push_scale(&mut self, sx: f32, sy: f32);
    fn push_translate(&mut self, x: f32, y: f32);
    fn push_skew(&mut self, x: f32, y: f32);
    fn push_rotation(&mut self, rad: f32);
    fn pop_matrix(&mut self);

    fn matrix_mut(&mut self) -> &mut Mat3;
    fn matrix(&self) -> &Mat3;

    fn new(device: &Self::Device) -> Result<Self, String>;
    fn set_shader(&mut self, shader: Option<&Self::Shader>);
    fn update_custom_shader(&mut self, shader: Option<&Self::Shader>);
    fn set_alpha(&mut self, alpha: f32);
    fn set_blend(&mut self, mode: BlendMode);
    fn set_size(&mut self, width: i32, height: i32);
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn set_color(&mut self, color: Color);
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
    fn geometry(&mut self, geometry: &Geometry);
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

/// Represents a graphics API
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GraphicsAPI {
    WebGl,
    WebGl2,
    WebGpu,
    OpenGl3_3,
    OpenGlEs2_0,
    Vulkan,
    Metal,
    Dx11,
    Dx12,
    Unknown(String),
}

pub trait BasePipeline {
    type Graphics: BaseGfx;

    fn bind(&self, gfx: &mut Self::Graphics);
    fn options(&mut self) -> &mut PipelineOptions;
    fn uniform_location(&self, id: &str) -> Result<<Self::Graphics as BaseGfx>::Location, String>;
}

// pub trait UniformValue {
//     type Graphics: BaseGfx;
//     fn bind_uniform(&self, gfx: &mut Self::Graphics, location: <Self::Graphics as BaseGfx>::Location);
// }

pub trait BaseVertexBuffer {
    type Graphics: BaseGfx;
    fn bind(
        &self,
        gfx: &mut Self::Graphics,
        pipeline: &<Self::Graphics as BaseGfx>::Pipeline,
        data: &[f32],
    );
}

pub trait BaseIndexBuffer {
    type Graphics: BaseGfx;
    fn bind(&self, gfx: &mut Self::Graphics, data: &[u32]);
}

pub trait BaseGfx
where
    Self: Sized,
{
    type Location;
    type Texture;
    type Pipeline;

    fn api(&self) -> GraphicsAPI;
    fn size(&self) -> (f32, f32);
    fn set_size(&mut self, width: f32, height: f32);
    fn viewport(&mut self, x: f32, y: f32, width: f32, height: f32);
    fn begin(&mut self, clear: &ClearOptions);
    fn bind_texture(&mut self, location: &Self::Location, texture: &Self::Texture);
    fn bind_texture_slot(&mut self, slot: u32, location: &Self::Location, texture: &Self::Texture);
    fn end(&mut self);
    fn set_pipeline(&mut self, pipeline: &Self::Pipeline);
    fn bind_vertex_buffer(
        &mut self,
        buffer: &BaseVertexBuffer<Graphics = Self>,
        pipeline: &Self::Pipeline,
        data: &[f32],
    );
    fn bind_index_buffer(&mut self, buffer: &BaseIndexBuffer<Graphics = Self>, data: &[u32]);
    fn draw(&mut self, offset: i32, count: i32);
    // fn bind_uniform(&mut self, location: Self::Location, value: &UniformValue<Graphics = Self>);
}
