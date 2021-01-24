use crate::buffer::*;
use crate::commands::*;
use crate::draw::*;
use crate::pipeline::*;
use crate::render_texture::*;
use crate::renderer::Renderer;
use crate::shader::*;
use crate::texture::*;
use parking_lot::RwLock;
use std::sync::Arc;

/// Graphics resource ID, used to know which resource was dropped
#[derive(Debug)]
pub enum ResourceId {
    Buffer(i32),
    Texture(i32),
    Pipeline(i32),
    RenderTexture(i32),
}

/// Represents a the implementation graphics backend like glow, wgpu or another
pub trait GraphicsBackend {
    /// Returns the name of the api used (like webgl, wgpu, etc...)
    fn api_name(&self) -> &str;

    /// Create a new pipeline and returns the id
    fn create_pipeline(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<i32, String>;

    /// Create a new vertex buffer object and returns the id
    fn create_vertex_buffer(&mut self) -> Result<i32, String>;

    /// Create a new index buffer object and returns the id
    fn create_index_buffer(&mut self) -> Result<i32, String>;

    /// Create a new uniform buffer and returns the id
    fn create_uniform_buffer(&mut self, slot: u32) -> Result<i32, String>;

    /// Create a new renderer using the size of the graphics
    fn render(&mut self, commands: &[Commands], target: Option<i32>);

    /// Clean all the dropped resources
    fn clean(&mut self, to_clean: &[ResourceId]);

    /// Sets the render size
    fn set_size(&mut self, width: i32, height: i32);

    /// Create a new texture and returns the id
    fn create_texture(&mut self, info: &TextureInfo) -> Result<i32, String>;

    /// Create a new render target and returns the id
    fn create_render_texture(&mut self, texture_id: i32, info: &TextureInfo)
        -> Result<i32, String>;
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
    draw_manager: DrawManager,
}

impl Graphics {
    pub fn new(mut backend: Box<GraphicsBackend>) -> Result<Self, String> {
        let draw_manager = DrawManager::new(&mut *backend)?;

        Ok(Self {
            backend,
            size: (1, 1),
            drop_manager: Arc::new(Default::default()),
            draw_manager,
        })
    }

    #[inline(always)]
    pub fn size(&self) -> (i32, i32) {
        self.size
    }

    #[inline(always)]
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
        self.backend.set_size(width, height);
    }

    #[inline(always)]
    pub fn create_renderer<'a>(&self) -> Renderer<'a> {
        Renderer::new(self.size.0, self.size.1)
    }

    #[inline(always)]
    pub fn create_draw<'a>(&self) -> Draw<'a> {
        Draw::new(self.size.0, self.size.1)
    }

    pub fn create_draw_pipeline_from_raw(
        &mut self,
        typ: DrawPipeline,
        fragment: Option<&[u8]>,
    ) -> Result<Pipeline, String> {
        create_draw_pipeline_from_raw(self, typ, fragment)
    }

    #[inline(always)]
    pub fn create_pipeline_from_raw(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<Pipeline, String> {
        let stride = vertex_attrs
            .iter()
            .fold(0, |acc, data| acc + data.format.bytes()) as usize;

        let id = self.backend.create_pipeline(
            vertex_source,
            fragment_source,
            vertex_attrs,
            options.clone(),
        )?;

        Ok(Pipeline::new(
            id,
            stride,
            options,
            self.drop_manager.clone(),
        ))
    }

    #[inline(always)]
    pub fn create_pipeline(
        &mut self,
        vertex_source: &ShaderSource,
        fragment_source: &ShaderSource,
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<Pipeline, String> {
        let api = self.backend.api_name();
        let vertex = vertex_source
            .get_source(api)
            .ok_or(format!("Vertex shader for api '{}' not available.", api))?;
        let fragment = fragment_source
            .get_source(api)
            .ok_or(format!("Fragment shader for api '{}' not available.", api))?;
        self.create_pipeline_from_raw(vertex, fragment, vertex_attrs, options)
    }

    #[inline(always)]
    pub fn create_vertex_buffer(&mut self) -> Result<Buffer, String> {
        let id = self.backend.create_vertex_buffer()?;
        Ok(Buffer::new(
            id,
            BufferUsage::Vertex,
            None,
            self.drop_manager.clone(),
        ))
    }

    #[inline(always)]
    pub fn create_index_buffer(&mut self) -> Result<Buffer, String> {
        let id = self.backend.create_index_buffer()?;
        Ok(Buffer::new(
            id,
            BufferUsage::Index,
            None,
            self.drop_manager.clone(),
        ))
    }

    #[inline(always)]
    pub fn create_uniform_buffer(&mut self, slot: u32) -> Result<Buffer, String> {
        //debug_assert!(current_pipeline.is_some()) //pipeline should be already binded
        let id = self.backend.create_uniform_buffer(slot)?;
        Ok(Buffer::new(
            id,
            BufferUsage::Uniform(slot),
            None,
            self.drop_manager.clone(),
        ))
    }

    #[inline(always)]
    pub fn create_texture(&mut self, info: TextureInfo) -> Result<Texture, String> {
        let (id) = self.backend.create_texture(&info)?;
        Ok(Texture::new(id, info, self.drop_manager.clone()))
    }

    #[inline]
    pub fn create_render_texture(&mut self, info: TextureInfo) -> Result<RenderTexture, String> {
        let tex_id = self.backend.create_texture(&info)?;

        let id = self.backend.create_render_texture(tex_id, &info)?;
        let texture = Texture::new(tex_id, info, self.drop_manager.clone());
        Ok(RenderTexture::new(id, texture, self.drop_manager.clone()))
    }

    #[inline(always)]
    pub fn render<'a>(&mut self, render: &'a ToCommandBuffer<'a>) {
        self.backend.render(render.commands(), None);
    }

    #[inline]
    pub fn render_to<'a>(&mut self, target: &RenderTexture, render: &'a ToCommandBuffer<'a>) {
        self.backend.render(render.commands(), Some(target.id()));
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
