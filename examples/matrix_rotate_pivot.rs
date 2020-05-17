use nae::prelude::*;
use nalgebra_glm as glm;

struct State {
    tex: Texture,
    matrix: glm::Mat4,
    angle: f32,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    let img = &state.tex;

    let pivot_x = img.width() * 0.5;
    let pivot_y = img.height() * 0.5;

    //Rotate from the center
    let matrix_center = glm::translate(&state.matrix, &glm::vec3(400.0, 300.0, 0.0));
    let matrix_scale = glm::rotate_z(&matrix_center, state.angle);
    let matrix_pivot = glm::translate(&matrix_scale, &glm::vec3(-pivot_x, -pivot_y, 0.0));

    //Draw the sprite on the center of the screen while it rotate
    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.push(&slice_to_matrix4(&matrix_pivot.as_slice()));
    draw.image(img, 0.0, 0.0);
    draw.pop();

    draw.end();

    state.angle += 1.0 * math::PI / 180.0;
}

fn init(app: &mut App) -> State {
    State {
        tex: Texture::from_bytes(app, include_bytes!("assets/ferris.png")).unwrap(),
        angle: 0.0,
        matrix: glm::identity(),
    }
}
