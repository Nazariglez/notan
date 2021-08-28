pub use notan_draw::*;
use notan_graphics::prelude::*;
pub use notan_graphics::*;

#[cfg(feature = "glyphs")]
use notan_glyph::{GlyphManager, GlyphRenderer, Text};

pub struct Graphics {
    device: Device,
    draw: DrawManager,

    #[cfg(feature = "glyphs")]
    glyphs: GlyphManager,
}

impl Graphics {
    pub fn new(backend: Box<DeviceBackend>) -> Result<Self, String> {
        let mut device = Device::new(backend)?;
        let draw = DrawManager::new(&mut device)?;

        #[cfg(feature = "glyphs")]
        let glyphs = GlyphManager::new(&mut device)?;

        Ok(Self {
            device,
            draw,

            #[cfg(feature = "glyphs")]
            glyphs,
        })
    }

    pub fn create_pipeline2(&mut self) -> PipelineBuilder {
        PipelineBuilder::new(&mut self.device)
    }

    #[inline(always)]
    pub fn create_font(&mut self, data: &'static [u8]) -> Result<Font, String> {
        self.glyphs.create_font(data)
    }

    #[cfg(feature = "glyphs")]
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

    #[cfg(feature = "glyphs")]
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

enum Shaders<'b> {
    Raw {
        vertex: &'b [u8],
        fragment: &'b [u8],
    },

    Source {
        vertex: &'b ShaderSource<'b>,
        fragment: &'b ShaderSource<'b>,
    },
}

/// Pipeline builder pattern
pub struct PipelineBuilder<'a, 'b> {
    device: &'a mut Device,
    attrs: Vec<VertexAttr>,
    options: PipelineOptions,
    shaders: Option<Shaders<'b>>,
}

impl<'a, 'b> PipelineBuilder<'a, 'b> {
    pub fn new(device: &'a mut Device) -> Self {
        Self {
            device,
            attrs: vec![],
            options: Default::default(),
            shaders: None,
        }
    }

    /// Set the shaders from a ShaderSource object
    pub fn from(mut self, vertex: &'b ShaderSource, fragment: &'b ShaderSource) -> Self {
        self.shaders = Some(Shaders::Source { vertex, fragment });
        self
    }

    /// Set the shaders from a bytes slice
    pub fn from_raw(mut self, vertex: &'b [u8], fragment: &'b [u8]) -> Self {
        self.shaders = Some(Shaders::Raw { vertex, fragment });
        self
    }

    /// Set the format and location for a vertex attribute
    pub fn vertex_attr(mut self, location: u32, data: VertexFormat) -> Self {
        self.attrs.push(VertexAttr::new(location, data));
        self
    }

    /// Set the Color blending mode
    pub fn with_color_blend(mut self, color_blend: BlendMode) -> Self {
        self.options.color_blend = Some(color_blend);
        self
    }

    /// Set the alpha blending mode
    pub fn with_alpha_blend(mut self, alpha_blend: BlendMode) -> Self {
        self.options.alpha_blend = Some(alpha_blend);
        self
    }

    /// Set the Culling mode
    pub fn with_cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.options.cull_mode = cull_mode;
        self
    }

    /// Set the Depth Stencil options
    pub fn with_depth_stencil(mut self, depth_stencil: DepthStencil) -> Self {
        self.options.depth_stencil = depth_stencil;
        self
    }

    /// Set the Color Mask options
    pub fn with_color_mask(mut self, color_mask: ColorMask) -> Self {
        self.options.color_mask = color_mask;
        self
    }

    /// Set the Stencil options
    pub fn with_stencil(mut self, stencil: StencilOptions) -> Self {
        self.options.stencil = Some(stencil);
        self
    }

    /// Build the pipeline with the data set on the builder
    pub fn build(self) -> Result<Pipeline, String> {
        match self.shaders {
            Some(Shaders::Source { vertex, fragment }) => {
                self.device
                    .create_pipeline(vertex, fragment, &self.attrs, self.options)
            }
            Some(Shaders::Raw { vertex, fragment }) => {
                self.device
                    .create_pipeline_from_raw(vertex, fragment, &self.attrs, self.options)
            }
            _ => Err("Vertex and Fragment shaders should be present".to_string()),
        }
    }
}
