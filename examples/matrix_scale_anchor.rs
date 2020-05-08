use nae::prelude::*;
use nalgebra_glm as glm;

struct State {
    tex: nae_gfx::texture::Texture,
    matrix: glm::Mat4,
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

    let initial_sx = 2.0;
    let initial_sy = 2.0;

    let sx = initial_sx + cos * 0.5;
    let sy = initial_sy + sin * 0.5;

    let anchor_x = img.width() * 0.5;
    let anchor_y = img.height() * 0.5;

    //Scale from the center
    let matrix_center = glm::translate(&state.matrix, &glm::vec3(400.0, 300.0, 0.0));
    let matrix_scale = glm::scale(&matrix_center, &glm::vec3(sx, sy, 1.0));
    let matrix_anchor = glm::translate(&matrix_scale, &glm::vec3(-anchor_x, -anchor_y, 0.0));

    let draw = app.draw2();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.push(&slice_to_matrix4(&matrix_anchor.as_slice()));
    draw.image(img, 0.0, 0.0);
    draw.pop();

    draw.end();

    state.count += 0.005;
}

fn init(app: &mut App) -> State {
    State {
        tex: nae_gfx::texture::Texture::from_bytes(app, include_bytes!("assets/ferris.png"))
            .unwrap(),
        count: 0.0,
        matrix: math::identity(),
    }
}
