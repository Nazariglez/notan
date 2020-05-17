use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init_with(init)
        .draw(draw)
        .size(1200, 800)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> Font {
    Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap()
}

fn draw(app: &mut App, font: &mut Font) {
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
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    draw.set_text_align(HorizontalAlign::Center, VerticalAlign::Center);
    draw.text(font, text, ww * 0.5, hh * 0.5, 40.0);
    draw.end();
}
