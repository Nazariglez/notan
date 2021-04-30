use crate::draw2::DrawBatch;
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

pub(crate) struct ShapePainter {
    vbo: Buffer<f32>,
    ebo: Buffer<u32>,
    ubo: Buffer<f32>,
    pipeline: Pipeline,
    clear_options: ClearOptions,
}

impl ShapePainter {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let pipeline = device.create_pipeline(
            &SHAPES_VERTEX,
            &SHAPES_FRAGMENT,
            &[
                VertexAttr::new(0, VertexFormat::Float2),
                VertexAttr::new(1, VertexFormat::Float4),
            ],
            PipelineOptions {
                color_blend: Some(BlendMode::NORMAL),
                ..Default::default()
            },
        )?;

        Ok(Self {
            vbo: device.create_vertex_buffer(vec![])?,
            ebo: device.create_index_buffer(vec![])?,
            ubo: device.create_uniform_buffer(0, vec![0.0; 16])?,
            pipeline,
            clear_options: ClearOptions::new(Color::new(0.1, 0.2, 0.3, 1.0)),
        })
    }

    pub fn push(&mut self, renderer: &mut Renderer, batch: &DrawBatch, projection: &glam::Mat4) {
        match batch {
            DrawBatch::Shape {
                pipeline,
                vertices,
                indices,
            } => {
                renderer.set_pipeline(&self.pipeline);
                {
                    let mut data = self.vbo.data_ptr().write();
                    data.clear();
                    data.extend(vertices);
                }
                {
                    let mut data = self.ebo.data_ptr().write();
                    data.clear();
                    data.extend(indices);
                }
                {
                    self.ubo
                        .data_mut()
                        .copy_from_slice(&projection.to_cols_array());
                }

                renderer.bind_vertex_buffer(&self.vbo);
                renderer.bind_index_buffer(&self.ebo);
                renderer.bind_uniform_buffer(&self.ubo);
                renderer.draw(0, indices.len() as i32);
            }
            _ => {}
        }
    }
}
