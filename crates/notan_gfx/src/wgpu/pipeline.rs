use crate::consts::MAX_BIND_GROUPS_PER_PIPELINE;
use crate::{BindGroupLayoutId, BindGroupLayoutRef, NotanRenderPipeline, PipelineId};
use arrayvec::ArrayVec;
use std::sync::Arc;
use wgpu::RenderPipeline as RawRenderPipeline;

#[derive(Clone)]
pub struct RenderPipeline {
    pub(crate) id: PipelineId,
    pub(crate) raw: Arc<RawRenderPipeline>,
    pub(crate) index_format: wgpu::IndexFormat,
    pub(crate) uses_depth: bool,
    pub(crate) uses_stencil: bool,
    pub(crate) bind_group_layout: ArrayVec<BindGroupLayoutRef, MAX_BIND_GROUPS_PER_PIPELINE>,
}

impl NotanRenderPipeline for RenderPipeline {
    fn id(&self) -> PipelineId {
        self.id
    }

    fn bind_group_layout_id(&self, index: u32) -> Result<&BindGroupLayoutRef, String> {
        self.bind_group_layout
            .get(index as usize)
            .ok_or_else(|| format!("Invalid Bind Group '{}' in pipeline", index))
    }
}
