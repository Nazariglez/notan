use crate::shader::Shader;
use crate::{GlContext, GlowValue, Graphics, VertexAttr, VertexFormat};
use glow::HasContext;
use nae_core::{
    BaseGfx, BasePipeline, BlendMode, CompareMode, PipelineOptions, StencilAction, StencilOptions,
};
use std::rc::Rc;

type VaoKey = <glow::Context as HasContext>::VertexArray;

struct InnerVao {
    gl: GlContext,
    vao: VaoKey,
}

impl InnerVao {
    fn new(gl: &GlContext) -> Result<Self, String> {
        let gl = gl.clone();
        let vao = unsafe {
            let vao = gl.create_vertex_array()?;
            gl.bind_vertex_array(Some(vao));
            vao
        };

        Ok(Self { gl, vao })
    }
}

impl Drop for InnerVao {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vao);
        }
    }
}

impl PartialEq for InnerVao {
    fn eq(&self, other: &Self) -> bool {
        self.vao == other.vao
    }
}

#[derive(Clone)]
pub struct Pipeline {
    gl: GlContext,
    vao: Rc<InnerVao>,

    pub options: PipelineOptions,

    pub(crate) shader: Shader,
    pub(crate) stride: usize,
    pub(crate) attrs: Vec<GlowVertexAttr>,
}

impl PartialEq for Pipeline {
    fn eq(&self, other: &Self) -> bool {
        self.vao == other.vao
    }
}

fn create_pipeline(
    gfx: &Graphics,
    shader: Shader,
    attributes: &[VertexAttr],
    options: PipelineOptions,
) -> Result<Pipeline, String> {
    let gl = gfx.gl.clone();

    let vao = Rc::new(InnerVao::new(&gl)?);
    let stride = attributes
        .iter()
        .fold(0, |acc, data| acc + data.format.bytes()) as usize;

    let mut offset = 0;
    let attrs = attributes
        .iter()
        .map(|attr| {
            let parsed_attr = GlowVertexAttr {
                location: attr.location,
                size: attr.format.size(),
                data_type: attr.format.glow_value(),
                normalized: attr.format.normalized(),
                offset: offset,
            };
            offset += attr.format.bytes();

            parsed_attr
        })
        .collect::<Vec<_>>();

    Ok(Pipeline {
        gl,
        vao,
        options,
        shader,
        stride,
        attrs,
    })
}

impl Pipeline {
    pub const COLOR_VERTEX: &'static [u8] = include_bytes!("shaders/color.vert.spv");
    pub const COLOR_FRAG: &'static [u8] = include_bytes!("shaders/color.frag.spv");

    pub const IMAGE_VERTEX: &'static [u8] = include_bytes!("shaders/image.vert.spv");
    pub const IMAGE_FRAG: &'static [u8] = include_bytes!("shaders/image.frag.spv");

    pub const PATTERN_VERTEX: &'static [u8] = include_bytes!("shaders/pattern.vert.spv");
    pub const PATTERN_FRAG: &'static [u8] = include_bytes!("shaders/pattern.frag.spv");

    pub const TEXT_VERTEX: &'static [u8] = include_bytes!("shaders/text.vert.spv");
    pub const TEXT_FRAG: &'static [u8] = include_bytes!("shaders/text.frag.spv");

    pub fn new(
        gfx: &Graphics,
        vertex: &[u8],
        fragment: &[u8],
        attributes: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<Self, String> {
        let shader = Shader::new(gfx, vertex, fragment)?;
        create_pipeline(gfx, shader, attributes, options)
    }

    pub fn from_source(
        gfx: &Graphics,
        vertex: &str,
        fragment: &str,
        attributes: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<Self, String> {
        let shader = Shader::from_source(gfx, vertex, fragment)?;
        create_pipeline(gfx, shader, attributes, options)
    }

    pub fn from_color_fragment(gfx: &mut Graphics, fragment: &[u8]) -> Result<Self, String> {
        Self::new(
            gfx,
            Pipeline::COLOR_VERTEX,
            fragment,
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float4),
            ],
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
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float4),
                VertexAttr::new(2, VertexFormat::Float2),
            ],
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
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float4),
                VertexAttr::new(2, VertexFormat::Float2),
            ],
            PipelineOptions {
                color_blend: Some(BlendMode::NORMAL),
                ..Default::default()
            },
        )
    }

    pub fn from_text_fragment(gfx: &mut Graphics, fragment: &[u8]) -> Result<Self, String> {
        Self::new(
            gfx,
            Pipeline::TEXT_VERTEX,
            fragment,
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float4),
                VertexAttr::new(2, VertexFormat::Float2),
            ],
            PipelineOptions {
                color_blend: Some(BlendMode::NORMAL),
                ..Default::default()
            },
        )
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn offset(&self) -> usize {
        self.stride / 4
    }
}

impl BasePipeline for Pipeline {
    type Graphics = Graphics;

    fn bind(&self, gfx: &mut Self::Graphics) {
        unsafe {
            gfx.gl.bind_vertex_array(Some(self.vao.vao));
            gfx.gl.use_program(Some(self.shader.inner.raw));

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
        }
    }

    fn options(&mut self) -> &mut PipelineOptions {
        &mut self.options
    }

    fn uniform_location(&self, id: &str) -> Result<<Self::Graphics as BaseGfx>::Location, String> {
        unsafe {
            self.gl
                .get_uniform_location(self.shader.inner.raw, id)
                .ok_or(format!("Invalid uniform id: {}", id))
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

#[derive(Debug, Clone)]
pub(crate) struct GlowVertexAttr {
    pub location: u32,
    pub size: i32,
    pub data_type: u32,
    pub normalized: bool,
    pub offset: i32,
}
