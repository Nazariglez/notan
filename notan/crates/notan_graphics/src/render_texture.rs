use crate::texture::*;
use crate::{DropManager, ResourceId};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug)]
struct RenderTextureId {
    id: i32,
    drop_manager: Arc<DropManager>,
}

impl Drop for RenderTextureId {
    fn drop(&mut self) {
        self.drop_manager.push(ResourceId::RenderTexture(self.id));
    }
}

#[derive(Debug, Clone)]
pub struct RenderTexture {
    id: Arc<RenderTextureId>,
    texture: Texture,
}

impl RenderTexture {
    pub(crate) fn new(id: i32, texture: Texture, drop_manager: Arc<DropManager>) -> Self {
        let id = Arc::new(RenderTextureId { id, drop_manager });

        Self { id, texture }
    }

    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.id.id
    }

    /// Returns a reference to the inner texture
    #[inline(always)]
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Consume the Render Texture and return the inner texture
    #[inline(always)]
    pub fn take_inner(self) -> Texture {
        let Self { texture, .. } = self;

        texture
    }
}

impl Deref for RenderTexture {
    type Target = Texture;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}
