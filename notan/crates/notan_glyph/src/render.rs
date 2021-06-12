use crate::font_vertex::FontVertex;
use notan_graphics::prelude::*;
use notan_macro::{fragment_shader, vertex_shader};

pub trait FontRender {
    fn update(&mut self, device: &mut Device, vertices: Option<&[FontVertex]>);
    fn render(&mut self, texture: &Texture, renderer: &mut Renderer);
}

//language=glsl
const TEXT_VERTEX: ShaderSource = vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec3 a_pos;
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
        gl_Position = u_projection * vec4(a_pos, 1.0);
    }
    "#
};

//language=glsl
const TEXT_FRAGMENT: ShaderSource = fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_uvs;
    layout(location = 1) in vec4 v_color;

    layout(set = 0, binding = 0) uniform sampler2D u_texture;

    layout(location = 0) out vec4 color;

    void main() {
        float alpha = texture(u_texture, v_uvs).r;
         if(alpha <= 0.0) {
             discard;
         }

        color = v_color * vec4(1.0, 1.0, 1.0, alpha);
    }
    "#
};

pub struct DefaultFontRenderer {
    pub pipeline: Pipeline,
    pub vbo: VertexBuffer,
    pub ebo: IndexBuffer,
    pub ubo: UniformBuffer,

    ebo_len: usize,
    cachedSize: (i32, i32),
}

impl DefaultFontRenderer {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let pipeline = create_font_pipeline(device, None)?;
        let vbo = device.create_vertex_buffer(vec![])?;
        let ebo = device.create_index_buffer(vec![])?;
        let ubo = device.create_uniform_buffer(0, "Locals", vec![0.0; 16])?;

        Ok(Self {
            pipeline,
            vbo,
            ebo,
            ebo_len: 0,
            ubo,
            cachedSize: (0, 0),
        })
    }
}

impl FontRender for DefaultFontRenderer {
    fn update(&mut self, device: &mut Device, vertices: Option<&[FontVertex]>) {
        let size = device.size();
        if self.cachedSize.0 != size.0 || self.cachedSize.1 != size.1 {
            let ubo_data =
                glam::Mat4::orthographic_lh(0.0, size.0 as _, size.1 as _, 0.0, -1.0, 1.0)
                    .to_cols_array();
            self.ubo.copy(&ubo_data);
            self.cachedSize = size;
        }

        if let Some(vert) = vertices {
            let (vbo_data, ebo_data): (Vec<[f32; 36]>, Vec<[u32; 6]>) = vert
                .iter()
                .enumerate()
                .map(|(i, fv)| {
                    let FontVertex {
                        pos: (x1, y1, z),
                        size: (ww, hh),
                        uvs: [u1, v1, u2, v2],
                        color: c,
                    } = *fv;

                    let x2 = x1 + ww;
                    let y2 = y1 + hh;

                    #[rustfmt::skip]
                    let vertices = [
                        x1, y1, z, u1, v1, c.r, c.g, c.b, c.a,
                        x2, y1, z, u2, v1, c.r, c.g, c.b, c.a,
                        x1, y2, z, u1, v2, c.r, c.g, c.b, c.a,
                        x2, y2, z, u2, v2, c.r, c.g, c.b, c.a,
                    ];

                    let n = (i as u32) * 4;

                    #[rustfmt::skip]
                    let indices:[u32; 6] = [
                        n + 0, n + 1, n + 2,
                        n + 2, n + 1, n + 3
                    ];

                    (vertices, indices)
                })
                .unzip();

            let vbo_data = vbo_data.concat();
            let ebo_data = ebo_data.concat();
            self.ebo_len = ebo_data.len();
            self.vbo.set(&vbo_data);
            self.ebo.set(&ebo_data);
        }
    }

    fn render(&mut self, texture: &Texture, renderer: &mut Renderer) {
        renderer.set_pipeline(&self.pipeline);
        renderer.bind_texture(0, texture);
        renderer.bind_vertex_buffer(&self.vbo);
        renderer.bind_index_buffer(&self.ebo);
        renderer.bind_uniform_buffer(&self.ubo);
        renderer.draw(0, self.ebo_len as _);
    }
}

pub fn create_font_pipeline(
    device: &mut Device,
    fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    let fragment = fragment.unwrap_or(&TEXT_FRAGMENT);
    device.create_pipeline(
        &TEXT_VERTEX,
        fragment,
        &[
            VertexAttr::new(0, VertexFormat::Float3),
            VertexAttr::new(1, VertexFormat::Float2),
            VertexAttr::new(2, VertexFormat::Float4),
        ],
        PipelineOptions {
            color_blend: Some(BlendMode::NORMAL),
            ..Default::default()
        },
    )
}
