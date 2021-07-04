use keyboard::*;
use notan::prelude::*;

#[derive(notan::AppState)]
struct State {
    font: Font,
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup).update(update).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("./assets/Ubuntu-B.ttf"))
        .unwrap();
    State { font }
}

fn update(app: &mut App) {
    if app.keyboard.was_pressed(KeyCode::Space) {
        let full = !app.window().is_fullscreen();
        app.window().set_fullscreen(full);
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let window = app.window();
    let ww = window.width() as f32;
    let hh = window.height() as f32;

    let text = if window.is_fullscreen() {
        "Press Space to disable the fullscreen mode"
    } else {
        "Press Space to enable the fullscreen mode"
    };

    let mut draw = gfx.create_draw();
    draw.text(&state.font, text)
        .position(ww * 0.5, hh * 0.5)
        .size(40.0)
        .h_align_center()
        .v_align_middle();

    gfx.render(&draw);
}
