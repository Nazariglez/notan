use crate::texture::*;
use crate::{Device, DropManager, ResourceId};
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

pub struct RenderTextureBuilder<'a> {
    device: &'a mut Device,
    info: TextureInfo,
}

impl<'a> RenderTextureBuilder<'a> {
    pub fn new(device: &'a mut Device) -> Self {
        Self {
            device,
            info: Default::default(),
        }
    }

    /// Set the size of the texture
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.info.width = width;
        self.info.height = height;
        self
    }

    /// Enable depth
    pub fn with_depth(mut self) -> Self {
        self.info.depth = true;
        self
    }

    /// Set the Texture format
    pub fn with_format(mut self, format: TextureFormat) -> Self {
        self.info.format = format;
        self.info.internal_format = format;
        self
    }

    /// Set the Texture filter modes
    pub fn with_filter(mut self, min: TextureFilter, mag: TextureFilter) -> Self {
        self.info.min_filter = min;
        self.info.mag_filter = mag;
        self
    }

    pub fn build(self) -> Result<RenderTexture, String> {
        let Self { device, info } = self;

        device.create_render_texture(info)
    }
}
