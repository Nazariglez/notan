use super::color_batcher::*;
use glam::Mat4;
use notan_graphics::prelude::*;

pub(crate) struct DrawManager {
    color_batcher: ColorBatcher,
}

impl DrawManager {
    pub fn new(gfx: &mut Device) -> Result<Self, String> {
        let color_batcher = ColorBatcher::new(gfx)?;

        Ok(Self { color_batcher })
    }
}
