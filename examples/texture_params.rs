// This example shows how to use texture params and mipmaps.

use notan::draw::*;
use notan::prelude::*;

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 600;
const CELL_WIDTH: f32 = (WINDOW_WIDTH as f32) / 3.0;
const CELL_HEIGHT: f32 = (WINDOW_HEIGHT as f32) / 2.0;

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let x = f32::clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

//language=glsl
const FRAGMENT: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_uvs;
    layout(location = 1) in vec4 v_color;

    layout(binding = 0) uniform sampler2D u_texture;
    layout(set = 0, binding = 1) uniform TextureInfo {
        float lod;
        float offset;
        float offset_dir;
    };

    layout(location = 0) out vec4 color;

    void main() {
        vec2 uv_offset = vec2(1.0, offset_dir) * offset;
        color = textureLod(u_texture, v_uvs + uv_offset, lod);
    }
"#
};

#[derive(AppState)]
struct State {
    texture1: Texture,
    texture2: Texture,
    texture3: Texture,
    render_texture1: RenderTexture,
    render_texture2: RenderTexture,
    render_texture3: RenderTexture,
    font: Font,
    pipeline: Pipeline,
    uniforms: Buffer,

    is_first_render: bool,
}

#[notan_main]
fn main() -> Result<(), String> {
    let window_config = WindowConfig::new().set_size(WINDOW_WIDTH, WINDOW_HEIGHT);

    notan::init_with(init)
        .add_config(window_config)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();

    let ferris = include_bytes!("assets/ferris.png");

    // Texture
    let texture1 = gfx
        .create_texture()
        .from_image(ferris)
        .with_premultiplied_alpha()
        .build()
        .unwrap();

    // Texture w/ mipmap
    let texture2 = gfx
        .create_texture()
        .from_image(ferris)
        .with_premultiplied_alpha()
        .with_filter(TextureFilter::Linear, TextureFilter::Linear)
        .with_mipmaps(true)
        .build()
        .unwrap();

    // Texture w/ mipmap & wrap
    let texture3 = gfx
        .create_texture()
        .from_image(ferris)
        .with_premultiplied_alpha()
        .with_wrap(TextureWrap::Repeat, TextureWrap::Repeat)
        .with_filter(TextureFilter::Linear, TextureFilter::Linear)
        .with_mipmaps(true)
        .build()
        .unwrap();

    let (width, height) = texture1.size();

    // RenderTexture
    let render_texture1 = gfx
        .create_render_texture(width as u32, height as u32)
        .with_format(TextureFormat::Rgba32)
        .build()
        .unwrap();

    // RenderTexture w/ mipmap
    let render_texture2 = gfx
        .create_render_texture(width as u32, height as u32)
        .with_format(TextureFormat::Rgba32)
        .with_mipmaps(true)
        .with_filter(TextureFilter::Linear, TextureFilter::Linear)
        .build()
        .unwrap();

    // RenderTexture w/ mipmap & wrap
    let render_texture3 = gfx
        .create_render_texture(width as u32, height as u32)
        .with_format(TextureFormat::Rgba32)
        .with_mipmaps(true)
        .with_filter(TextureFilter::Linear, TextureFilter::Linear)
        .with_wrap(TextureWrap::Repeat, TextureWrap::Repeat)
        .build()
        .unwrap();

    let pipeline = create_image_pipeline(gfx, Some(&FRAGMENT)).unwrap();
    let uniforms = gfx
        .create_uniform_buffer(1, "TextureInfo")
        .with_data(&[0.0])
        .build()
        .unwrap();

    State {
        font,

        texture1,
        texture2,
        texture3,

        render_texture1,
        render_texture2,
        render_texture3,

        pipeline,
        uniforms,

        is_first_render: true,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if state.is_first_render {
        // Copy the original texture content to the RenderTextures
        for rt in [
            &state.render_texture1,
            &state.render_texture2,
            &state.render_texture3,
        ] {
            let mut draw = gfx.create_draw();
            draw.set_size(state.texture1.width(), state.texture1.height());
            draw.image(&state.texture1).blend_mode(BlendMode::NONE);
            gfx.render_to(rt, &draw);
        }

        state.is_first_render = false;
    }

    // Update params
    let time = app.timer.elapsed_f32();
    let lod = smoothstep(0.0, 1.0, f32::max(time.cos(), 0.0)) * 5.0;
    let offset = smoothstep(0.0, 1.0, f32::max(-time.cos(), 0.0)) * 0.5;

    // Clear canvas
    let mut draw = gfx.create_draw();
    draw.clear(Color::GRAY);
    gfx.render(&draw);

    let cells = [
        (&state.texture1, "Texture"),
        (&state.texture2, "Texture w/ mipmap"),
        (&state.texture3, "Texture w/ mipmap & wrap"),
        (&state.render_texture1, "RenderTexture"),
        (&state.render_texture2, "RenderTexture w/ mipmap"),
        (&state.render_texture3, "RenderTexture w/ mipmap & wrap"),
    ];
    let scale = CELL_WIDTH / state.texture1.width();

    // Render textures
    for (i, (tex, label)) in cells.iter().enumerate() {
        let x = (i % 3) as f32;
        let y = (i / 3) as f32;

        let offset_dir = if tex.is_render_texture() { -1.0 } else { 1.0 };
        gfx.set_buffer_data(&state.uniforms, &[lod, offset, offset_dir]);

        let mut draw = gfx.create_draw();

        draw.image_pipeline()
            .pipeline(&state.pipeline)
            .uniform_buffer(&state.uniforms);

        draw.image(tex)
            .blend_mode(BlendMode::OVER)
            .scale(scale, scale)
            .translate(x * CELL_WIDTH, y * CELL_HEIGHT + 10.0);

        draw.text(&state.font, label)
            .size(20.0)
            .position(x * CELL_WIDTH + 10.0, y * CELL_HEIGHT + 10.0)
            .color(Color::WHITE);

        gfx.render(&draw);
    }

    // Show LOD and offset as text
    let mut draw = gfx.create_draw();
    draw.text(
        &state.font,
        &format!("LOD: {:.2} / Offset: {:.2}", lod, offset),
    )
    .size(20.0)
    .position(10.0, WINDOW_HEIGHT as f32 - 30.0)
    .color(Color::WHITE);

    gfx.render(&draw);
}
