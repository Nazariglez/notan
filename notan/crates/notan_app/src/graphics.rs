pub use notan_draw::*;
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
    pub fn create_draw(&self) -> Draw {
        let (width, height) = self.device.size();
        self.draw.create_draw(width, height)
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

impl std::ops::Deref for Graphics {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl std::ops::DerefMut for Graphics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.device
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
