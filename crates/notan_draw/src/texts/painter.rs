use crate::batch::*;
use crate::manager::process_pipeline;
use notan_glyph::{FontVertex, GlyphPipeline, GlyphPlugin};
use notan_graphics::prelude::*;
use notan_macro::{fragment_shader, vertex_shader};
use notan_math::glam::{Mat4, Vec3};

//language=glsl
const TEXT_VERTEX: ShaderSource = vertex_shader! {
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

pub(crate) struct TextPainter {
    pub pipeline: Pipeline,
    pub vbo: Buffer,
    pub ebo: Buffer,
    pub ubo: Buffer,

    count_chars: usize,
    vertices: Vec<f32>,
    indices: Vec<u32>,
    uniforms: [f32; 16],
    count_vertices: usize,
    count_indices: usize,
    font_vertices: Vec<FontVertex>,
    dirty_buffer: bool,
}

impl TextPainter {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let uniforms = [0.0; 16];

        let pipeline = create_text_pipeline(device, None)?;
        let vbo = device.create_vertex_buffer(
            None,
            &[
                VertexAttr::new(0, VertexFormat::Float2),
                VertexAttr::new(1, VertexFormat::Float2),
                VertexAttr::new(2, VertexFormat::Float4),
            ],
            VertexStepMode::Vertex,
        )?;
        let ebo = device.create_index_buffer(None)?;
        let ubo = device.create_uniform_buffer(0, "Locals", Some(&uniforms))?;

        Ok(Self {
            pipeline,
            vbo,
            ebo,
            ubo,

            count_chars: 0,
            vertices: vec![],
            indices: vec![],
            uniforms,
            count_vertices: 0,
            count_indices: 0,
            font_vertices: vec![],
            dirty_buffer: false,
        })
    }

    pub fn push(
        &mut self,
        _device: &mut Device,
        glyphs: &mut GlyphPlugin,
        renderer: &mut Renderer,
        batch: &Batch,
        projection: &Mat4,
    ) {
        if let BatchType::Text { texts } = &batch.typ {
            process_pipeline(renderer, batch, &self.pipeline);
            let mut vertices = vec![];
            let mut indices = vec![];

            texts.iter().for_each(|d| {
                let count = d.count;
                let start = self.count_chars;
                let end = start + count;
                let vert = &self.font_vertices[start..end];
                vert.iter().enumerate().for_each(|(i, fv)| {
                    let FontVertex {
                        pos: (x1, y1, _),
                        size: (ww, hh),
                        uvs: [u1, v1, u2, v2],
                        color: c,
                    } = *fv;

                    let x2 = x1 + ww;
                    let y2 = y1 + hh;

                    let a = c.a * d.alpha;

                    let matrix = d.transform;
                    let xyz1 = matrix * Vec3::new(x1, y1, 1.0);
                    let xyz2 = matrix * Vec3::new(x2, y2, 1.0);
                    let (x1, y1, x2, y2) = (xyz1.x, xyz1.y, xyz2.x, xyz2.y);

                    #[rustfmt::skip]
                        vertices.extend_from_slice(&[
                            x1, y1, u1, v1, c.r, c.g, c.b, a,
                            x2, y1, u2, v1, c.r, c.g, c.b, a,
                            x1, y2, u1, v2, c.r, c.g, c.b, a,
                            x2, y2, u2, v2, c.r, c.g, c.b, a,
                        ]);

                    let n = ((start as u32) + (i as u32)) * 4;

                    #[rustfmt::skip]
                        indices.extend_from_slice(&[
                            n    , n + 1, n + 2,
                            n + 2, n + 1, n + 3
                        ]);
                });

                self.count_chars = end;
            });

            let offset = self.count_indices;

            self.indices.extend(&indices);
            self.count_indices = self.indices.len();

            self.vertices.extend(&vertices);
            self.count_vertices = self.vertices.len();

            self.uniforms.copy_from_slice(&projection.to_cols_array());

            renderer.bind_texture(0, &glyphs.texture);
            renderer.bind_buffers(&[&self.vbo, &self.ebo, &self.ubo]);
            renderer.draw(offset as _, indices.len() as _);
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
        self.count_chars = 0;
        self.count_vertices = 0;
        self.count_indices = 0;
        self.vertices.clear();
        self.indices.clear();
    }
}

impl GlyphPipeline for TextPainter {
    fn update(&mut self, _device: &mut Device, vertices: Option<&[FontVertex]>) {
        if let Some(vert) = vertices {
            self.font_vertices = vert.to_vec();
        }
    }

    fn render(&mut self, _texture: &Texture, _renderer: &mut Renderer) {}
}

pub fn create_text_pipeline(
    device: &mut Device,
    fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    let fragment = fragment.unwrap_or(&TEXT_FRAGMENT);
    device.create_pipeline(
        &TEXT_VERTEX,
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
