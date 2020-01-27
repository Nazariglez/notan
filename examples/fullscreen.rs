use nae::prelude::*;

#[nae::main]
fn main() {
    log::init();
    nae::init().draw(draw).build().unwrap();
}

fn draw(app: &mut App, _: &mut ()) {
    let ww = app.width();
    let hh = app.height();

    if app.keyboard.was_pressed(KeyCode::Space) {
        let full = !app.fullscreen();
        app.set_fullscreen(full);
    }

    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.text_ext(
        "Press Space to toggle fullscreen mode",
        ww * 0.5,
        hh * 0.5,
        40.0,
        HorizontalAlign::Center,
        VerticalAlign::Center,
        None,
    );
    draw.end();
}
