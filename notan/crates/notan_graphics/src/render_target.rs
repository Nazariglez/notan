use crate::texture::*;
use crate::{DropManager, ResourceId};
use std::sync::Arc;

#[derive(Debug)]
struct RenderTargetId {
    id: i32,
    drop_manager: Arc<DropManager>,
}

impl Drop for RenderTargetId {
    fn drop(&mut self) {
        self.drop_manager.push(ResourceId::RenderTarget(self.id));
    }
}

#[derive(Debug, Clone)]
pub struct RenderTarget {
    id: Arc<RenderTargetId>,
    depth_texture: Option<Texture>,

    pub texture: Texture,
}

impl RenderTarget {
    pub(crate) fn new(
        id: i32,
        texture: Texture,
        depth_texture: Option<Texture>,
        drop_manager: Arc<DropManager>,
    ) -> Self {
        let id = Arc::new(RenderTargetId { id, drop_manager });

        Self {
            id,
            depth_texture,
            texture,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.id.id
    }

    #[inline(always)]
    pub fn width(&self) -> f32 {
        self.texture.width()
    }

    #[inline(always)]
    pub fn height(&self) -> f32 {
        self.texture.height()
    }
}
