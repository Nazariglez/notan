use glow::*;
use hashbrown::HashMap;
use notan_graphics::prelude::*;
use notan_graphics::{Graphics, GraphicsBackend};
use std::rc::Rc;

mod buffer;
mod pipeline;
mod to_glow;
mod utils;

use buffer::InnerBuffer;
use pipeline::{InnerPipeline, VertexAttributes};

pub struct GlowBackend {
    gl: Rc<Context>,
    buffer_count: i32,
    pipeline_count: i32,
    size: (i32, i32),
    pipelines: HashMap<i32, InnerPipeline>,
    buffers: HashMap<i32, InnerBuffer>,
    current_vertex_attrs: Option<VertexAttributes>,
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
            current_vertex_attrs: None,
            buffers: HashMap::new(),
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

        self.current_vertex_attrs = None;
        //TODO pipeline clean and stats
    }

    fn clean_pipeline(&mut self, id: i32) {
        if let Some(pip) = self.pipelines.remove(&id) {
            pip.clean(&self.gl);
        }
    }

    fn set_pipeline(&mut self, id: i32, options: &PipelineOptions) {
        if let Some(pip) = self.pipelines.get(&id) {
            pip.bind(&self.gl, options);
            self.current_vertex_attrs = Some(pip.attrs.clone());
        }
    }

    fn bind_buffer(&mut self, id: i32, data: &[u8], usage: &BufferUsage, draw: &DrawType) {
        if let Some(buffer) = self.buffers.get(&id) {
            match usage {
                BufferUsage::Vertex => {
                    buffer.bind_as_vbo_with_data(&self.gl, &self.current_vertex_attrs, draw, data)
                }
                BufferUsage::Index => buffer.bind_as_ebo_with_data(&self.gl, draw, data),
            }
        }
    }

    fn clean_buffer(&mut self, id: i32) {
        if let Some(buffer) = self.buffers.remove(&id) {
            buffer.clean(&self.gl);
        }
    }

    fn draw(&mut self, offset: i32, count: i32) {
        unsafe {
            self.gl.draw_arrays(glow::TRIANGLES, offset, count);
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

        let inner_pipeline =
            InnerPipeline::new(&self.gl, vertex_source, fragment_source, vertex_attrs)?;
        inner_pipeline.bind(&self.gl, &options);

        self.pipeline_count += 1;
        self.pipelines.insert(self.pipeline_count, inner_pipeline);
        Ok(self.pipeline_count)
    }

    fn create_vertex_buffer(&mut self, _draw: DrawType) -> Result<i32, String> {
        let inner_buffer = InnerBuffer::new(&self.gl)?;
        inner_buffer.bind_as_vbo(&self.gl, &self.current_vertex_attrs);
        self.buffer_count += 1;
        self.buffers.insert(self.buffer_count, inner_buffer);
        Ok(self.buffer_count)
    }

    fn create_index_buffer(&mut self, _draw: DrawType) -> Result<i32, String> {
        let inner_buffer = InnerBuffer::new(&self.gl)?;
        inner_buffer.bind_as_ebo(&self.gl);
        self.buffer_count += 1;
        self.buffers.insert(self.buffer_count, inner_buffer);
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
                BindBuffer {
                    id,
                    ptr,
                    usage,
                    draw,
                } => self.bind_buffer(*id, ptr, usage, draw),
                Draw { offset, count } => self.draw(*offset, *count),
                _ => {}
            }
        });
    }

    fn clean(&mut self, to_clean: &[ResourceId]) {
        notan_log::info!("{:?}", to_clean);
        to_clean.iter().for_each(|res| match &res {
            ResourceId::Pipeline(id) => self.clean_pipeline(*id),
            ResourceId::Buffer(id) => self.clean_buffer(*id),
            _ => {}
        })
    }

    fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
    }
}
