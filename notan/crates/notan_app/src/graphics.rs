pub use notan_draw::*;
use notan_glyph::{GlyphManager, GlyphRenderer, Text};
use notan_graphics::prelude::*;
pub use notan_graphics::*;

pub struct Graphics {
    device: Device,
    draw: DrawManager,
    glyphs: GlyphManager,
}

impl Graphics {
    pub fn new(backend: Box<DeviceBackend>) -> Result<Self, String> {
        let mut device = Device::new(backend)?;
        let draw = DrawManager::new(&mut device)?;
        let glyphs = GlyphManager::new(&mut device)?;
        Ok(Self {
            device,
            draw,
            glyphs,
        })
    }

    #[inline(always)]
    pub fn create_font(&mut self, data: &'static [u8]) -> Result<Font, String> {
        self.glyphs.create_font(data)
    }

    #[inline(always)]
    pub fn update_glyphs(&mut self, render: &mut GlyphRenderer) -> Result<(), String> {
        self.glyphs.update(&mut self.device, render)
    }

    #[inline(always)]
    pub fn process_text(&mut self, font: &Font, text: &Text) {
        self.glyphs.process_text(font, text);
    }

    #[inline(always)]
    pub fn create_draw(&self) -> Draw {
        let (width, height) = self.device.size();
        self.draw.create_draw(width, height)
    }

    #[inline(always)]
    pub fn glyphs_texture(&self) -> &Texture {
        &self.glyphs.texture
    }

    pub fn render_to<'a>(
        &mut self,
        target: &RenderTexture,
        render: impl Into<GraphicsRenderer<'a>>,
    ) {
        let commands = match render.into() {
            GraphicsRenderer::Raw(r) => r,
            GraphicsRenderer::Device(r) => r.commands_from(&mut self.device),
            GraphicsRenderer::Draw(r) => {
                r.commands(&mut self.device, &mut self.draw, &mut self.glyphs)
            }
        };
        self.device.render_to(target, commands);
    }

    pub fn render<'a>(&mut self, render: impl Into<GraphicsRenderer<'a>>) {
        let commands = match render.into() {
            GraphicsRenderer::Raw(r) => r,
            GraphicsRenderer::Device(r) => r.commands_from(&mut self.device),
            GraphicsRenderer::Draw(r) => {
                r.commands(&mut self.device, &mut self.draw, &mut self.glyphs)
            }
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
