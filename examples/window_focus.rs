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
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();
    State { font }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let window = app.window();
    let ww = window.width() as f32;
    let hh = window.height() as f32;

    let text = if window.is_focused() {
        "Window is currently focused"
    } else {
        "Window is no longer focused"
    };

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.text(&state.font, text)
        .position(ww * 0.5, hh * 0.5)
        .size(40.0)
        .h_align_center()
        .v_align_middle();

    gfx.render(&draw);
}
