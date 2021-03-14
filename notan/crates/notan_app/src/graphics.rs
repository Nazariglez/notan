use notan_draw::Draw;
use notan_draw::*;
use notan_graphics::prelude::*;
pub use notan_graphics::*;

pub struct Graphics {
    device: Device,
    draw: DrawManager<'static>,
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
    pub fn create_renderer<'a>(&self) -> renderer::Renderer<'a> {
        self.device.create_renderer()
    }

    #[inline(always)]
    pub fn create_draw<'a>(&self) -> Draw<'a> {
        let (width, height) = self.device.size();
        self.draw.create_draw(width, height)
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

    // #[inline(always)]
    // pub fn render_to<'a>(&mut self, target: &RenderTexture, render: &'a ToCommandBuffer<'a>) {
    //     self.device.render_to(target, render)
    // }
    //
    // #[inline(always)]
    // pub fn render<'a>(&mut self, render: &'a ToCommandBuffer<'a>) {
    //     self.device.render(render)
    // }

    pub fn render_to<'a>(
        &'a mut self,
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

    pub fn render<'a>(&'a mut self, render: impl Into<GraphicsRenderer<'a>>) {
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
    pub fn create_uniform_buffer(&mut self, slot: u32) -> Result<Buffer, String> {
        self.device.create_uniform_buffer(slot)
    }

    #[inline(always)]
    pub fn create_index_buffer(&mut self) -> Result<Buffer, String> {
        self.device.create_index_buffer()
    }

    #[inline(always)]
    pub fn create_vertex_buffer(&mut self) -> Result<Buffer, String> {
        self.device.create_vertex_buffer()
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
    pub fn create_draw_pipeline(
        &mut self,
        mode: DrawMode,
        fragment: &ShaderSource,
    ) -> Result<Pipeline, String> {
        self.draw
            .create_pipeline(&mut self.device, mode, Some(fragment))
    }

    #[inline(always)]
    pub fn create_draw_pipeline_from_raw(
        &mut self,
        mode: DrawMode,
        fragment: &[u8],
    ) -> Result<Pipeline, String> {
        self.draw
            .create_pipeline_from_raw(&mut self.device, mode, Some(fragment))
    }
}

fn commands_from<'a>(
    gfx: &'a mut Graphics,
    render: impl Into<GraphicsRenderer<'a>>,
) -> &'a [Commands<'a>] {
    match render.into() {
        GraphicsRenderer::Raw(r) => r,
        GraphicsRenderer::Device(r) => r.commands_from(&mut gfx.device),
        GraphicsRenderer::Draw(r) => r.commands(&mut gfx.device, &mut gfx.draw),
    }
}

pub enum GraphicsRenderer<'a> {
    Raw(&'a [Commands<'a>]),
    Device(&'a DeviceRenderer<'a>),
    Draw(&'a DrawRenderer<'a>),
}

impl<'a> From<&'a [Commands<'a>]> for GraphicsRenderer<'a> {
    fn from(r: &'a [Commands<'a>]) -> GraphicsRenderer<'a> {
        GraphicsRenderer::Raw(r)
    }
}

impl<'a> From<&'a Renderer<'a>> for GraphicsRenderer<'a> {
    fn from(r: &'a Renderer<'a>) -> GraphicsRenderer<'a> {
        GraphicsRenderer::Device(r)
    }
}

impl<'a> From<&'a Draw<'a>> for GraphicsRenderer<'a> {
    fn from(r: &'a Draw<'a>) -> GraphicsRenderer<'a> {
        GraphicsRenderer::Draw(r)
    }
}
