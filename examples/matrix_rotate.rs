use nae::prelude::*;

struct State {
    tex: nae_gfx::texture::Texture,
    angle: f32,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    let img = &state.tex;

    let draw = app.draw2();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.push_rotation(state.angle);
    draw.image(img, 0.0, 0.0);
    draw.pop();

    draw.end();

    state.angle += 1.0 * math::PI / 180.0;
}

fn init(app: &mut App) -> State {
    app.draw().push_translate(400.0, 300.0);
    State {
        tex: nae_gfx::texture::Texture::from_bytes(app, include_bytes!("assets/ferris.png"))
            .unwrap(),
        angle: 0.0,
    }
}
