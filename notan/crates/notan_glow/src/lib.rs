use notan_graphics::prelude::*;
use notan_graphics::{Graphics, GraphicsBackend};
// use notan_graphics::commands::*;
use glow::*;
use std::rc::Rc;

mod utils;

pub struct GlowBackend {
    gl: Rc<Context>,
    buffer_count: i32,
    pipeline_count: i32,
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
        })
    }
}

impl GlowBackend {
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
}

impl GraphicsBackend for GlowBackend {
    fn create_pipeline(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<i32, String> {
        self.pipeline_count += 1;
        Ok(self.pipeline_count)
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
            // notan_log::info!("{:?}", cmd);

            match cmd {
                Commands::Begin {
                    color,
                    depth,
                    stencil,
                } => self.clear(color, depth, stencil),
                _ => {}
            }
        });
    }

    fn clean(&mut self, to_clean: &[ResourceId]) {
        notan_log::info!("{:?}", to_clean);
    }
}
