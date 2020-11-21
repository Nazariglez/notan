// pub enum BufferUsage {
//     Vertex,
//     Index,
//     Uniform
// }
//
// pub struct Buffer<'a> {
//     usage: BufferUsage,
//     content: &'a [u8]
// }
//

use crate::pipeline::*;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Default)]
pub struct BufferId(pub i32);

pub struct Buffer {
    id: BufferId,
    usage: BufferUsage,
    draw: DrawType,
}

impl Buffer {
    pub fn new(id: BufferId, usage: BufferUsage, draw: DrawType) -> Self {
        Self { id, usage, draw }
    }

    #[inline(always)]
    pub fn id(&self) -> BufferId {
        self.id
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
    Uniform,
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
