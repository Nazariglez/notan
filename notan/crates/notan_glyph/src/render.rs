use notan_graphics::prelude::*;
use notan_macro::{fragment_shader, vertex_shader};

pub trait FontRender {
    fn render(&mut self, texture: &mut Texture, vertices: &[f32], renderer: &mut Renderer);
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
}

impl DefaultFontRenderer {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let pipeline = create_font_pipeline(device, None)?;
        let vbo = device.create_vertex_buffer(vec![])?;
        let ebo = device.create_index_buffer(vec![])?;
        let ubo = device.create_uniform_buffer(0, "Locals", vec![])?;
        Ok(Self {
            pipeline,
            vbo,
            ebo,
            ubo,
        })
    }
}

impl FontRender for DefaultFontRenderer {
    fn render(&mut self, texture: &mut Texture, vertices: &[f32], renderer: &mut Renderer) {
        // todo
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
