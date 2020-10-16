use nae::prelude::*;

const TEXT: &'static str = include_str!("assets/loremipsum.txt");

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> Font {
    Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap()
}

fn draw(app: &mut App, font: &mut Font) {
    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    draw.set_text_align(HorizontalAlign::Center, VerticalAlign::Center);
    draw.text_ext(font, TEXT, 400.0, 300.0, 16.0, 600.0);
    draw.end();
}
