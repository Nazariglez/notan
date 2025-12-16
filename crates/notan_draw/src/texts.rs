mod painter;
mod text;

use crate::builder::DrawBuilder;
use crate::draw::Draw;
pub use notan_text::{CreateFont, Font};
pub use painter::create_text_pipeline;
pub(crate) use painter::*;
pub use text::*;

#[allow(mismatched_lifetime_syntaxes)]
pub trait DrawTextSection {
    fn text<'a>(&mut self, font: &'a Font, text: &'a str) -> DrawBuilder<'_, TextSection<'a>>;
}

#[allow(mismatched_lifetime_syntaxes)]
impl DrawTextSection for Draw {
    fn text<'a>(&mut self, font: &'a Font, text: &'a str) -> DrawBuilder<'_, TextSection<'a>> {
        DrawBuilder::new(self, TextSection::new(font, text))
    }
}
