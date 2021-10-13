mod animation;
mod image;
mod instanced;
mod nine_slice;
mod painter;

//pub use instanced::*;
use crate::builder::DrawBuilder;
use crate::draw::Draw;
pub use animation::*;
pub use image::*;
pub use nine_slice::*;
use notan_graphics::Texture;
pub use painter::create_image_pipeline;
pub(crate) use painter::*;

pub trait DrawImages {
    fn image<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<Image<'a>>;
    fn nine_slice<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<NineSlice<'a>>;
    fn animation_grid<'a>(
        &mut self,
        texture: &'a Texture,
        cols: usize,
        rows: usize,
    ) -> DrawBuilder<ImageAnimation<'a>>;
    fn animation_list<'a>(&mut self, list: &'a [&'a Texture]) -> DrawBuilder<ImageAnimation<'a>>;
    //fn instanced_image<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<InstancedImage<'a>>;
}

impl DrawImages for Draw {
    fn image<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<Image<'a>> {
        DrawBuilder::new(self, Image::new(texture))
    }

    fn nine_slice<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<NineSlice<'a>> {
        DrawBuilder::new(self, NineSlice::new(texture))
    }

    fn animation_grid<'a>(
        &mut self,
        texture: &'a Texture,
        cols: usize,
        rows: usize,
    ) -> DrawBuilder<ImageAnimation<'a>> {
        DrawBuilder::new(self, ImageAnimation::from_grid(texture, cols, rows))
    }

    fn animation_list<'a>(&mut self, list: &'a [&'a Texture]) -> DrawBuilder<ImageAnimation<'a>> {
        DrawBuilder::new(self, ImageAnimation::from_list(list))
    }
}
