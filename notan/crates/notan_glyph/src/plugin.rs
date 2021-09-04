use crate::{BasicPipeline, Font, GlyphManager, GlyphPipeline};
use notan_app::{Graphics, Plugin};

// TODO plugins should have the option to be created after the system init

pub struct GlyphPlugin<T: GlyphPipeline> {
    manager: GlyphManager,
    renderer: T,
}

impl<T: GlyphPipeline> GlyphPlugin<T> {
    pub fn new(gfx: &mut Graphics, renderer: T) -> Result<Self, String> {
        let manager = GlyphManager::new(gfx)?;
        Ok(Self { manager, renderer })
    }

    #[inline]
    pub fn create_font(&mut self, data: &'static [u8]) -> Result<Font, String> {
        self.manager.create_font(data)
    }
}

impl<T: 'static + GlyphPipeline> Plugin for GlyphPlugin<T> {}
