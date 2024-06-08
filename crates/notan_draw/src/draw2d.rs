use notan_core::Plugin;
use notan_gfx::Gfx;

pub struct Draw2D {}

impl Plugin for Draw2D {}

impl Draw2D {
    pub fn new(gfx: &mut Gfx) -> Result<Self, String> {
        Ok(Self {})
    }
}
