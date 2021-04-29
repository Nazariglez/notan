mod image;
mod painter;

use crate::builder::DrawBuilder;
use crate::draw2::Draw2;
pub use image::*;
use notan_graphics::Texture;
pub(crate) use painter::ImagePainter;

pub trait DrawImages {
    fn image<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<Image<'a>>;
}

impl DrawImages for Draw2 {
    fn image<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<Image<'a>> {
        DrawBuilder::new(self, Image::new(texture))
    }
}
