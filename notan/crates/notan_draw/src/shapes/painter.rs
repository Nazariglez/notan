use crate::draw2::*;
use notan_graphics::prelude::*;
use notan_macro::{fragment_shader, vertex_shader};

//language=glsl
const SHAPES_VERTEX: ShaderSource = vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec4 a_color;

    layout(location = 0) out vec4 v_color;
    layout(set = 0, binding = 0) uniform Locals {
        mat4 u_projection;
    };

    void main() {
        v_color = a_color;
        gl_Position = u_projection * vec4(a_pos, 0.0, 1.0);
    }
    "#
};

//language=glsl
const SHAPES_FRAGMENT: ShaderSource = fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec4 v_color;
    layout(location = 0) out vec4 color;

    void main() {
        color = v_color;
    }
    "#
};

pub fn create_shape_pipeline(
    device: &mut Device,
    fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    let fragment = fragment.unwrap_or_else(|| &SHAPES_FRAGMENT);
    device.create_pipeline(
        &SHAPES_VERTEX,
        fragment,
        &[
            VertexAttr::new(0, VertexFormat::Float2),
            VertexAttr::new(1, VertexFormat::Float4),
        ],
        PipelineOptions {
            color_blend: Some(BlendMode::NORMAL),
            ..Default::default()
        },
    )
}

pub(crate) struct ShapePainter {
    vbo: Buffer<f32>,
    ebo: Buffer<u32>,
    ubo: Buffer<f32>,
    pipeline: Pipeline,
    count_vertices: usize,
    count_indices: usize,
}

impl ShapePainter {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let pipeline = create_shape_pipeline(device, None)?;

        Ok(Self {
            vbo: device.create_vertex_buffer(vec![])?,
            ebo: device.create_index_buffer(vec![])?,
            ubo: device.create_uniform_buffer(0, "Locals", vec![0.0; 16])?,
            pipeline,
            count_indices: 0,
            count_vertices: 0,
        })
    }

    pub fn push(&mut self, renderer: &mut Renderer, batch: &Batch, projection: &glam::Mat4) {
        if let BatchType::Shape = &batch.typ {
            match &batch.pipeline {
                Some(pip) => {
                    renderer.set_pipeline(&pip);
                    if let Some(buffers) = &batch.uniform_buffers {
                        buffers.iter().for_each(|u| renderer.bind_uniform_buffer(u));
                    }
                }
                _ => {
                    renderer.set_pipeline(&self.pipeline);
                }
            }

            let len = (self.count_vertices / self.pipeline.offset()) as u32;
            let offset = self.count_indices;

            {
                let mut data = self.ebo.data_ptr().write();
                data.extend(batch.indices.iter().map(|i| i + len));
                self.count_indices = data.len();
            }

            {
                let mut data = self.vbo.data_ptr().write();
                data.extend(&batch.vertices);
                self.count_vertices = data.len();
            }

            {
                self.ubo
                    .data_mut()
                    .copy_from_slice(&projection.to_cols_array());
            }

            renderer.bind_vertex_buffer(&self.vbo);
            renderer.bind_index_buffer(&self.ebo);
            renderer.bind_uniform_buffer(&self.ubo);
            renderer.draw(offset as _, batch.indices.len() as _);
        }
    }

    pub fn clear(&mut self) {
        self.count_vertices = 0;
        self.count_indices = 0;
        self.vbo.data_ptr().write().clear();
        self.ebo.data_ptr().write().clear();
    }
}
