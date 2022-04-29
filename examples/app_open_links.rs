use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .draw(draw)
        .update(update)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();
    State { font }
}

fn update(app: &mut App) {
    if app.keyboard.was_pressed(KeyCode::N) {
        app.open_link("https://github.com/Nazariglez/notan");
    } else if app.keyboard.was_pressed(KeyCode::R) {
        app.open_link("https://www.rust-lang.org");
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.text(&state.font, "Press 'n' to open Notan's website")
        .position(20.0, 20.0)
        .size(40.0)
        .color(Color::WHITE);

    draw.text(&state.font, "Press 'r' to open rust-lang.org")
        .position(20.0, 120.0)
        .size(40.0)
        .color(Color::WHITE);

    gfx.render(&draw);
}
