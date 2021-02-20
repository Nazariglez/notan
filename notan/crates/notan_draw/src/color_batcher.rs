use super::draw::DrawPipeline;
use notan_graphics::prelude::*;
use notan_macro::{fragment_shader, vertex_shader};

//language=glsl
const COLOR_VERTEX: ShaderSource = vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec3 a_color;

    layout(location = 0) out vec3 v_color;

    void main() {
        v_color = a_color;
        gl_Position = vec4(a_pos - 0.5, 0.0, 1.0);
    }
    "#
};

//language=glsl
const COLOR_FRAGMENT: ShaderSource = fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec3 v_color;
    layout(location = 0) out vec4 color;

    void main() {
        color = vec4(v_color, 1.0);
    }
    "#
};

pub(crate) fn create_color_pipeline(
    gfx: &mut Graphics,
    fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    // let vertex = shader_to_bytes(gfx, &COLOR_VERTEX)?;
    let fragment = fragment.unwrap_or_else(|| &COLOR_FRAGMENT);

    gfx.create_pipeline(
        &COLOR_FRAGMENT,
        fragment,
        &[
            VertexAttr::new(0, VertexFormat::Float2),
            VertexAttr::new(1, VertexFormat::Float3),
        ],
        PipelineOptions::default(),
    )
}

pub(crate) fn create_color_pipeline_from_raw(
    gfx: &mut Graphics,
    fragment: Option<&[u8]>,
) -> Result<Pipeline, String> {
    unimplemented!()
}

pub(crate) struct ColorBatcher {
    pipeline: Pipeline,
}

impl ColorBatcher {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let pipeline = create_color_pipeline(gfx, None)?;
        Ok(Self { pipeline })
    }
}
