mod painter;
mod pattern;

use crate::builder::DrawBuilder;
use crate::draw::Draw;
use notan_graphics::Texture;
pub use painter::create_pattern_pipeline;
pub(crate) use painter::*;
pub use pattern::*;

pub trait DrawPattern {
    fn pattern<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<Pattern<'a>>;
}

impl DrawPattern for Draw {
    fn pattern<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<Pattern<'a>> {
        DrawBuilder::new(self, Pattern::new(texture))
    }
}
