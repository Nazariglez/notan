use notan::prelude::*;

//language=glsl
const FRAGMENT: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_uvs;
    layout(location = 1) in vec4 v_color;

    layout(set = 0, binding = 0) uniform sampler2D u_texture;
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

#[derive(notan::AppState)]
struct State {
    img: Texture,
    pipeline: Pipeline,
    uniforms: Buffer<f32>,
    count: f32,
    multi: f32,
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(init).update(update).draw(draw).build()
}

fn init(gfx: &mut Graphics) -> State {
    let img = TextureInfo::from_image(include_bytes!("assets/ferris.png")).unwrap();
    let texture = gfx.create_texture(img).unwrap();
    State {
        img: texture,
        pipeline: gfx.create_draw_image_pipeline(Some(&FRAGMENT)).unwrap(),
        uniforms: gfx
            .create_uniform_buffer(1, "TextureInfo", vec![5.0])
            .unwrap(),
        count: 1.0,
        multi: 1.0,
    }
}

// Change the size of the pixel effect
fn update(app: &mut App, state: &mut State) {
    let pixel_size = 5.0 + state.count;

    {
        let mut data = state.uniforms.data_mut();
        data[0] = pixel_size;
    }

    if state.count > 5.0 || state.count < 0.0 {
        state.multi *= -1.0;
    }

    state.count += 0.3 * state.multi * app.timer.delta_f32();
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // Image without a custom shader
    draw.image(&state.img).position(10.0, 200.0);

    // Set the custom pipeline for imagec
    draw.image_pipeline()
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.uniforms);

    draw.image(&state.img)
        .position(10.0 + state.img.width() + 40.0, 200.0);

    draw.image_pipeline().remove();

    gfx.render(&draw);
}
