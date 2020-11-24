use glow::*;
use hashbrown::HashMap;
use notan_graphics::prelude::*;
use notan_graphics::{Graphics, GraphicsBackend};
use std::rc::Rc;

mod pipeline;
mod utils;

pub struct GlowBackend {
    gl: Rc<Context>,
    buffer_count: i32,
    pipeline_count: i32,
    size: (i32, i32),
    pipelines: HashMap<i32, pipeline::InnerPipeline>,
}

impl GlowBackend {
    #[cfg(target_arch = "wasm32")]
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<Self, String> {
        let (gl, api) = utils::create_gl_context(canvas)?;
        notan_log::info!("Using {} graphics api", api);
        Ok(Self {
            pipeline_count: 0,
            buffer_count: 0,
            gl,
            size: (0, 0),
            pipelines: HashMap::new(),
        })
    }
}

impl GlowBackend {
    #[inline(always)]
    fn clear(&self, color: &Option<Color>, depth: &Option<f32>, stencil: &Option<i32>) {
        let mut mask = 0;
        unsafe {
            if let Some(color) = color {
                mask |= glow::COLOR_BUFFER_BIT;
                self.gl.clear_color(color.r, color.g, color.b, color.a);
            }

            if let Some(depth) = *depth {
                mask |= glow::DEPTH_BUFFER_BIT;
                self.gl.enable(glow::DEPTH_TEST);
                self.gl.depth_mask(true);
                self.gl.clear_depth_f32(depth);
            }

            if let Some(stencil) = *stencil {
                mask |= glow::STENCIL_BUFFER_BIT;
                self.gl.enable(glow::STENCIL_TEST);
                self.gl.stencil_mask(0xff);
                self.gl.clear_stencil(stencil);
            }

            self.gl.clear(mask);
        }
    }

    fn begin(
        &self,
        target: &Option<i32>,
        color: &Option<Color>,
        depth: &Option<f32>,
        stencil: &Option<i32>,
    ) {
        unsafe {
            let (width, height) = match &target {
                Some(_) => {
                    //Bind framebuffer to the target
                    (0, 0)
                } //TODO
                None => {
                    self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
                    self.size
                }
            };

            self.gl.viewport(0, 0, width, height);
        }

        self.clear(color, depth, stencil);
    }

    fn end(&mut self) {
        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            self.gl.bind_vertex_array(None);
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }

        //TODO pipeline clean and stats
    }

    fn clean_pipeline(&mut self, id: i32) {
        if let Some(pip) = self.pipelines.remove(&id) {
            pipeline::clean_pipeline(&self.gl, pip);
        }
    }

    fn set_pipeline(&mut self, id: i32, options: &PipelineOptions) {
        if let Some(pip) = self.pipelines.get(&id) {
            unsafe {
                self.gl.bind_vertex_array(Some(pip.vao));
                self.gl.use_program(Some(pip.program));

                // Stencil
                if utils::should_disable_stencil(&options.stencil) {
                    self.gl.disable(glow::STENCIL_TEST);
                } else {
                    if let Some(opts) = options.stencil {
                        self.gl.enable(glow::STENCIL_TEST);
                        self.gl.stencil_mask(opts.write_mask);
                        self.gl.stencil_op(
                            opts.stencil_fail.to_glow(),
                            opts.depth_fail.to_glow(),
                            opts.pass.to_glow(),
                        );
                        self.gl.stencil_func(
                            opts.compare.to_glow().unwrap_or(glow::ALWAYS),
                            opts.reference as _,
                            opts.read_mask,
                        );
                    }
                }

                // Depth stencil
                match options.depth_stencil.compare.to_glow() {
                    Some(d) => {
                        self.gl.enable(glow::DEPTH_TEST);
                        self.gl.depth_func(d);
                    }
                    _ => self.gl.disable(glow::DEPTH_TEST),
                }

                self.gl.depth_mask(options.depth_stencil.write);

                // Color mask
                self.gl.color_mask(
                    options.color_mask.r,
                    options.color_mask.g,
                    options.color_mask.b,
                    options.color_mask.a,
                );

                // Culling
                match options.cull_mode.to_glow() {
                    Some(mode) => {
                        self.gl.enable(glow::CULL_FACE);
                        self.gl.cull_face(mode);
                    }
                    _ => self.gl.disable(glow::CULL_FACE),
                }

                // Blend modes
                match (options.color_blend, options.alpha_blend) {
                    (Some(cbm), None) => {
                        self.gl.enable(glow::BLEND);
                        self.gl.blend_func(cbm.src.to_glow(), cbm.dst.to_glow());
                        self.gl.blend_equation(cbm.op.to_glow());
                    }
                    (Some(cbm), Some(abm)) => {
                        self.gl.enable(glow::BLEND);
                        self.gl.blend_func_separate(
                            cbm.src.to_glow(),
                            cbm.dst.to_glow(),
                            abm.src.to_glow(),
                            abm.dst.to_glow(),
                        );
                        self.gl
                            .blend_equation_separate(cbm.op.to_glow(), abm.op.to_glow());
                    }
                    (None, Some(abm)) => {
                        let cbm = BlendMode::NORMAL;
                        self.gl.enable(glow::BLEND);
                        self.gl.blend_func_separate(
                            cbm.src.to_glow(),
                            cbm.dst.to_glow(),
                            abm.src.to_glow(),
                            abm.dst.to_glow(),
                        );
                        self.gl
                            .blend_equation_separate(cbm.op.to_glow(), abm.op.to_glow());
                    }
                    (None, None) => {
                        self.gl.disable(glow::BLEND);
                    }
                }
            }
        }
    }
}

