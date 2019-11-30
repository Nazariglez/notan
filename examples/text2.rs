use nae::prelude::*;

const TEXT: &'static str = include_str!("./assets/loremipsum.txt");

#[nae_start]
fn main() {
    nae::init().draw(draw).build().unwrap();
}

fn draw(app: &mut App, _: &mut ()) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.text_ext(
        TEXT,
        400.0,
        300.0,
        16.0,
        HorizontalAlign::Center,
        VerticalAlign::Center,
        Some(600.0),
    );
    draw.end();
}
