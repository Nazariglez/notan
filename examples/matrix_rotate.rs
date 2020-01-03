use nae::prelude::*;

struct State {
    tex: Texture,
    angle: f32,
}

#[nae::main]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    let img = &state.tex;

    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.push_rotation(state.angle);
    draw.image(img, 0.0, 0.0);
    draw.pop_matrix();

    draw.end();

    state.angle += 1.0 * math::PI / 180.0;
}

fn init(app: &mut App) -> State {
    app.draw().push_translate(400.0, 300.0);
    State {
        tex: app.load_file("./examples/assets/ferris.png").unwrap(),
        angle: 0.0,
    }
}