impl GraphicsBackend for GlowBackend {
    fn create_pipeline(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<i32, String> {
        let vertex_source = std::str::from_utf8(vertex_source).map_err(|e| e.to_string())?;

        let fragment_source = std::str::from_utf8(fragment_source).map_err(|e| e.to_string())?;

        let inner_pipeline = pipeline::create_pipeline(&self.gl, vertex_source, fragment_source)?;

        self.pipeline_count += 1;

        let id = self.pipeline_count;
        self.pipelines.insert(id, inner_pipeline);
        Ok(id)
    }

    fn create_vertex_buffer(&mut self, draw: DrawType) -> Result<i32, String> {
        self.buffer_count += 1;
        Ok(self.buffer_count)
    }

    fn create_index_buffer(&mut self, draw: DrawType) -> Result<i32, String> {
        self.buffer_count += 1;
        Ok(self.buffer_count)
    }

    fn render(&mut self, commands: &[Commands]) {
        commands.iter().for_each(|cmd| {
            use Commands::*;
            // notan_log::info!("{:?}", cmd);

            match cmd {
                Begin {
                    render_target,
                    color,
                    depth,
                    stencil,
                } => self.begin(render_target, color, depth, stencil),
                End => self.end(),
                Pipeline { id, options } => self.set_pipeline(*id, options),
                _ => {}
            }
        });
    }

    fn clean(&mut self, to_clean: &[ResourceId]) {
        notan_log::info!("{:?}", to_clean);
        to_clean.iter().for_each(|res| match &res {
            ResourceId::Pipeline(id) => self.clean_pipeline(*id),
            _ => {}
        })
    }

    fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
    }
}

pub trait ToGlow {
    fn to_glow(&self) -> u32;
}

pub trait ToOptionalGlow {
    fn to_glow(&self) -> Option<u32>;
}

impl ToGlow for StencilAction {
    fn to_glow(&self) -> u32 {
        use StencilAction::*;
        match self {
            Keep => glow::KEEP,
            Zero => glow::ZERO,
            Replace => glow::REPLACE,
            Increment => glow::INCR,
            IncrementWrap => glow::INCR_WRAP,
            Decrement => glow::DECR,
            DecrementWrap => glow::DECR_WRAP,
            Invert => glow::INVERT,
        }
    }
}

impl ToGlow for BlendOperation {
    fn to_glow(&self) -> u32 {
        use BlendOperation::*;
        match self {
            Add => glow::FUNC_ADD,
            Subtract => glow::FUNC_SUBTRACT,
            ReverseSubtract => glow::FUNC_REVERSE_SUBTRACT,
            Max => glow::MAX,
            Min => glow::MIN,
        }
    }
}

impl ToGlow for BlendFactor {
    fn to_glow(&self) -> u32 {
        use BlendFactor::*;
        match self {
            Zero => glow::ZERO,
            One => glow::ONE,
            SourceAlpha => glow::SRC_ALPHA,
            SourceColor => glow::SRC_COLOR,
            InverseSourceAlpha => glow::ONE_MINUS_SRC_ALPHA,
            InverseSourceColor => glow::ONE_MINUS_SRC_COLOR,
            DestinationAlpha => glow::DST_ALPHA,
            DestinationColor => glow::SRC_COLOR,
            InverseDestinationAlpha => glow::ONE_MINUS_DST_ALPHA,
            InverseDestinationColor => glow::ONE_MINUS_DST_COLOR,
        }
    }
}

impl ToOptionalGlow for CompareMode {
    fn to_glow(&self) -> Option<u32> {
        use CompareMode::*;
        Some(match self {
            None => return Option::None,
            Less => glow::LESS,
            Equal => glow::EQUAL,
            LEqual => glow::LEQUAL,
            Greater => glow::GREATER,
            NotEqual => glow::NOTEQUAL,
            GEqual => glow::GEQUAL,
            Always => glow::ALWAYS,
        })
    }
}

impl ToOptionalGlow for CullMode {
    fn to_glow(&self) -> Option<u32> {
        use CullMode::*;
        Some(match self {
            None => return Option::None,
            Front => glow::FRONT,
            Back => glow::BACK,
        })
    }
}
