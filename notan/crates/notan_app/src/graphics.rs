pub use notan_draw::*;
use notan_draw::*;
use notan_graphics::prelude::*;
pub use notan_graphics::*;

pub struct Graphics {
    device: Device,
    draw: DrawManager,
}

impl Graphics {
    pub fn new(backend: Box<DeviceBackend>) -> Result<Self, String> {
        let mut device = Device::new(backend)?;
        let draw = DrawManager::new(&mut device)?;
        Ok(Self { device, draw })
    }

    #[inline(always)]
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.device.set_size(width, height)
    }

    #[inline(always)]
    pub fn create_renderer(&self) -> renderer::Renderer {
        self.device.create_renderer()
    }

    #[inline(always)]
    pub fn create_draw(&self) -> Draw {
        let (width, height) = self.device.size();
        self.draw.create_draw(width, height)
    }

    #[inline(always)]
    pub fn create_draw2(&self) -> Draw2 {
        let (width, height) = self.device.size();
        self.draw.create_draw2(width, height)
    }

    #[inline(always)]
    pub fn clean(&mut self) {
        self.device.clean()
    }

    #[inline(always)]
    pub fn size(&self) -> (i32, i32) {
        self.device.size()
    }

    #[inline(always)]
    pub fn create_texture(&mut self, info: TextureInfo) -> Result<Texture, String> {
        self.device.create_texture(info)
    }

    pub fn render_to<'a>(
        &mut self,
        target: &RenderTexture,
        render: impl Into<GraphicsRenderer<'a>>,
    ) {
        let commands = match render.into() {
            GraphicsRenderer::Raw(r) => r,
            GraphicsRenderer::Device(r) => r.commands_from(&mut self.device),
            GraphicsRenderer::Draw(r) => r.commands(&mut self.device, &mut self.draw),
        };
        self.device.render_to(target, commands);
    }

    pub fn render<'a>(&mut self, render: impl Into<GraphicsRenderer<'a>>) {
        let commands = match render.into() {
            GraphicsRenderer::Raw(r) => r,
            GraphicsRenderer::Device(r) => r.commands_from(&mut self.device),
            GraphicsRenderer::Draw(r) => r.commands(&mut self.device, &mut self.draw),
        };
        self.device.render(commands);
    }

    #[inline(always)]
    pub fn create_render_texture(&mut self, info: TextureInfo) -> Result<RenderTexture, String> {
        self.device.create_render_texture(info)
    }

    #[inline(always)]
    pub fn create_uniform_buffer(
        &mut self,
        slot: u32,
        name: &str,
        data: Vec<f32>,
    ) -> Result<Buffer<f32>, String> {
        self.device.create_uniform_buffer(slot, name, data)
    }

    #[inline(always)]
    pub fn create_index_buffer(&mut self, data: Vec<u32>) -> Result<Buffer<u32>, String> {
        self.device.create_index_buffer(data)
    }

    #[inline(always)]
    pub fn create_vertex_buffer(&mut self, data: Vec<f32>) -> Result<Buffer<f32>, String> {
        self.device.create_vertex_buffer(data)
    }

    #[inline(always)]
    pub fn create_pipeline(
        &mut self,
        vertex_source: &ShaderSource,
        fragment_source: &ShaderSource,
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<Pipeline, String> {
        self.device
            .create_pipeline(vertex_source, fragment_source, vertex_attrs, options)
    }

    #[inline(always)]
    pub fn create_pipeline_from_raw(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<Pipeline, String> {
        self.device
            .create_pipeline_from_raw(vertex_source, fragment_source, vertex_attrs, options)
    }

    #[inline(always)]
    pub fn create_draw_image_pipeline(
        &mut self,
        fragment: Option<&ShaderSource>,
    ) -> Result<Pipeline, String> {
        self.draw.create_image_pipeline(&mut self.device, fragment)
    }

    #[inline(always)]
    pub fn create_draw_pattern_pipeline(
        &mut self,
        fragment: Option<&ShaderSource>,
    ) -> Result<Pipeline, String> {
        self.draw
            .create_pattern_pipeline(&mut self.device, fragment)
    }

    #[inline(always)]
    pub fn create_draw_shape_pipeline(
        &mut self,
        fragment: Option<&ShaderSource>,
    ) -> Result<Pipeline, String> {
        self.draw.create_shape_pipeline(&mut self.device, fragment)
    }

    #[inline(always)]
    pub fn create_draw_text_pipeline(
        &mut self,
        fragment: Option<&ShaderSource>,
    ) -> Result<Pipeline, String> {
        self.draw.create_text_pipeline(&mut self.device, fragment)
    }
}

pub enum GraphicsRenderer<'a> {
    Raw(&'a [Commands]),
    Device(&'a DeviceRenderer),
    Draw(&'a DrawRenderer),
}

impl<'a> From<&'a [Commands]> for GraphicsRenderer<'a> {
    fn from(r: &'a [Commands]) -> GraphicsRenderer {
        GraphicsRenderer::Raw(r)
    }
}

impl<'a> From<&'a Renderer> for GraphicsRenderer<'a> {
    fn from(r: &'a Renderer) -> GraphicsRenderer {
        GraphicsRenderer::Device(r)
    }
}

impl<'a> From<&'a Draw> for GraphicsRenderer<'a> {
    fn from(r: &'a Draw) -> GraphicsRenderer {
        GraphicsRenderer::Draw(r)
    }
}

impl<'a> From<&'a Draw2> for GraphicsRenderer<'a> {
    fn from(r: &'a Draw2) -> GraphicsRenderer {
        GraphicsRenderer::Draw(r)
    }
}
