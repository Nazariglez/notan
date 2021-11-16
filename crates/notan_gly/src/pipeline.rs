use crate::cache::Cache;
use crate::instance::Instance;
use notan_app::graphics::*;
use notan_math::glam::{Mat3, Mat4};
use notan_math::Rect;

// TODO CHECK THIS https://github.com/hecrj/glow_glyph/blob/master/src/pipeline.rs
// TODO CHECK THIS https://github.com/hecrj/wgpu_glyph/blob/master/src/pipeline.rs

//language=glsl
const GLYPH_VERTEX: ShaderSource = vertex_shader! {
    r#"
    #version 450

    layout(set = 0, binding = 0) uniform Locals {
        mat4 transform;
    };

    layout(location = 0) in vec3 left_top;
    layout(location = 1) in vec2 right_bottom;
    layout(location = 2) in vec2 tex_left_top;
    layout(location = 3) in vec2 tex_right_bottom;
    layout(location = 4) in vec4 color;

    layout(location = 0) out vec2 f_tex_pos;
    layout(location = 1) out vec4 f_color;

    // generate positional data based on vertex ID
    void main() {
        vec2 pos = vec2(0.0);
        float left = left_top.x;
        float right = right_bottom.x;
        float top = left_top.y;
        float bottom = right_bottom.y;

        switch (gl_VertexIndex) {
            case 0:
                pos = vec2(left, top);
                f_tex_pos = tex_left_top;
                break;

            case 1:
                pos = vec2(right, top);
                f_tex_pos = vec2(tex_right_bottom.x, tex_left_top.y);
                break;

            case 2:
                pos = vec2(left, bottom);
                f_tex_pos = vec2(tex_left_top.x, tex_right_bottom.y);
                break;

            case 3:
                pos = vec2(right, bottom);
                f_tex_pos = tex_right_bottom;
                break;
        }

        f_color = color;
        gl_Position = transform * vec4(pos, left_top.z, 1.0);
    }
    "#
};

//language=glsl
const GLYPH_FRAGMENT: ShaderSource = fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(set = 0, binding = 0) uniform sampler2D font_sampler;

    layout(location = 0) in vec2 f_tex_pos;
    layout(location = 1) in vec4 f_color;

    layout(location = 0) out vec4 Target0;

    void main() {
        float alpha = texture(font_sampler, f_tex_pos).r;

        if (alpha <= 0.0) {
            discard;
        }

        Target0 = f_color * vec4(1.0, 1.0, 1.0, alpha);
    }
    "#
};

pub struct GlyPipeline {
    pub pipeline: Pipeline,
    pub vbo: Buffer,
    pub ebo: Buffer,
    pub ubo: Buffer,
    cache: Cache,
    current_instances: usize,
    supported_instances: usize,
    current_transform: Mat4,
}

impl GlyPipeline {
    pub fn new(
        gfx: &mut Graphics,
        texture_width: u32,
        texture_height: u32,
    ) -> Result<Self, String> {
        let cache = Cache::new(gfx, texture_width, texture_height)?;
        let vertex_info = VertexInfo::new()
            .attr(0, VertexFormat::Float3)
            .attr(1, VertexFormat::Float2)
            .attr(2, VertexFormat::Float2)
            .attr(3, VertexFormat::Float2)
            .attr(4, VertexFormat::Float4)
            .step_mode(VertexStepMode::Instance);

        let pipeline = create_pipeline(gfx, &vertex_info)?;
        let vbo = gfx.create_vertex_buffer().with_info(&vertex_info).build()?;
        let ebo = gfx.create_index_buffer().build()?;
        let ubo = gfx
            .create_uniform_buffer(0, "Locals")
            .with_data(&[0.0; 16])
            .build()?;

        Ok(Self {
            pipeline,
            vbo,
            ebo,
            ubo,
            cache,
            current_instances: 0,
            supported_instances: 50000,
            current_transform: Mat4::IDENTITY,
        })
    }

    pub fn draw(&mut self, gfx: &mut Graphics, transform: Mat4, region: Option<Rect>) {
        if self.current_transform != transform {
            gfx.set_buffer_data(&self.ubo, &transform.to_cols_array());
        }

        let mut renderer = gfx.create_renderer();
        renderer.set_primitive(DrawPrimitive::TriangleStrip);

        if let Some(region) = region {
            renderer.set_scissors(region.x, region.y, region.width, region.height);
        }

        renderer.begin(Some(&ClearOptions::new(Color::BLACK))); // TODO clear should be public to be managed by the user
        renderer.set_pipeline(&self.pipeline);
        renderer.bind_texture(0, self.cache.texture());
        renderer.bind_buffers(&[&self.vbo, &self.ubo]);
        // TODO https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawElementsInstanced
        // TODO https://github.com/Kode/Kha/blob/0296e6d576332eacafa923a9de2b6354a39a2f9b/Backends/HTML5/kha/js/graphics4/Graphics.hx#L692
        // TODO https://github.com/hecrj/wgpu_glyph/blob/master/src/pipeline.rs#L403
        // TODO https://github.com/hecrj/glow_glyph/blob/master/src/pipeline.rs#L118-L123
        renderer.draw_instanced(0, 4, self.current_instances as _);
        renderer.end();

        // gfx.render and render_to?

        gfx.render(&renderer);
    }

    pub fn increase_cache_size(&mut self, gfx: &mut Graphics, width: u32, height: u32) {
        self.cache = Cache::new(gfx, width, height).unwrap();
    }

    pub fn update_cache(
        &mut self,
        gfx: &mut Graphics,
        offset: [u16; 2],
        size: [u16; 2],
        data: &[u8],
    ) -> Result<(), String> {
        self.cache.update(gfx, offset, size, data)
    }

    // TODO https://github.com/hecrj/glow_glyph/blob/master/src/pipeline.rs#L157
    pub fn upload(&mut self, gfx: &mut Graphics, instances: &[Instance]) {
        if instances.is_empty() {
            self.current_instances = 0;
            return;
        }

        let data: &[f32] = bytemuck::cast_slice(instances);
        gfx.set_buffer_data(&self.vbo, data);
        self.current_instances = instances.len();
    }
}

fn create_pipeline(gfx: &mut Graphics, info: &VertexInfo) -> Result<Pipeline, String> {
    gfx.create_pipeline()
        .from(&GLYPH_VERTEX, &GLYPH_FRAGMENT)
        .with_vertex_info(&info)
        .with_color_blend(BlendMode::NORMAL)
        .with_alpha_blend(BlendMode {
            src: BlendFactor::One,
            dst: BlendFactor::InverseSourceAlpha,
            op: BlendOperation::Add,
        })
        // TODO depth stencil and culling
        .build()
}
