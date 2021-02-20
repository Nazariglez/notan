use super::color_batcher::*;
use glam::Mat4;
use notan_graphics::prelude::*;

// TODO cargo make

pub struct Draw<'a> {
    renderer: Renderer<'a>,
}

impl<'a> Draw<'a> {
    pub fn new(renderer: Renderer<'a>) -> Self {
        Self { renderer }
    }
}

#[inline]
pub fn create_draw_pipeline(
    gfx: &mut Graphics,
    typ: DrawPipeline,
    fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    use DrawPipeline::*;
    match typ {
        Color => create_color_pipeline(gfx, fragment),
    }
}

#[inline]
pub fn create_draw_pipeline_from_raw(
    gfx: &mut Graphics,
    typ: DrawPipeline,
    fragment: Option<&[u8]>,
) -> Result<Pipeline, String> {
    use DrawPipeline::*;
    match typ {
        Color => create_color_pipeline_from_raw(gfx, fragment),
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DrawPipeline {
    Color,
    //Image,
    //Text,
}
