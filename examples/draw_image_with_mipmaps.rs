use notan::draw::*;
use notan::prelude::*;

//language=glsl
const FRAGMENT: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_uvs;
    layout(location = 1) in vec4 v_color;

    layout(binding = 0) uniform sampler2D u_texture;
    layout(set = 0, binding = 1) uniform TextureInfo {
        float time;
    };

    layout(location = 0) out vec4 color;

    void main() {
        float lod = (sin(time) * 0.5 + 0.5) * 5.0;
        vec2 offset = vec2(cos(time), sin(time)) * 0.2;

        color = textureLod(u_texture, v_uvs, lod);
    }
"#
};

#[derive(AppState)]
struct State {
    texture1: Texture,
    texture2: Texture,
    render_texture1: RenderTexture,
    render_texture2: RenderTexture,
    font: Font,
    pipeline: Pipeline,
    uniforms: Buffer,

    pipeline2: Pipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,

    is_first_render: bool,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();

    let ferris = include_bytes!("assets/ferris.png");

    let texture1 = gfx
        .create_texture()
        .from_image(ferris)
        .with_premultiplied_alpha()
        .build()
        .unwrap();

    let texture2 = gfx
        .create_texture()
        .from_image(ferris)
        .with_premultiplied_alpha()
        .with_filter(TextureFilter::Linear, TextureFilter::Linear)
        .with_mipmaps(true)
        .build()
        .unwrap();

    let (width, height) = texture1.size();
    let render_texture1 = gfx
        .create_render_texture(width as u32, height as u32)
        .with_format(TextureFormat::Rgba32)
        .build()
        .unwrap();

    let render_texture2 = gfx
        .create_render_texture(width as u32, height as u32)
        .with_format(TextureFormat::Rgba32)
        .with_mipmaps(true)
        .with_filter(TextureFilter::Linear, TextureFilter::Linear)
        .build()
        .unwrap();

    let pipeline = create_image_pipeline(gfx, Some(&FRAGMENT)).unwrap();
    let uniforms = gfx
        .create_uniform_buffer(1, "TextureInfo")
        .with_data(&[0.0])
        .build()
        .unwrap();

    // Pipeline2
    let vertex_info = VertexInfo::new()
        .attr(0, VertexFormat::Float32x3)
        .attr(1, VertexFormat::Float32x2);

    #[rustfmt::skip]
            let vertices = [
                //pos               //coords
                1.0,  1.0, 0.0,     1.0, 1.0,
                1.0, -1.0, 0.0,     1.0, 0.0,
                -1.0, -1.0, 0.0,    0.0, 0.0,
                -1.0,  1.0, 0.0,    0.0, 1.0
            ];

    #[rustfmt::skip]
            let indices = [
                0, 1, 3,
                1, 2, 3,
            ];

    let vertex_buffer = gfx
        .create_vertex_buffer()
        .with_info(&vertex_info)
        .with_data(&vertices)
        .build()
        .unwrap();

    let index_buffer = gfx
        .create_index_buffer()
        .with_data(&indices)
        .build()
        .unwrap();

    let pipeline2 = gfx
        .create_pipeline()
        .from(&VERT, &FRAG)
        .with_vertex_info(&vertex_info)
        .with_color_blend(BlendMode::NONE)
        .with_texture_location(0, "u_texture")
        .build()
        .unwrap();

    State {
        font,
        texture1,
        texture2,
        render_texture1,
        render_texture2,
        pipeline,
        uniforms,

        pipeline2,
        vertex_buffer,
        index_buffer,

        is_first_render: true,
    }
}

//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec4 a_position;
    layout(location = 1) in vec2 a_texcoord;
    layout(location = 0) out vec2 v_texcoord;
    void main() {
        v_texcoord = a_texcoord;
        gl_Position = vec4(a_position.x, a_position.y * -1.0, a_position.z, 1.0);
    }
    "#
};

//language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 v_texcoord;
    layout(location = 0) out vec4 outColor;
    layout(binding = 0) uniform sampler2D u_texture;

    void main() {
        outColor = texture(u_texture, v_texcoord);
    }
    "#
};

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if state.is_first_render {
        // Copy the original texture content to the RenderTextures
        {
            let mut renderer = gfx.device.create_renderer();
            renderer.begin(Some(ClearOptions::color(Color::TRANSPARENT)));

            renderer.set_pipeline(&state.pipeline2);
            renderer.bind_texture_slot(0, 0, &state.texture1);
            renderer.bind_buffers(&[&state.vertex_buffer, &state.index_buffer]);
            renderer.draw(0, 6);

            // Bind RT before the mipmap is generated
            renderer.bind_texture_slot(0, 0, &state.render_texture1);

            renderer.end();
            gfx.render_to(&state.render_texture1, &renderer);
        }
        {
            let mut renderer = gfx.device.create_renderer();
            renderer.begin(Some(ClearOptions::color(Color::TRANSPARENT)));

            renderer.set_pipeline(&state.pipeline2);
            renderer.bind_texture_slot(0, 0, &state.texture1);
            renderer.bind_buffers(&[&state.vertex_buffer, &state.index_buffer]);
            renderer.draw(0, 6);

            // Bind RT before the mipmap is generated
            renderer.bind_texture_slot(0, 0, &state.render_texture2);

            renderer.end();
            gfx.render_to(&state.render_texture2, &renderer);
        }

        state.is_first_render = false;
    }

    let mut draw = gfx.create_draw();
    draw.clear(Color::GRAY);

    let time = app.timer.elapsed_f32();
    gfx.set_buffer_data(&state.uniforms, &[time]);

    draw.image_pipeline()
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.uniforms);

    let width = 800.0 / 2.0;
    let height = 600.0 / 2.0;
    let scale = width / state.texture1.width();

    let pics = [
        (&state.texture1, "Texture"),
        (&state.texture2, "Texture with mipmap"),
        (&state.render_texture1, "RenderTexture"),
        (&state.render_texture2, "RenderTexture with mipmap"),
    ];

    for (i, (tex, label)) in pics.iter().enumerate() {
        let x = (i % 2) as f32;
        let y = (i / 2) as f32;

        draw.image(tex)
            .blend_mode(BlendMode::OVER)
            .scale(scale, scale)
            .translate(x * width, y * height + 10.0);

        draw.text(&state.font, label)
            .size(20.0)
            .position(x * width + 10.0, y * height + 10.0)
            .color(Color::WHITE);
    }

    // Show LOD as text
    let lod = (time.sin() * 0.5 + 0.5) * 5.0;
    draw.text(&state.font, &format!("LOD: {:.1}", lod))
        .size(20.0)
        .position(10.0, height * 2.0 - 30.0)
        .color(Color::WHITE);

    gfx.render(&draw);
}
