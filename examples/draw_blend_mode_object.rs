use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    texture: Texture,
    font: Font,
    count: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();

    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/ferris.png"))
        .build()
        .unwrap();

    State {
        font,
        texture,
        count: 0.0,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // background with different color
    let colors = [
        Color::GREEN,
        Color::BLUE,
        Color::WHITE,
        Color::RED,
        Color::YELLOW,
        Color::AQUA,
    ];
    colors.iter().enumerate().for_each(|(i, color)| {
        let height = (600 / colors.len()) as f32;
        let yy = height * (i as f32);
        draw.rect((0.0, yy), (800.0, height)).color(*color);
    });

    // a few blend modes
    #[rustfmt::skip]
    let modes = [
        ("Normal", BlendMode::NORMAL),
        ("Add", BlendMode::ADD),
        ("Erase", BlendMode::ERASE),
        ("Screen", BlendMode::SCREEN),
        ("Multiply", BlendMode::MULTIPLY),
    ];

    modes.iter().enumerate().for_each(|(i, (name, mode))| {
        let width = (800 / modes.len()) as f32;
        let xx = width * (i as f32);

        let scale = width / state.texture.width();
        let yy = 300.0 - (i as f32 * 20.0) + state.count.sin() * 300.0;

        // Draw image with a custom blend mode
        draw.image(&state.texture)
            .translate(xx, yy)
            .scale(scale, scale)
            .blend_mode(*mode);

        // print names
        draw.text(&state.font, name)
            .size(20.0)
            .position(xx + 10.0, 10.0)
            .color(Color::BLACK);

        state.count += 0.05 * app.timer.delta_f32();
    });

    gfx.render(&draw);
}
