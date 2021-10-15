use crate::device::{DropManager, ResourceId};
use crate::pipeline::*;
use crate::Device;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::sync::Arc;

/// Alias for the common used type for Vertex Buffer
pub type VertexBuffer = Buffer<f32>;

/// Alias for the common used type for Index Buffer
pub type IndexBuffer = Buffer<u32>;

/// Alias for the common used type for the Uniform Buffer
pub type UniformBuffer = Buffer<f32>;

#[derive(Debug)]
struct BufferId {
    id: u64,
    drop_manager: Arc<DropManager>,
}

impl Drop for BufferId {
    fn drop(&mut self) {
        self.drop_manager.push(ResourceId::Buffer(self.id));
    }
}

/// GPU buffer, it works as a thread-safe wrapper of a Vector
#[derive(Debug, Clone)]
pub struct Buffer<T>
where
    T: BufferDataType + Copy,
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
    T: BufferDataType + Copy,
{
    pub(crate) fn new(
        id: u64,
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
    pub fn id(&self) -> u64 {
        self.id.id
    }

    /// Read only reference for the inner buffer data
    pub fn data(&mut self) -> MappedRwLockReadGuard<'_, [T]> {
        RwLockReadGuard::map(self.data.read(), |data| data.as_slice())
    }

    /// Mutable reference for the inner buffer data
    pub fn data_mut(&mut self) -> MappedRwLockWriteGuard<'_, [T]> {
        RwLockWriteGuard::map(self.data.write(), |data| data.as_mut())
    }

    /// Replace the inner buffer data with the data from the passed data if both are the same length. It will panic otherwise
    pub fn copy(&mut self, data: &[T]) {
        self.data.write().copy_from_slice(data);
    }

    /// Clear and replace the innner buffer data no matter the length
    pub fn set(&mut self, data: &[T]) {
        let mut d = self.data.write();
        d.clear();
        d.extend(data);
    }

    /// It will clear the inner buffer data
    pub fn clear(&mut self) {
        self.data.write().clear();
    }

    /// It will extend the inner buffer data
    pub fn extend(&mut self, data: &[T]) {
        self.data.write().extend(data);
    }

    /// Returns the Arc reference for the inner data,
    /// useful if we need to do more than one action with the buffer
    /// we can acquire the mut with write and do all actions with that reference
    pub fn data_ptr(&self) -> &Arc<RwLock<Vec<T>>> {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> std::cmp::PartialEq for Buffer<T>
where
    T: BufferDataType + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

pub struct BufferBuilder<'a, T>
where
    T: BufferDataType + Copy,
{
    device: &'a mut Device,
    usage: BufferUsage,
    data: Option<Vec<T>>,
    ubo_name: Option<String>,
}

impl<'a, T> BufferBuilder<'a, T>
where
    T: BufferDataType + Copy,
{
    pub fn new(device: &'a mut Device, usage: BufferUsage, ubo_name: Option<&str>) -> Self {
        Self {
            device,
            usage,
            data: None,
            ubo_name: ubo_name.map(|v| v.to_string()),
        }
    }

    pub fn with_data(mut self, data: Vec<T>) -> Self {
        self.data = Some(data);
        self
    }
}

pub trait BufferBuildImpl<T>
where
    T: BufferDataType + Copy,
{
    fn build(self) -> Result<Buffer<T>, String>;
}

impl BufferBuildImpl<f32> for BufferBuilder<'_, f32> {
    fn build(self) -> Result<Buffer<f32>, String> {
        let Self {
            device,
            usage,
            data,
            ubo_name,
        } = self;

        match usage {
            BufferUsage::Vertex => {
                device.create_vertex_buffer(data.unwrap_or_else(std::vec::Vec::new))
            }
            BufferUsage::Uniform(loc) => device.create_uniform_buffer(
                loc,
                &ubo_name.ok_or_else(|| "Missing UBO name".to_string())?,
                data.unwrap_or_else(std::vec::Vec::new),
            ),
            _ => Err("Invalid Buffer Type...".to_string()),
        }
    }
}

impl BufferBuildImpl<u32> for BufferBuilder<'_, u32> {
    fn build(self) -> Result<Buffer<u32>, String> {
        let Self {
            device,
            usage,
            data,
            ..
        } = self;

        match usage {
            BufferUsage::Index => {
                device.create_index_buffer(data.unwrap_or_else(std::vec::Vec::new))
            }
            _ => Err("Invalid Buffer Type...".to_string()),
        }
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
            location,
            format: vertex_data,
        }
    }
}

// FIXME: VertexBuffer only support f32, add u8 support
#[derive(Debug, Clone, Copy)]
pub enum VertexFormat {
    Float1,
    Float2,
    Float3,
    Float4,
    // Uint1,
    // Uint2,
    // Uint3,
    // Uint4
}

impl VertexFormat {
    pub fn size(&self) -> i32 {
        match self {
            VertexFormat::Float1 => 1,
            VertexFormat::Float2 => 2,
            VertexFormat::Float3 => 3,
            VertexFormat::Float4 => 4,
            // VertexFormat::Uint1 => 1,
            // VertexFormat::Uint2 => 2,
            // VertexFormat::Uint3 => 3,
            // VertexFormat::Uint4 => 4,
        }
    }

    pub fn bytes(&self) -> i32 {
        self.size() * 4
        // TODO if Uint return self.size() if float return self.size() * 4
    }

    pub fn normalized(&self) -> bool {
        false //check type
              // todo u8 = true, float = false
    }
}
