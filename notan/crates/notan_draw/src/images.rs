mod image;
mod instanced;
mod nine_slice;
mod painter;

//pub use instanced::*;
use crate::builder::DrawBuilder;
use crate::draw2::{CustomDrawPipeline, Draw2};
pub use image::*;
pub use nine_slice::*;
use notan_graphics::prelude::*;
use notan_graphics::Texture;
pub(crate) use painter::ImagePainter;

pub trait DrawImages {
    fn set_image_pipeline(&mut self, pipeline: &Pipeline, uniforms: &[&Buffer<f32>]);
    fn remove_image_pipeline(&mut self);
    fn image_pipeline(&mut self) -> Option<&mut Pipeline>;
    fn image_uniforms(&mut self) -> Option<&mut [Buffer<f32>]>;

    fn image<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<Image<'a>>;
    fn nine_slice<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<NineSlice<'a>>;
    //fn instanced_image<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<InstancedImage<'a>>;
}

impl DrawImages for Draw2 {
    fn set_image_pipeline(&mut self, pipeline: &Pipeline, uniforms: &[&Buffer<f32>]) {
        self.image_pipeline = Some(CustomDrawPipeline {
            pipeline: pipeline.clone(),
            uniforms: if uniforms.is_empty() {
                None
            } else {
                Some(uniforms.iter().map(|u| (*u).clone()).collect::<Vec<_>>())
            },
        })
    }

    fn remove_image_pipeline(&mut self) {
        self.image_pipeline = None;
    }

    fn image_pipeline(&mut self) -> Option<&mut Pipeline> {
        self.image_pipeline.as_mut().map(|c| &mut c.pipeline)
    }

    fn image_uniforms(&mut self) -> Option<&mut [Buffer<f32>]> {
        match &mut self.image_pipeline {
            Some(pipe) => pipe.uniforms.as_mut().map(|u| u.as_mut_slice()),
            _ => None,
        }
    }

    fn image<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<Image<'a>> {
        DrawBuilder::new(self, Image::new(texture))
    }

    fn nine_slice<'a>(&mut self, texture: &'a Texture) -> DrawBuilder<NineSlice<'a>> {
        DrawBuilder::new(self, NineSlice::new(texture))
    }
}
