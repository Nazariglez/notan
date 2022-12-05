use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    texture: Texture,
    time: f32,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let texture = gfx
            .create_texture()
            .from_image(include_bytes!("assets/golem-walk.png"))
            .build()
            .unwrap();
        Self { texture, time: 0.0 }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // Simple animations can be defined passing a texture,
    // the number of columns and rows that it has
    // and a normalized time to calculate the current frame
    let cols = 7;
    let rows = 4;

    // frames in the first row of the image
    draw.animation_grid(&state.texture, cols, rows)
        .position(50.0, 50.0)
        // Set he frames we want, if this is not set then all frames will be used
        .frames(&[0, 1, 2, 3, 4, 5, 6])
        // Normalized time for the animation 0.0 is the start 1.0 is the last frame
        .time(state.time);

    // frame in the 2nd row of the image
    draw.animation_grid(&state.texture, cols, rows)
        .position(250.0, 180.0)
        .frames(&[7, 8, 9, 10, 11, 12, 13])
        .time(state.time);

    // frames in the 3rd row of the image
    draw.animation_grid(&state.texture, cols, rows)
        .position(450.0, 310.0)
        .frames(&[14, 15, 16, 17, 18, 19, 20])
        .time(state.time);

    // frames in the 4th row of the image
    draw.animation_grid(&state.texture, cols, rows)
        .position(650.0, 440.0)
        .frames(&[21, 22, 23, 24, 25, 26, 27])
        .time(state.time);

    gfx.render(&draw);

    state.time += app.timer.delta_f32();
}
