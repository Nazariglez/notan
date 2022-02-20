use crate::batch::*;
use crate::manager::process_pipeline;
use notan_graphics::prelude::*;
use notan_macro::{fragment_shader, vertex_shader};
use notan_math::glam::Mat4;

fn vertex_info() -> VertexInfo {
    VertexInfo::new()
        .attr(0, VertexFormat::Float2)
        .attr(1, VertexFormat::Float2)
        .attr(2, VertexFormat::Float4)
}

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

pub fn create_image_pipeline(
    device: &mut Device,
    fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    let fragment = fragment.unwrap_or(&IMAGE_FRAGMENT);
    device
        .create_pipeline()
        .from(&IMAGE_VERTEX, fragment)
        .with_vertex_info(&vertex_info())
        .with_color_blend(BlendMode::NORMAL)
        .build()
}

pub(crate) struct ImagePainter {
    vbo: Buffer,
    ebo: Buffer,
    ubo: Buffer,
    pipeline: Pipeline,
    vertices: Vec<f32>,
    indices: Vec<u32>,
    uniforms: [f32; 16],
    count_vertices: usize,
    count_indices: usize,
    dirty_buffer: bool,
}

impl ImagePainter {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let pipeline = create_image_pipeline(device, None)?;

        let uniforms = [0.0; 16];
        let vbo = device
            .create_vertex_buffer()
            .with_info(&vertex_info())
            .build()?;

        let ebo = device.create_index_buffer().build()?;

        let ubo = device
            .create_uniform_buffer(0, "Locals")
            .with_data(&uniforms)
            .build()?;

        Ok(Self {
            vbo,
            ebo,
            ubo,
            pipeline,
            vertices: vec![],
            indices: vec![],
            uniforms,
            count_indices: 0,
            count_vertices: 0,
            dirty_buffer: false,
        })
    }

    pub fn push(&mut self, renderer: &mut Renderer, batch: &Batch, projection: &Mat4) {
        if let BatchType::Image { texture } = &batch.typ {
            process_pipeline(renderer, batch, &self.pipeline);

            let len = (self.count_vertices / self.pipeline.offset()) as u32;
            let offset = self.count_indices;

            self.indices.extend(batch.indices.iter().map(|i| i + len));
            self.count_indices = self.indices.len();

            self.vertices.extend(&batch.vertices);
            self.count_vertices = self.vertices.len();

            self.uniforms.copy_from_slice(&projection.to_cols_array());

            renderer.bind_texture(0, texture);
            renderer.bind_buffers(&[&self.vbo, &self.ebo, &self.ubo]);
            renderer.draw(offset as _, batch.indices.len() as _);

            self.dirty_buffer = true;
        }
    }

    #[inline]
    pub fn upload_buffers(&mut self, device: &mut Device) {
        if self.dirty_buffer {
            self.dirty_buffer = false;
            device.set_buffer_data(&self.vbo, &self.vertices);
            device.set_buffer_data(&self.ebo, &self.indices);
            device.set_buffer_data(&self.ubo, &self.uniforms);
        }
    }

    pub fn clear(&mut self) {
        self.count_vertices = 0;
        self.count_indices = 0;
        self.vertices.clear();
        self.indices.clear();
    }
}
