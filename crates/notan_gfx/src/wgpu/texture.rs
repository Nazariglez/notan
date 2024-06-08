use crate::frame::NotanDrawFrame;
use crate::render_target::RenderTarget;
use crate::texture::{NotanTexture, TextureId};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use wgpu::{Texture as RawTexture, TextureView};

#[derive(Clone)]
pub struct Texture {
    pub(crate) id: TextureId,
    pub(crate) raw: Arc<RawTexture>,
    pub(crate) view: Arc<TextureView>,
    pub(crate) size: (u32, u32),
    pub(crate) write: bool,
}

impl NotanTexture for Texture {
    fn id(&self) -> TextureId {
        self.id
    }

    fn size(&self) -> (u32, u32) {
        self.size
    }

    fn width(&self) -> u32 {
        self.size.0
    }

    fn height(&self) -> u32 {
        self.size.1
    }
}

impl Debug for Texture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Texture")
            .field("id", &self.id)
            .field("size", &self.size)
            .finish()
    }
}
