use notan::draw::*;
use notan::prelude::*;
use std::ops::Rem;

const COLORS: [Color; 8] = [
    Color::RED,
    Color::ORANGE,
    Color::OLIVE,
    Color::BLUE,
    Color::GREEN,
    Color::SILVER,
    Color::PURPLE,
    Color::PINK,
];

#[derive(AppState)]
struct State {
    font: Font,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();

    State { font }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    app.touch.down.iter().for_each(|(&index, _)| {
        if let Some((x, y)) = app.touch.position(index) {
            draw.circle(12.0)
                .position(x, y)
                .color(COLORS[(index as usize).rem(COLORS.len())]);

            draw.text(&state.font, &format!("{index} -> {x:.0}x{y:.0}"))
                .position(x, y - 16.0)
                .h_align_center()
                .v_align_bottom();
        }
    });

    gfx.render(&draw);
}
