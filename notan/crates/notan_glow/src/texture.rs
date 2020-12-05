use glow::*;

pub(crate) struct InnerTexture {}

impl InnerTexture {
    pub fn new(gl: &Context) -> Result<Self, String> {
        Ok(Self {})
    }
}
