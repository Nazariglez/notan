use crate::{Font, Text};
use hashbrown::HashMap;
use notan_glyph::ab_glyph::FontArc;
use notan_glyph::{GlyphCalculator, GlyphCalculatorBuilder, GlyphCruncher, Section};
use notan_math::Rect;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct Calculator {
    fonts: HashMap<u64, Font>,
    dirty: bool,
    glyphs: Option<GlyphCalculator>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            dirty: false,
            glyphs: None,
        }
    }

    pub fn add_fonts(&mut self, fonts: &[Font]) {
        fonts.iter().for_each(|f| self.add_font(f));
    }

    pub fn add_font(&mut self, font: &Font) {
        let dirty = self.fonts.insert(font.id, font.clone()).is_none();
        if dirty {
            self.dirty = true;
        }
    }

    pub fn get_bounds(&mut self, sections: &[Section]) -> Rect {
        if self.dirty {
            self.dirty = false;
            self.glyphs = Some(create_calculator(&self.fonts));
        }

        // Glyphs is always present so it's safe to unwrap
        let glyphs = self.glyphs.as_mut().unwrap();
        let mut cache = glyphs.cache_scope();
        let initial_rect = Rect {
            x: f32::MAX,
            y: f32::MAX,
            width: f32::MIN_POSITIVE,
            height: f32::MIN_POSITIVE,
        };
        let rect = sections.iter().fold(initial_rect, |rect, section| {
            match cache.glyph_bounds(section) {
                None => rect,
                Some(bounds) => {
                    println!("-->");
                    dbg!(bounds);
                    println!("<--");
                    Rect {
                        x: rect.x.min(bounds.min.x),
                        y: rect.y.min(bounds.min.y),
                        width: rect.width.max(bounds.width()),
                        height: rect.height.max(bounds.height()),
                    }
                }
            }
        });

        rect
    }
}

fn create_calculator(fonts: &HashMap<u64, Font>) -> GlyphCalculator {
    let mut fonts = fonts
        .values()
        .map(|f| f.inner_ref.clone())
        .collect::<Vec<_>>();

    GlyphCalculatorBuilder::using_fonts(fonts).build()
}
