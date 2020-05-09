use crate::shader::Shader;
use crate::{GlContext, GlowValue, Graphics};
use glow::HasContext;
use nae_core::{
    BaseGfx, BasePipeline, BlendMode, CompareMode, PipelineOptions, StencilAction, StencilOptions,
};

#[derive(Clone)]
pub struct Pipeline {
    gl: GlContext,
    vao: <glow::Context as HasContext>::VertexArray,
    pub(crate) shader: Shader,
    pub options: PipelineOptions,
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vao);
        }
    }
}

impl Pipeline {
    pub const COLOR_VERTEX: &'static [u8] = include_bytes!("shaders/color.vert.spv");
    pub const COLOR_FRAG: &'static [u8] = include_bytes!("shaders/color.frag.spv");

    pub const IMAGE_VERTEX: &'static [u8] = include_bytes!("shaders/image.vert.spv");
    pub const IMAGE_FRAG: &'static [u8] = include_bytes!("shaders/image.frag.spv");

    pub const PATTERN_VERTEX: &'static [u8] = include_bytes!("shaders/pattern.vert.spv");
    pub const PATTERN_FRAG: &'static [u8] = include_bytes!("shaders/pattern.frag.spv");

    pub fn new(
        gfx: &Graphics,
        vertex: &[u8],
        fragment: &[u8],
        options: PipelineOptions,
    ) -> Result<Self, String> {
        let gl = gfx.gl.clone();

        let shader = Shader::new(gfx, vertex, fragment)?;

        let vao = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            vao
        };

        Ok(Self {
            gl,
            vao,
            options,
            shader,
        })
    }

    pub fn from_color_fragment(gfx: &mut Graphics, fragment: &[u8]) -> Result<Self, String> {
        Self::new(
            gfx,
            Pipeline::COLOR_VERTEX,
            fragment,
            PipelineOptions {
                color_blend: Some(BlendMode::NORMAL),
                ..Default::default()
            },
        )
    }

    pub fn from_image_fragment(gfx: &mut Graphics, fragment: &[u8]) -> Result<Self, String> {
        Self::new(
            gfx,
            Pipeline::IMAGE_VERTEX,
            fragment,
            PipelineOptions {
                color_blend: Some(BlendMode::NORMAL),
                ..Default::default()
            },
        )
    }

    pub fn from_pattern_fragment(gfx: &mut Graphics, fragment: &[u8]) -> Result<Self, String> {
        Self::new(
            gfx,
            Pipeline::PATTERN_VERTEX,
            fragment,
            PipelineOptions {
                color_blend: Some(BlendMode::NORMAL),
                ..Default::default()
            },
        )
    }

    pub fn from_text_fragment(gfx: &mut Graphics, fragment: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}

impl BasePipeline for Pipeline {
    type Graphics = Graphics;

    fn bind(&self, gfx: &mut Self::Graphics) {
        unsafe {
            //Stencil
            if should_disable_stencil(&self.options.stencil) {
                self.gl.disable(glow::STENCIL_TEST);
            } else {
                if let Some(opts) = &self.options.stencil {
                    self.gl.enable(glow::STENCIL_TEST);
                    self.gl.stencil_mask(opts.write_mask);
                    self.gl.stencil_op(
                        opts.stencil_fail.glow_value(),
                        opts.depth_fail.glow_value(),
                        opts.pass.glow_value(),
                    );
                    self.gl.stencil_func(
                        opts.compare.glow_value().unwrap_or(glow::ALWAYS),
                        opts.reference as _,
                        opts.read_mask,
                    );
                }
            }

            //Depth stencil
            if let Some(d) = self.options.depth_stencil.compare.glow_value() {
                gfx.gl.enable(glow::DEPTH_TEST);
                gfx.gl.depth_func(d);
            } else {
                gfx.gl.disable(glow::DEPTH_TEST);
            }

            gfx.gl.depth_mask(self.options.depth_stencil.write);

            //Color mask
            self.gl.color_mask(
                self.options.color_mask.r,
                self.options.color_mask.g,
                self.options.color_mask.b,
                self.options.color_mask.a,
            );

            //Culling
            if let Some(mode) = self.options.cull_mode.glow_value() {
                gfx.gl.enable(glow::CULL_FACE);
                gfx.gl.cull_face(mode);
            } else {
                gfx.gl.disable(glow::CULL_FACE);
            }

            //Blend modes
            match (self.options.color_blend, self.options.alpha_blend) {
                (Some(cbm), None) => {
                    gfx.gl.enable(glow::BLEND);
                    gfx.gl
                        .blend_func(cbm.src.glow_value(), cbm.dst.glow_value());
                    gfx.gl.blend_equation(cbm.op.glow_value());
                }
                (Some(cbm), Some(abm)) => {
                    gfx.gl.enable(glow::BLEND);
                    gfx.gl.blend_func_separate(
                        cbm.src.glow_value(),
                        cbm.dst.glow_value(),
                        abm.src.glow_value(),
                        abm.dst.glow_value(),
                    );
                    gfx.gl
                        .blend_equation_separate(cbm.op.glow_value(), abm.op.glow_value());
                }
                (None, Some(abm)) => {
                    let cbm = BlendMode::NORMAL;
                    gfx.gl.enable(glow::BLEND);
                    gfx.gl.blend_func_separate(
                        cbm.src.glow_value(),
                        cbm.dst.glow_value(),
                        abm.src.glow_value(),
                        abm.dst.glow_value(),
                    );
                    gfx.gl
                        .blend_equation_separate(cbm.op.glow_value(), abm.op.glow_value());
                }
                (None, None) => {
                    gfx.gl.disable(glow::BLEND);
                }
            }

            gfx.gl.bind_vertex_array(Some(self.vao));
            gfx.gl.use_program(Some(self.shader.inner.raw));
        }
    }

    fn options(&mut self) -> &mut PipelineOptions {
        &mut self.options
    }

    fn uniform_location(&self, id: &str) -> <Self::Graphics as BaseGfx>::Location {
        unsafe {
            self.gl
                .get_uniform_location(self.shader.inner.raw, id)
                .unwrap()
        }
    }
}

fn should_disable_stencil(stencil: &Option<StencilOptions>) -> bool {
    match stencil {
        Some(stencil) => {
            stencil.compare == CompareMode::Always
                && stencil.stencil_fail == StencilAction::Keep
                && stencil.depth_fail == StencilAction::Keep
                && stencil.pass == StencilAction::Keep
        }
        None => true,
    }
}
