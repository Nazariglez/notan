// // use crate::draw::{Batcher, Draw};
use notan_graphics::prelude::*;
use notan_macro::{fragment_shader, vertex_shader};
//
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
    device: &mut Device,
    fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    let fragment = fragment.unwrap_or_else(|| &COLOR_FRAGMENT);

    device.create_pipeline(
        &COLOR_VERTEX,
        fragment,
        &[
            VertexAttr::new(0, VertexFormat::Float2),
            VertexAttr::new(1, VertexFormat::Float3),
        ],
        PipelineOptions::default(),
    )
}
//
pub(crate) fn create_color_pipeline_from_raw(
    device: &mut Device,
    fragment: Option<&[u8]>,
) -> Result<Pipeline, String> {
    unimplemented!()
}

pub(crate) struct ColorBatcher {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    vbo: Buffer<f32>,
    ibo: Buffer<u32>,
    pipeline: Pipeline,
    clear_options: ClearOptions,
    index: usize,
}

impl ColorBatcher {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let pipeline = create_color_pipeline(device, None)?;
        Ok(Self {
            vertices: vec![],
            indices: vec![],
            vbo: device.create_vertex_buffer(vec![])?,
            ibo: device.create_index_buffer(vec![])?,
            pipeline,
            clear_options: ClearOptions::new(Color::new(0.1, 0.2, 0.3, 1.0)),
            index: 0,
        })
    }

    pub fn push<'a>(data: ColorData<'a>) {}
}

pub struct ColorData<'a> {
    vertices: &'a [f32],
    indices: &'a [u32],
    pipeline: Option<&'a Pipeline>,
}

//
// pub(crate) struct ColorBatcher {
//     pipeline: Pipeline,
//     // vertices: Vec<f32>,
//     // indices: Vec<u32>,
// }
//
// impl ColorBatcher {
//     pub fn new(pipeline: Pipeline) -> Self {
//         Self { pipeline }
//     }
// }
//
// impl Batcher for ColorBatcher {
//     fn pipeline(&mut self) -> &mut Pipeline {
//         &mut self.pipeline
//     }
// }
