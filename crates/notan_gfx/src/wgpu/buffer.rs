use crate::buffer::{BufferId, BufferUsage, NotanBuffer};
use std::sync::Arc;
use wgpu::Buffer as RawBuffer;

#[derive(Clone)]
pub struct Buffer {
    pub(crate) id: BufferId,
    pub(crate) raw: Arc<RawBuffer>,
    pub(crate) usage: BufferUsage,
    pub(crate) write: bool,
    pub(crate) size: usize,
}

impl NotanBuffer for Buffer {
    fn id(&self) -> BufferId {
        self.id
    }

    fn usage(&self) -> BufferUsage {
        self.usage
    }

    fn is_writable(&self) -> bool {
        self.write
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
