use nae::prelude::*;

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
    draw.text(font, "Hello World!", 188.0, 260.0, 80.0);
    draw.end();
}
