use crate::graphics::{DropManager, ResourceId};
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

pub struct Buffer {
    id: Arc<BufferId>,
    usage: BufferUsage,
    draw: DrawType,
}

impl Buffer {
    pub(crate) fn new(
        id: i32,
        usage: BufferUsage,
        draw: DrawType,
        drop_manager: Arc<DropManager>,
    ) -> Self {
        let id = Arc::new(BufferId { id, drop_manager });

        Self { id, usage, draw }
    }

    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.id.id
    }

    #[inline(always)]
    pub fn draw_type(&self) -> DrawType {
        self.draw
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BufferUsage {
    Vertex,
    Index,
    // Uniform,
}

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

pub enum VertexFormat {
    Float1,
    Float2,
    Float3,
    Float4,
}

impl VertexFormat {
    pub fn size(&self) -> i32 {
        use VertexFormat::*;
        match self {
            Float1 => 1,
            Float2 => 2,
            Float3 => 3,
            Float4 => 4,
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
