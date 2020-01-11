use nae::prelude::*;

struct State {
    shader: Shader,
    tex: Texture,
    count: f32,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    State {
        shader: Shader::from_image_fragment(app, include_str!("./assets/pixel.frag.glsl")).unwrap(),
        tex: app.load_file("./examples/assets/ferris.png").unwrap(),
        count: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
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
