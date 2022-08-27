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
        float u_size;
    };

    layout(location = 0) out vec4 color;

    void main() {
        vec2 tex_size = textureSize(u_texture, 0);
        vec2 p_size = vec2(u_size);
        vec2 coord = fract(v_uvs) * tex_size;
        coord = floor(coord/p_size) * p_size;
        color = texture(u_texture, coord / tex_size) * v_color;
    }
"#
};

#[derive(AppState)]
struct State {
    texture: Texture,
    pipeline: Pipeline,
    uniforms: Buffer,
    count: f32,
    multi: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/ferris.png"))
        .build()
        .unwrap();

    let pipeline = create_image_pipeline(gfx, Some(&FRAGMENT)).unwrap();
    let uniforms = gfx
        .create_uniform_buffer(1, "TextureInfo")
        .with_data(&[5.0])
        .build()
        .unwrap();

    State {
        texture,
        pipeline,
        uniforms,
        count: 1.0,
        multi: 1.0,
    }
}

// Change the size of the pixel effect
fn update(app: &mut App, state: &mut State) {
    if state.count > 5.0 || state.count < 0.0 {
        state.multi *= -1.0;
    }

    state.count += 0.3 * state.multi * app.timer.delta_f32();
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let pixel_size = 5.0 + state.count;
    gfx.set_buffer_data(&state.uniforms, &[pixel_size]);

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // Image without a custom shader
    draw.image(&state.texture).position(10.0, 200.0);

    // Set the custom pipeline for image
    draw.image_pipeline()
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.uniforms);

    draw.image(&state.texture)
        .position(10.0 + state.texture.width() + 40.0, 200.0);

    draw.image_pipeline().remove();

    gfx.render(&draw);
}
