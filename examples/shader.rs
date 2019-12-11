use nae::prelude::*;

const FRAG: &'static str = r#"#version 300 es
precision mediump float;

out vec4 outColor;
in vec2 v_texcoord;
uniform sampler2D u_texture;

uniform vec2 u_tex_size;
uniform vec2 u_size;

void main() {
    vec2 coord = v_texcoord * u_tex_size;
    coord = floor(coord/u_size) * u_size;
    outColor = texture(u_texture, coord / u_tex_size);
}
"#;

struct State {
    shader: shader::Shader,
    tex: Texture,
    count: f32,
}

#[nae_start]
fn main() {
    if let Err(e) = nae::with_state(init).draw(draw).build() {
        log(&e);
    }
}

fn init(app: &mut App) -> State {
    State {
        shader: shader::Shader::from_image_fragment(app, FRAG).unwrap(),
        tex: app.load_file("../assets/ferris.png").unwrap(),
        count: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    if !state.tex.is_loaded() {
        return;
    }

    let image = &state.tex;
    let shader = &state.shader;
    let size = 5.0 + state.count.sin();

    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.image(image, 20.0, 150.0);

    draw.set_shader(Some(shader));
    shader.set_uniform("u_size", &[size, size]);
    shader.set_uniform("u_tex_size", &[image.width(), image.height()]);
    draw.image(image, 400.0, 150.0);
    draw.set_shader(None);

    draw.end();

    state.count += 0.005;
}
