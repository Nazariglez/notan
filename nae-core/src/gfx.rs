use super::math::Mat3;
use crate::resources::{HorizontalAlign, VerticalAlign};
use crate::{BaseApp, BaseSystem, BlendMode, ClearOptions, Color, Geometry, PipelineOptions};

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
