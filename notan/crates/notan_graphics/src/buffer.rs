use crate::device::{DropManager, ResourceId};
use crate::pipeline::*;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
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
pub struct Buffer<T>
where
    T: BufferDataType,
{
    id: Arc<BufferId>,
    pub(crate) usage: BufferUsage,
    pub draw: Option<DrawType>,
    data: Arc<RwLock<Vec<T>>>,
}

pub trait BufferDataType {}
impl BufferDataType for f32 {}
impl BufferDataType for u32 {}

impl<T> Buffer<T>
where
    T: BufferDataType,
{
    pub(crate) fn new(
        id: i32,
        data: Vec<T>,
        usage: BufferUsage,
        draw: Option<DrawType>,
        drop_manager: Arc<DropManager>,
    ) -> Self {
        let id = Arc::new(BufferId { id, drop_manager });
        let data = Arc::new(RwLock::new(data));

        Self {
            id,
            usage,
            draw,
            data,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.id.id
    }

    pub fn data(&mut self) -> MappedRwLockReadGuard<'_, [T]> {
        RwLockReadGuard::map(self.data.read(), |data| data.as_slice())
    }

    pub fn data_mut(&mut self) -> MappedRwLockWriteGuard<'_, [T]> {
        RwLockWriteGuard::map(self.data.write(), |data| data.as_mut())
    }

    pub fn data_ptr(&self) -> &Arc<RwLock<Vec<T>>> {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.read().len()
    }
}

impl<T> std::cmp::PartialEq for Buffer<T>
where
    T: BufferDataType,
{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
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
