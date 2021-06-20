use crate::text::{section_from_text, Text};
use glyph_brush::{ab_glyph::*, *};
use notan_graphics::prelude::*;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Font {
    pub(crate) id: FontId,
    pub(crate) glyphs: Arc<GlyphCalculator<FontRef<'static>, Extra>>,
}

impl Font {
    pub fn id(&self) -> i32 {
        self.id.0 as _
    }

    pub fn text_size(&self, text: &Text) -> (f32, f32) {
        let section = section_from_text(&self, text);
        let mut cache = self.glyphs.cache_scope();
        if let Some(bounds) = cache.glyph_bounds(section) {
            return (bounds.width(), bounds.height());
        }

        (0.0, 0.0)
    }
}

impl PartialEq for Font {
    fn eq(&self, other: &Font) -> bool {
        self.id == other.id
    }
}
