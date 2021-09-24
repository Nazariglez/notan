use crate::text::{section_from_text, Text};
use glyph_brush::{ab_glyph::*, *};
use std::sync::Arc;

/// Represent a loaded font on memory
#[derive(Debug, Clone)]
pub struct Font {
    pub(crate) id: FontId,
    pub(crate) glyphs: Arc<GlyphCalculator<FontRef<'static>, Extra>>,
}

impl Font {
    /// Font id
    pub fn id(&self) -> i32 {
        self.id.0 as _
    }

    /// Returns the width and the height of a custom text
    pub fn text_size(&self, text: &Text) -> (f32, f32) {
        let section = section_from_text(self, text);
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
