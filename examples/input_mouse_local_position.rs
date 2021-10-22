use notan::draw::*;
use notan::prelude::*;

#[derive(Default, AppState)]
struct State {
    rot: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::default)
        .set_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    let size = (400.0, 300.0);

    // Clone matrix
    let mut m = glam::Mat3::IDENTITY;

    // Draw the rect
    draw.rect((0.0, 0.0), size)
        .translate(200.0, 150.0)
        .rotate_from((size.0 * 0.5, size.1 * 0.5), state.rot)
        .clone_matrix_to(&mut m) // clone the rect's matrix
        .color(rect_color(app.mouse.local_position(m), size)); // calculate the color using the mouse local_position

    state.rot += app.timer.delta_f32();

    gfx.render(&draw);
}

// Set the color to red if the mouse is inside the bounds of the matrix
fn rect_color(local_position: (f32, f32), size: (f32, f32)) -> Color {
    let (local_x, local_y) = local_position;
    let (width, height) = size;

    let in_bounds = local_x >= 0.0 && local_x <= width && local_y >= 0.0 && local_y <= height;
    if in_bounds {
        Color::RED
    } else {
        Color::WHITE
    }
}
