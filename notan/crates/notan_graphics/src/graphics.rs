use crate::buffer::*;
use crate::commands::*;
use crate::pipeline::*;
use crate::renderer::Renderer;
use parking_lot::RwLock;
use std::sync::Arc;

/// Graphics resource ID, used to know which resource was dropped
#[derive(Debug)]
pub enum ResourceId {
    Buffer(i32),
    Texture(i32),
    Pipeline(i32),
}

/// Represents a the implementation graphics backend like glow, wgpu or another
pub trait GraphicsBackend {
    /// Create a new pipeline and returns the id
    fn create_pipeline(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<i32, String>;

    /// Create a new vertex buffer object and returns the id
    fn create_vertex_buffer(&mut self, draw: DrawType) -> Result<i32, String>;

    /// Create a new index buffer object and returns the id
    fn create_index_buffer(&mut self, draw: DrawType) -> Result<i32, String>;

    /// Create a new renderer using the size of the graphics
    fn render(&mut self, commands: &[Commands]);

    /// Clean all the dropped resources
    fn clean(&mut self, to_clean: &[ResourceId]);
}

/// Helper to drop resources on the backend
/// Like pipelines, textures, buffers
#[derive(Debug, Default)]
pub(crate) struct DropManager {
    dropped: RwLock<Vec<ResourceId>>,
}

impl DropManager {
    pub fn push(&self, id: ResourceId) {
        self.dropped.write().push(id);
    }

    pub fn clean(&self) {
        self.dropped.write().clear();
    }
}

pub struct Graphics {
    size: (i32, i32),
    backend: Box<GraphicsBackend>, //TODO generic?
    drop_manager: Arc<DropManager>,
}

impl Graphics {
    pub fn new(backend: Box<GraphicsBackend>) -> Self {
        Self {
            backend,
            size: (1, 1),
            drop_manager: Arc::new(Default::default()),
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
        Ok(Pipeline::new(id, options, self.drop_manager.clone()))
    }

    #[inline(always)]
    pub fn create_vertex_buffer(&mut self, draw: DrawType) -> Result<Buffer, String> {
        let id = self.backend.create_vertex_buffer(draw)?;
        Ok(Buffer::new(
            id,
            BufferUsage::Vertex,
            draw,
            self.drop_manager.clone(),
        ))
    }

    #[inline(always)]
    pub fn create_index_buffer(&mut self, draw: DrawType) -> Result<Buffer, String> {
        let id = self.backend.create_index_buffer(draw)?;
        Ok(Buffer::new(
            id,
            BufferUsage::Index,
            draw,
            self.drop_manager.clone(),
        ))
    }

    #[inline(always)]
    pub fn render<'a>(&mut self, render: &'a ToCommandBuffer<'a>) {
        self.backend.render(render.commands());
    }

    #[inline]
    pub fn clean(&mut self) {
        if self.drop_manager.dropped.read().is_empty() {
            return;
        }

        self.backend.clean(&self.drop_manager.dropped.read());
        self.drop_manager.clean();
    }
}
