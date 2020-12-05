use glow::*;
use notan_graphics::prelude::*;

pub(crate) struct InnerTexture {}

impl InnerTexture {
    pub fn new(gl: &Context, info: &TextureInfo) -> Result<Self, String> {
        Ok(Self {})
    }
}
