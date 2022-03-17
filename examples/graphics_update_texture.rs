use notan::draw::*;
use notan::prelude::*;

const COLORS: [Color; 6] = [
    Color::WHITE,
    Color::RED,
    Color::GREEN,
    Color::BLUE,
    Color::ORANGE,
    Color::from_rgb(0.1, 0.2, 0.3),
];

#[derive(AppState)]
struct State {
    texture: Texture,
    bytes: Vec<u8>,
    count: usize,
    color: Color,
    step: usize,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (width, height) = app.window().size();
    let bytes_per_pixel = 4;

    let len = (width * height * bytes_per_pixel) as usize;
    let bytes = vec![0; len];

    let texture = gfx
        .create_texture()
        .from_bytes(&bytes, width, height)
        .build()
        .unwrap();

    State {
        texture,
        bytes,
        count: 0,
        color: Color::WHITE,
        step: 0,
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    // update the data that will be send to the gpu
    update_bytes(state);

    // Update the texture with the new data
    gfx.update_texture(&mut state.texture)
        .with_data(&state.bytes)
        .update()
        .unwrap();

    // Draw the texture usign the draw 2d API for convenience
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.image(&state.texture);
    gfx.render(&draw);
}

fn update_bytes(state: &mut State) {
    for _ in 0..100 {
        let index = state.count * 4;
        state.bytes[index..index + 4].copy_from_slice(&state.color.rgba_u8());
        state.count += 9;

        let len = state.bytes.len() / 4;
        if state.count >= len {
            state.count -= len;
            state.step = (state.step + 1) % COLORS.len();
            state.color = COLORS[state.step];
        }
    }
}
