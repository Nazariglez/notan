use super::manager::DrawPipeline;
use crate::graphics::*;
use crate::pipeline::*;
use crate::shader::*;

pub fn create_draw_pipeline(
    gfx: &mut Graphics,
    typ: DrawPipeline,
    fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    unimplemented!()
}

pub fn create_draw_pipeline_from_raw(
    gfx: &mut Graphics,
    typ: DrawPipeline,
    fragment: Option<&[u8]>,
) -> Result<Pipeline, String> {
    use DrawPipeline::*;
    match typ {
        Color => create_color_pipeline(gfx, fragment),
    }
}

fn create_color_pipeline(gfx: &mut Graphics, fragment: Option<&[u8]>) -> Result<Pipeline, String> {
    unimplemented!()
}

pub(crate) struct ColorBatcher {
    pipeline: Pipeline,
}

impl ColorBatcher {
    pub fn new(backend: &mut GraphicsBackend) -> Result<Self, String> {
        unimplemented!()
    }
}
