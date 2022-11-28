use crate::FONTS;
use notan_glyph::{GlyphCalculator, GlyphCalculatorBuilder, GlyphCruncher, Section};
use notan_math::{vec2, Rect, Vec2};

/// Calculate the bounds of a glyph section
#[derive(Default, Debug)]
pub struct Calculator {
    fonts: usize,
    glyphs: Option<GlyphCalculator>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            fonts: 0,
            glyphs: None,
        }
    }

    fn create_calculator(&mut self) {
        if let Some((length, calculator)) = generate_calculator_if_necessary(self.fonts) {
            self.fonts = length;
            self.glyphs = Some(calculator);
        }
    }

    /// Returns the bound of the section passed
    pub fn bounds(&mut self, section: &Section) -> Rect {
        self.create_calculator();

        // Glyphs is always present so it's safe to unwrap
        let glyphs = self.glyphs.as_mut().unwrap();
        let mut cache = glyphs.cache_scope();

        match cache.glyph_bounds(section) {
            None => Rect::default(),
            Some(bounds) => Rect {
                x: bounds.min.x,
                y: bounds.min.y,
                width: bounds.width(),
                height: bounds.height(),
            },
        }
    }

    /// Returns the bounds of the all sections mixed
    pub fn mixed_bounds(&mut self, sections: &[Section]) -> Rect {
        self.create_calculator();

        // Glyphs is always present so it's safe to unwrap
        let glyphs = self.glyphs.as_mut().unwrap();
        let mut cache = glyphs.cache_scope();

        // iterate over sections to get the min position and the max size
        let rect = sections.iter().fold(MinMax::default(), |mm, section| {
            match cache.glyph_bounds(section) {
                None => mm,
                Some(bounds) => MinMax {
                    min: vec2(mm.min.x.min(bounds.min.x), mm.min.y.min(bounds.min.y)),
                    max: vec2(mm.max.x.max(bounds.max.x), mm.max.y.max(bounds.max.y)),
                },
            }
        });

        rect.into()
    }
}

struct MinMax {
    min: Vec2,
    max: Vec2,
}

impl Default for MinMax {
    fn default() -> Self {
        Self {
            min: Vec2::splat(f32::MAX),
            max: Vec2::splat(f32::MIN),
        }
    }
}

impl From<MinMax> for Rect {
    fn from(mm: MinMax) -> Self {
        Rect {
            x: mm.min.x,
            y: mm.min.y,
            width: mm.max.x - mm.min.x,
            height: mm.max.y - mm.min.y,
        }
    }
}

fn generate_calculator_if_necessary(length: usize) -> Option<(usize, GlyphCalculator)> {
    let fonts = FONTS.read();
    let dirty = length != fonts.len();

    if !dirty {
        return None;
    }

    let calculator = GlyphCalculatorBuilder::using_fonts(fonts.to_vec()).build();
    Some((fonts.len(), calculator))
}
