use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init().draw(draw).size(1200, 800).build().unwrap();
}

fn draw(app: &mut App, _: &mut ()) {
    let ww = app.width();
    let hh = app.height();

    if app.keyboard.was_pressed(KeyCode::Space) {
        let full = !app.fullscreen();
        app.set_fullscreen(full);
    }

    let text = if app.fullscreen() {
        "Press Space to disable the fullscreen mode"
    } else {
        "Press Space to enable the fullscreen mode"
    };

    let draw = app.draw();
    draw.begin();
    draw.clear(Color::new(0.1, 0.2, 0.3, 1.0));
    draw.text_ext(
        text,
        ww * 0.5,
        hh * 0.5,
        40.0,
        HorizontalAlign::Center,
        VerticalAlign::Center,
        None,
    );
    draw.end();
}
