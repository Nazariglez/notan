use crate::draw2::DrawBatch;
use glam::Mat4;
use notan_graphics::prelude::*;
use notan_macro::{fragment_shader, vertex_shader};

//language=glsl
const IMAGE_VERTEX: ShaderSource = vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec2 a_uvs;
    layout(location = 2) in vec4 a_color;

    layout(location = 0) out vec4 v_color;
    layout(location = 1) out vec2 v_uvs;
    layout(set = 0, binding = 0) uniform Locals {
        mat4 u_projection;
    };

    void main() {
        v_color = a_color;
        v_uvs = a_uvs;
        gl_Position = u_projection * vec4(a_pos, 0.0, 1.0);
    }
    "#
};

//language=glsl
const IMAGE_FRAGMENT: ShaderSource = fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_uvs;
    layout(location = 1) in vec4 v_color;

    layout(set = 0, binding = 0) uniform sampler2D u_texture;

    layout(location = 0) out vec4 color;

    void main() {
        color = texture(u_texture, v_uvs) * v_color;
    }
    "#
};

pub(crate) struct ImagePainter {
    vbo: Buffer<f32>,
    ebo: Buffer<u32>,
    ubo: Buffer<f32>,
    pipeline: Pipeline,
    count_vertices: usize,
    count_indices: usize,
}

pub fn create_image_pipeline(
    device: &mut Device,
    fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    let fragment = fragment.unwrap_or_else(|| &IMAGE_FRAGMENT);
    device.create_pipeline(
        &IMAGE_VERTEX,
        fragment,
        &[
            VertexAttr::new(0, VertexFormat::Float2),
            VertexAttr::new(1, VertexFormat::Float2),
            VertexAttr::new(2, VertexFormat::Float4),
        ],
        PipelineOptions {
            color_blend: Some(BlendMode::NORMAL),
            ..Default::default()
        },
    )
}

impl ImagePainter {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let pipeline = create_image_pipeline(device, None)?;

        Ok(Self {
            vbo: device.create_vertex_buffer(vec![])?,
            ebo: device.create_index_buffer(vec![])?,
            ubo: device.create_uniform_buffer(0, "Locals", vec![0.0; 16])?,
            pipeline,
            count_indices: 0,
            count_vertices: 0,
        })
    }

    pub fn push(&mut self, renderer: &mut Renderer, batch: &DrawBatch, projection: &Mat4) {
        match batch {
            DrawBatch::Image {
                texture,
                pipeline,
                vertices,
                indices,
            } => {
                match pipeline {
                    Some(c) => {
                        renderer.set_pipeline(&c.pipeline);
                        if let Some(uniforms) = &c.uniforms {
                            uniforms
                                .iter()
                                .for_each(|u| renderer.bind_uniform_buffer(u));
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
                    data.extend(indices.iter().map(|i| i + len));
                    self.count_indices = data.len();
                }

                {
                    let mut data = self.vbo.data_ptr().write();
                    data.extend(vertices);
                    self.count_vertices = data.len();
                }

                {
                    self.ubo
                        .data_mut()
                        .copy_from_slice(&projection.to_cols_array());
                }

                renderer.bind_texture(0, texture);
                renderer.bind_vertex_buffer(&self.vbo);
                renderer.bind_index_buffer(&self.ebo);
                renderer.bind_uniform_buffer(&self.ubo);
                renderer.draw(offset as _, indices.len() as _);
            }
            _ => {}
        }
    }

    pub fn clear(&mut self) {
        self.count_vertices = 0;
        self.count_indices = 0;
        self.vbo.data_ptr().write().clear();
        self.ebo.data_ptr().write().clear();
        // self.ubo.data_mut().clear(); //this maybe is not necessary because it's replaced all the time
    }
}
