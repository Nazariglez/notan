mod image;
mod instanced;
mod nine_slice;
mod painter;

//pub use instanced::*;
use crate::builder::DrawBuilder;
use crate::draw2::Draw2;
pub use image::*;
pub use nine_slice::*;
use notan_graphics::Texture;
pub(crate) use painter::ImagePainter;

pub trait DrawImages {
    fn image<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<Image<'a>>;
    fn nine_slice<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<NineSlice<'a>>;
    //fn instanced_image<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<InstancedImage<'a>>;
}

impl DrawImages for Draw2 {
    fn image<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<Image<'a>> {
        DrawBuilder::new(self, Image::new(texture))
    }

    fn nine_slice<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<NineSlice<'a>> {
        DrawBuilder::new(self, NineSlice::new(texture))
    }
}
