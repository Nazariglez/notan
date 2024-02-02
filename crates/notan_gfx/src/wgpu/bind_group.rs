use crate::{BindGroupId, BindGroupLayoutId, NotanBindGroup, NotanBindGroupLayoutRef};
use std::sync::Arc;
use wgpu::{BindGroup as RawBindGroup, BindGroupLayout};

#[derive(Clone)]
pub struct BindGroup {
    pub(crate) id: BindGroupId,
    pub(crate) raw: Arc<RawBindGroup>,
}

impl NotanBindGroup for BindGroup {
    fn id(&self) -> BindGroupId {
        self.id
    }
}

#[derive(Clone)]
pub struct BindGroupLayoutRef {
    pub(crate) id: BindGroupLayoutId,
    pub(crate) raw: Arc<BindGroupLayout>,
}

impl NotanBindGroupLayoutRef for BindGroupLayoutRef {
    fn id(&self) -> BindGroupLayoutId {
        self.id
    }
}
