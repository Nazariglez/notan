use nae::prelude::*;

struct State {
    tex: Texture,
    count: f32,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    let cos = state.count.cos();
    let sin = state.count.sin();
    let img = &state.tex;

    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.push_skew(cos, sin);
    draw.image(img, 0.0, 0.0);
    draw.pop_matrix();

    draw.end();

    state.count += 0.005;
}

fn init(app: &mut App) -> State {
    app.draw().push_translate(300.0, 300.0);
    State {
        tex: app.load_file("./examples/assets/ferris.png").unwrap(),
        count: 0.0,
    }
}
