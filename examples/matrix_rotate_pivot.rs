use math::glm::{rotate2d, scale2d, translate2d, vec2, Mat3};
use nae::prelude::*;

struct State {
    tex: Texture,
    matrix: math::Mat3,
    angle: f32,
}

#[nae::main]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    let img = &state.tex;

    let pivot_x = img.width() * 0.5;
    let pivot_y = img.height() * 0.5;

    //Rotate from the center
    let matrix_center = translate2d(&state.matrix, &vec2(400.0, 300.0));
    let matrix_scale = rotate2d(&matrix_center, state.angle);
    let matrix_pivot = translate2d(&matrix_scale, &vec2(-pivot_x, -pivot_y));

    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.push_matrix(&matrix_pivot);
    draw.image(img, 0.0, 0.0);
    draw.pop_matrix();

    draw.end();

    state.angle += 1.0 * math::PI / 180.0;
}

fn init(app: &mut App) -> State {
    State {
        tex: app.load_file("./examples/assets/ferris.png").unwrap(),
        angle: 0.0,
        matrix: math::identity(),
    }
}
