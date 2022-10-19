use crate::device::{DropManager, ResourceId};
use crate::pipeline::*;
use crate::{BufferData, Device};
use std::sync::Arc;

#[derive(Debug)]
struct BufferIdRef {
    id: u64,
    drop_manager: Arc<DropManager>,
}

impl Drop for BufferIdRef {
    fn drop(&mut self) {
        self.drop_manager.push(ResourceId::Buffer(self.id));
    }
}

/// GPU buffer
#[derive(Debug, Clone)]
pub struct Buffer {
    id: u64,
    _id_ref: Arc<BufferIdRef>,
    pub usage: BufferUsage,
    pub draw: Option<DrawType>,
}

impl Buffer {
    pub(crate) fn new(
        id: u64,
        usage: BufferUsage,
        draw: Option<DrawType>,
        drop_manager: Arc<DropManager>,
    ) -> Self {
        let id_ref = Arc::new(BufferIdRef { id, drop_manager });

        Self {
            id,
            _id_ref: id_ref,
            usage,
            draw,
        }
    }

    /// Returns the inner id of the buffer
    #[inline(always)]
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns true if it's a uniform's buffer
    pub fn is_uniform(&self) -> bool {
        matches!(self.usage, BufferUsage::Uniform(_))
    }

    /// Returns true if it's a vertex's buffer
    pub fn is_vertex(&self) -> bool {
        matches!(self.usage, BufferUsage::Vertex)
    }

    /// Returns true if it's a element's buffer
    pub fn is_index(&self) -> bool {
        matches!(self.usage, BufferUsage::Index)
    }
}

impl std::cmp::PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

pub struct VertexBufferBuilder<'a> {
    device: &'a mut Device,
    data: Option<&'a [f32]>,
    vertex_attrs: Vec<VertexAttr>,
    vertex_step_mode: VertexStepMode,
}

impl<'a> VertexBufferBuilder<'a> {
    pub fn new(device: &'a mut Device) -> Self {
        Self {
            device,
            data: None,
            vertex_attrs: vec![],
            vertex_step_mode: VertexStepMode::Vertex,
        }
    }

    pub fn with_data(mut self, data: &'a [f32]) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_info(mut self, info: &VertexInfo) -> Self {
        self.vertex_attrs = info.attrs.clone();
        self.vertex_step_mode = info.step_mode;
        self
    }

    pub fn build(self) -> Result<Buffer, String> {
        let Self {
            device,
            data,
            vertex_attrs,
            vertex_step_mode,
        } = self;

        debug_assert!(
            !vertex_attrs.is_empty(),
            "Missing vertex attributes for a VertexBuffer"
        );

        device.inner_create_vertex_buffer(data, &vertex_attrs, vertex_step_mode)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}

pub(crate) enum IndexBufferWrapper<'a> {
    Uint16(&'a [u16]),
    Uint32(&'a [u32]),
}

pub struct IndexBufferBuilder<'a> {
    device: &'a mut Device,
    data: Option<IndexBufferWrapper<'a>>,
    format: IndexFormat,
}

impl<'a> IndexBufferBuilder<'a> {
    pub fn new(device: &'a mut Device) -> Self {
        Self {
            device,
            data: None,
            format: IndexFormat::Uint32,
        }
    }

    pub fn with_data_u16(mut self, data: &'a [u16]) -> Self {
        self.data = Some(IndexBufferWrapper::Uint16(data));
        self.format = IndexFormat::Uint16;
        self
    }

    pub fn with_data(mut self, data: &'a [u32]) -> Self {
        self.data = Some(IndexBufferWrapper::Uint32(data));
        self.format = IndexFormat::Uint32;
        self
    }

    pub fn with_format(mut self, format: IndexFormat) -> Self {
        self.format = format;
        self
    }

    pub fn build(self) -> Result<Buffer, String> {
        let Self {
            device,
            data,
            format,
        } = self;
        device.inner_create_index_buffer(data, format)
    }
}

pub struct UniformBufferBuilder<'a> {
    device: &'a mut Device,
    data: Option<Vec<u8>>,
    name: String,
    loc: u32,
}

impl<'a> UniformBufferBuilder<'a> {
    pub fn new(device: &'a mut Device, location: u32, name: &str) -> Self {
        Self {
            device,
            data: None,
            name: name.to_string(),
            loc: location,
        }
    }

    pub fn with_data<T: BufferData>(mut self, data: T) -> Self {
        let mut buffer = vec![];
        data.save_as_bytes(&mut buffer);
        self.data = Some(buffer);
        self
    }

    pub fn build(self) -> Result<Buffer, String> {
        let Self {
            device,
            data,
            name,
            loc,
        } = self;

        device.inner_create_uniform_buffer(loc, &name, data)
    }
}

#[derive(Clone, Debug, Default)]
pub struct VertexInfo {
    pub(crate) attrs: Vec<VertexAttr>,
    pub(crate) step_mode: VertexStepMode,
}

impl VertexInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attr(mut self, location: u32, format: VertexFormat) -> Self {
        self.attrs.push(VertexAttr::new(location, format));
        self
    }

    pub fn step_mode(mut self, mode: VertexStepMode) -> Self {
        self.step_mode = mode;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BufferUsage {
    Vertex,
    Index,
    Uniform(u32),
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub enum VertexStepMode {
    Vertex,
    Instance,
}

impl Default for VertexStepMode {
    fn default() -> Self {
        VertexStepMode::Vertex
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VertexFormat {
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
    UInt8,
    UInt8Norm,
    UInt8x2,
    UInt8x2Norm,
    UInt8x3,
    UInt8x3Norm,
    UInt8x4,
    UInt8x4Norm,
}

impl VertexFormat {
    pub fn size(&self) -> i32 {
        match self {
            VertexFormat::Float32 => 1,
            VertexFormat::Float32x2 => 2,
            VertexFormat::Float32x3 => 3,
            VertexFormat::Float32x4 => 4,
            VertexFormat::UInt8 => 1,
            VertexFormat::UInt8Norm => 1,
            VertexFormat::UInt8x2 => 2,
            VertexFormat::UInt8x2Norm => 2,
            VertexFormat::UInt8x3 => 3,
            VertexFormat::UInt8x3Norm => 3,
            VertexFormat::UInt8x4 => 4,
            VertexFormat::UInt8x4Norm => 4,
        }
    }

    pub fn bytes(&self) -> i32 {
        use VertexFormat::*;
        match &self {
            UInt8 | UInt8x2 | UInt8x3 | UInt8x4 => self.size(),
            UInt8Norm | UInt8x2Norm | UInt8x3Norm | UInt8x4Norm => self.size(),
            _ => self.size() * 4,
        }
    }

    pub fn normalized(&self) -> bool {
        matches!(
            self,
            VertexFormat::UInt8Norm
                | VertexFormat::UInt8x2Norm
                | VertexFormat::UInt8x3Norm
                | VertexFormat::UInt8x4Norm
        )
    }
}
