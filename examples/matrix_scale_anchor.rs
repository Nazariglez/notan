use math::glm::{scale2d, translate2d, vec2, Mat3};
use nae::prelude::*;

struct State {
    tex: Texture,
    matrix: math::Mat3,
    count: f32,
}

#[nae::main]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    let cos = state.count.cos();
    let sin = state.count.sin();
    let img = &state.tex;

    let initial_sx = 2.0;
    let initial_sy = 2.0;

    let sx = initial_sx + cos * 0.5;
    let sy = initial_sy + sin * 0.5;

    let anchor_x = img.width() * 0.5;
    let anchor_y = img.height() * 0.5;

    //Scale from the center
    let matrix_center = translate2d(&state.matrix, &vec2(400.0, 300.0));
    let matrix_scale = scale2d(&matrix_center, &vec2(sx, sy));
    let matrix_anchor = translate2d(&matrix_scale, &vec2(-anchor_x, -anchor_y));

    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.push_matrix(&matrix_anchor);
    draw.image(img, 0.0, 0.0);
    draw.pop_matrix();

    draw.end();

    state.count += 0.005;
}

fn init(app: &mut App) -> State {
    State {
        tex: app.load_file("./examples/assets/ferris.png").unwrap(),
        count: 0.0,
        matrix: math::identity(),
    }
}
