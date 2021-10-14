mod painter;
mod text;

use crate::builder::DrawBuilder;
use crate::draw::Draw;
pub use notan_glyph::Font;
pub use painter::create_text_pipeline;
pub(crate) use painter::*;
pub use text::*;

pub trait DrawTextSection {
    fn text<'a>(&mut self, font: &'a Font, text: &'a str) -> DrawBuilder<TextSection<'a>>;
}

impl DrawTextSection for Draw {
    fn text<'a>(&mut self, font: &'a Font, text: &'a str) -> DrawBuilder<TextSection<'a>> {
        DrawBuilder::new(self, TextSection::new(font, text))
    }
}
