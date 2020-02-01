use nae::prelude::*;

#[nae::main]
fn main() {
    //    log::init();
    nae::init().draw(draw)
        .size(1200, 800)
        .resizable()
        .build()
        .unwrap();
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

    draw.set_color(Color::RED);
    draw.rect(0.0, 0.0, ww, hh);

    draw.set_color(Color::WHITE);
    draw.text(&format!("{} {}", ww, hh), 10.0, 10.0, 20.0);
    draw.text_ext(&format!("{} {}", ww, hh), ww-10.0, 10.0, 20.0, HorizontalAlign::Right, VerticalAlign::Top, None);
    draw.text_ext(&format!("{} {}", ww, hh), 10.0, hh-10.0, 20.0, HorizontalAlign::Left, VerticalAlign::Bottom, None);
    draw.text_ext(&format!("{} {}", ww, hh), ww-10.0, hh-10.0, 20.0, HorizontalAlign::Right, VerticalAlign::Bottom, None);

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
