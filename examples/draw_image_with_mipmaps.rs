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

    State {
        font,
        texture1,
        texture2,
        render_texture1,
        render_texture2,
        pipeline,
        uniforms,

        is_first_render: true,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if state.is_first_render {
        // Copy the original texture content to the RenderTextures
        for rt in [&state.render_texture1, &state.render_texture2] {
            let mut draw = gfx.create_draw();
            draw.set_size(state.texture1.width(), state.texture1.height());
            draw.image(&state.texture1).blend_mode(BlendMode::NONE);
            gfx.render_to(rt, &draw);
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
