use notan::draw::*;
use notan::math::{Mat3, Vec2, DEG_TO_RAD};
use notan::prelude::*;

#[derive(AppState, Default)]
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

    let n = state.rot * 0.1;

    draw.rect((0.0, 0.0), (100.0, 100.0))
        // Use a helper function to translate the matrix
        .translate(110.0 + n.sin() * 100.0, 10.0);

    draw.rect((0.0, 0.0), (100.0, 100.0))
        .color(Color::AQUA)
        // Matrix translation
        .translate(220.0, 220.0)
        // Helper to pivot from a point using degrees
        .rotate_degrees_from((50.0, 50.0), state.rot);

    draw.circle(20.0)
        .color(Color::ORANGE)
        // Matrix translation
        .translate(500.0, 320.0)
        // Helper to scale from a point
        .scale_from((0.0, 0.0), (2.0 + n.sin(), 2.0 + n.cos()));

    // Create a matrix that we can set to the next paint
    let translation = Mat3::from_translation(Vec2::new(200.0, 400.0));
    let rotation = Mat3::from_angle(state.rot * 0.5 * DEG_TO_RAD);
    let matrix = translation * rotation;

    draw.rect((0.0, 0.0), (100.0, 100.0))
        .color(Color::MAGENTA)
        // Set directly a matrix for this object
        .transform(matrix);

    // Render the frame
    gfx.render(&draw);

    state.rot = state.rot + app.timer.delta_f32() * 25.0;
}
