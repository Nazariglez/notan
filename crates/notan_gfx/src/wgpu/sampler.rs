use crate::texture::{NotanSampler, SamplerId};
use std::sync::Arc;
use wgpu::Sampler as RawSampler;

#[derive(Clone)]
pub struct Sampler {
    pub(crate) id: SamplerId,
    pub(crate) raw: Arc<RawSampler>,
}

impl NotanSampler for Sampler {
    fn id(&self) -> SamplerId {
        self.id
    }
}
