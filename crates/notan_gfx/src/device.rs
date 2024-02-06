use crate::attrs::GfxAttributes;
use crate::buffer::{BufferDescriptor, NotanBuffer};
use crate::frame::NotanDrawFrame;
use crate::pipeline::{NotanRenderPipeline, RenderPipelineDescriptor};
use crate::render_target::RenderTarget;
use crate::render_texture::{NotanRenderTexture, RenderTextureDescriptor};
use crate::renderer::Renderer;
use crate::texture::{NotanSampler, NotanTexture, SamplerDescriptor, TextureData, TextureDescriptor};
use crate::{BindGroupDescriptor, DrawFrame, NotanBindGroup, NotanBindGroupLayoutRef};
use notan_core::window::{NotanWindow, WindowId};

pub trait NotanDevice<
    DF: NotanDrawFrame,
    RP: NotanRenderPipeline,
    B: NotanBuffer,
    T: NotanTexture,
    S: NotanSampler,
    BG: NotanBindGroup,
    BGL: NotanBindGroupLayoutRef,
    RT: NotanRenderTexture,
>
{
    fn new(attrs: GfxAttributes) -> Result<Self, String>
    where
        Self: Sized;
    fn create_frame(&mut self, window_id: WindowId) -> Result<DF, String>;
    fn present(&mut self, frame: DF) -> Result<(), String>;
    fn init_surface<W: NotanWindow>(&mut self, win: &W) -> Result<(), String>;
    fn create_render_pipeline(&mut self, desc: RenderPipelineDescriptor) -> Result<RP, String>;
    fn create_buffer(&mut self, desc: BufferDescriptor) -> Result<B, String>;
    fn create_render_texture(&mut self, desc: RenderTextureDescriptor) -> Result<RT, String>;
    fn create_texture(
        &mut self,
        desc: TextureDescriptor,
        data: Option<TextureData>,
    ) -> Result<T, String>;
    fn write_buffer(&mut self, buffer: &B, offset: u64, data: &[u8]) -> Result<(), String>;
    fn create_sampler(&mut self, desc: SamplerDescriptor) -> Result<S, String>;
    fn create_bind_group(&mut self, desc: BindGroupDescriptor) -> Result<BG, String>;
    fn resize(&mut self, id: WindowId, width: u32, height: u32) -> Result<(), String>;
    fn size(&self, id: WindowId) -> (u32, u32);

    fn render_to_frame(&mut self, frame: &DF, renderer: &Renderer) -> Result<(), String>;

    fn render_to_texture(&mut self, frame: &RT, renderer: &Renderer) -> Result<(), String>;
}
