use notan::draw::*;
use notan::math::{Mat3, Vec2, DEG_TO_RAD};
use notan::prelude::*;

const COLORS: [Color; 8] = [
    Color::WHITE,
    Color::MAGENTA,
    Color::ORANGE,
    Color::RED,
    Color::YELLOW,
    Color::AQUA,
    Color::MAROON,
    Color::PINK,
];

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

    // Push to the transformation stack a translation matrix
    draw.transform()
        .push(Mat3::from_translation(Vec2::new(350.0, 250.0)));

    // Calculate the matrix that we use for each object
    let translation = Mat3::from_translation(Vec2::new(30.0, 20.0));
    let rotation = Mat3::from_angle(state.rot * DEG_TO_RAD);
    let matrix = translation * rotation;

    for (i, c) in COLORS.iter().enumerate() {
        let n = (i * 7) as f32;
        let size = 100.0 - n;

        // Push to the stack the same matrix on each draw
        // The new matrices will be multiplied by the latest on the stack
        draw.transform().push(matrix);

        // Create a rect
        draw.rect((0.0, 0.0), (size, size))
            // Using different color for each rect
            .color(*c);
    }

    // Reset the transformation stack
    draw.transform().clear();

    // Render the frame
    gfx.render(&draw);

    state.rot = (state.rot + app.timer.delta_f32() * 25.0) % 360.0;
}
