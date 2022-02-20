use crate::texture::*;
use crate::{Device, DropManager, Renderer, ResourceId};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug)]
struct RenderTextureIdRef {
    id: u64,
    drop_manager: Arc<DropManager>,
}

impl Drop for RenderTextureIdRef {
    fn drop(&mut self) {
        self.drop_manager.push(ResourceId::RenderTexture(self.id));
    }
}

#[derive(Debug, Clone)]
pub struct RenderTexture {
    id: u64,
    _id_ref: Arc<RenderTextureIdRef>,
    texture: Texture,
}

impl RenderTexture {
    pub(crate) fn new(id: u64, texture: Texture, drop_manager: Arc<DropManager>) -> Self {
        let id_ref = Arc::new(RenderTextureIdRef { id, drop_manager });

        Self {
            id,
            _id_ref: id_ref,
            texture,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> u64 {
        self.id
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

    pub fn create_renderer(&mut self) -> Renderer {
        Renderer::new(self.width() as _, self.height() as _)
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
    pub fn new(device: &'a mut Device, width: i32, height: i32) -> Self {
        let info = TextureInfo {
            width,
            height,
            ..Default::default()
        };

        Self { device, info }
    }

    /// Enable depth
    pub fn with_depth(mut self) -> Self {
        self.info.depth = true;
        self
    }

    /// Set the Texture format
    pub fn with_format(mut self, format: TextureFormat) -> Self {
        self.info.format = format;
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

        device.inner_create_render_texture(info)
    }
}
