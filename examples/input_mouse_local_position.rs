use notan::draw::*;
use notan::math::Vec2;
use notan::prelude::*;
use notan_math::{vec2, Mat3, Mat4};

#[derive(Default, AppState)]
struct State {
    rot: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::default)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    {
        // rectangle's size
        let size = (400.0, 300.0);

        // get the draw builder
        let mut rect = draw.rect((0.0, 0.0), size);

        // set the rectangle's transformation
        rect.translate(200.0, 150.0)
            .rotate_from((size.0 * 0.5, size.1 * 0.5), state.rot);

        // get the local position based on the current projection + matrix
        let local = rect.local_position(app.mouse.x, app.mouse.y);

        // if it's in bound set color red otherwise white
        let color = rect_color(local, size);
        rect.color(color);
    }

    // rotate the rectangle
    state.rot += app.timer.delta_f32();

    gfx.render(&draw);
}

// Set the color to red if the mouse is inside the bounds of the matrix
fn rect_color(local: Vec2, size: (f32, f32)) -> Color {
    let (width, height) = size;

    let in_bounds = local.x >= 0.0 && local.x <= width && local.y >= 0.0 && local.y <= height;
    if in_bounds {
        Color::RED
    } else {
        Color::WHITE
    }
}
