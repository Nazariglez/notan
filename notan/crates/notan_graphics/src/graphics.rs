use crate::buffer::*;
use crate::commands::*;
use crate::pipeline::*;
use crate::renderer::Renderer;

pub trait GraphicsBackend {
    fn create_pipeline(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<PipelineId, String>;
    fn create_vertex_buffer(&mut self, draw: DrawType) -> Result<BufferId, String>;
    fn create_index_buffer(&mut self, draw: DrawType) -> Result<BufferId, String>;
    fn render(&mut self, commands: &[Commands]);
}

pub struct Graphics {
    size: (i32, i32),
    backend: Box<GraphicsBackend>, //TODO generic?
}

impl Graphics {
    pub fn new(backend: Box<GraphicsBackend>) -> Self {
        Self {
            backend,
            size: (1, 1),
        }
    }

    #[inline(always)]
    pub fn size(&self) -> (i32, i32) {
        self.size
    }

    #[inline(always)]
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
    }

    #[inline(always)]
    pub fn create_renderer<'a>(&self) -> Renderer<'a> {
        Renderer::new(self.size.0, self.size.1)
    }

    #[inline(always)]
    pub fn create_pipeline(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<Pipeline, String> {
        let id = self.backend.create_pipeline(
            vertex_source,
            fragment_source,
            vertex_attrs,
            options.clone(),
        )?;
        Ok(Pipeline::new(id, options))
    }

    #[inline(always)]
    pub fn create_vertex_buffer(&mut self, draw: DrawType) -> Result<Buffer, String> {
        let id = self.backend.create_vertex_buffer(draw)?;
        Ok(Buffer::new(id, BufferUsage::Vertex, draw))
    }

    #[inline(always)]
    pub fn create_index_buffer(&mut self, draw: DrawType) -> Result<Buffer, String> {
        let id = self.backend.create_index_buffer(draw)?;
        Ok(Buffer::new(id, BufferUsage::Index, draw))
    }

    #[inline(always)]
    pub fn render<'a>(&mut self, render: &'a ToCommandBuffer<'a>) {
        self.backend.render(render.commands());
    }
}
