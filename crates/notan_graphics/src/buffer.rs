use crate::device::{DropManager, ResourceId};
use crate::pipeline::*;
use crate::Device;
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

    #[inline(always)]
    pub fn id(&self) -> u64 {
        self.id
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

pub struct IndexBufferBuilder<'a> {
    device: &'a mut Device,
    data: Option<&'a [u32]>,
}

impl<'a> IndexBufferBuilder<'a> {
    pub fn new(device: &'a mut Device) -> Self {
        Self { device, data: None }
    }

    pub fn with_data(mut self, data: &'a [u32]) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> Result<Buffer, String> {
        let Self { device, data } = self;

        device.inner_create_index_buffer(data)
    }
}

pub struct UniformBufferBuilder<'a> {
    device: &'a mut Device,
    data: Option<&'a [f32]>,
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

    pub fn with_data(mut self, data: &'a [f32]) -> Self {
        self.data = Some(data);
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
