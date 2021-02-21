use crate::device::{DropManager, ResourceId};
use crate::pipeline::*;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Debug)]
struct BufferId {
    id: i32,
    drop_manager: Arc<DropManager>,
}

impl Drop for BufferId {
    fn drop(&mut self) {
        self.drop_manager.push(ResourceId::Buffer(self.id));
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    id: Arc<BufferId>,
    pub(crate) usage: BufferUsage,
    pub draw: Option<DrawType>,
}

impl Buffer {
    pub(crate) fn new(
        id: i32,
        usage: BufferUsage,
        draw: Option<DrawType>,
        drop_manager: Arc<DropManager>,
    ) -> Self {
        let id = Arc::new(BufferId { id, drop_manager });

        Self { id, usage, draw }
    }

    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.id.id
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BufferUsage {
    Vertex,
    Index,
    Uniform(u32),
}

#[derive(Debug, Clone)]
pub struct VertexAttr {
    pub location: u32,
    pub format: VertexFormat,
}

impl VertexAttr {
    pub fn new(location: u32, vertex_data: VertexFormat) -> Self {
        Self {
            location: location,
            format: vertex_data,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VertexFormat {
    Float1,
    Float2,
    Float3,
    Float4,
}

impl VertexFormat {
    pub fn size(&self) -> i32 {
        match self {
            VertexFormat::Float1 => 1,
            VertexFormat::Float2 => 2,
            VertexFormat::Float3 => 3,
            VertexFormat::Float4 => 4,
        }
    }

    pub fn bytes(&self) -> i32 {
        self.size() * 4
    }

    pub fn normalized(&self) -> bool {
        use VertexFormat::*;
        match self {
            _ => false,
        }
    }
}
