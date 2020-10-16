use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> Font {
    Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap()
}

fn draw(app: &mut App, font: &mut Font) {
    let (x, y) = app.mouse.position();

    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    //Draw circle on the mouse position
    draw.color = Color::RED;
    draw.circle(x, y, 10.0);

    //Draw mouse position
    draw.color = Color::WHITE;
    draw.set_text_align(HorizontalAlign::Center, VerticalAlign::Center);
    draw.text(font, &format!("x: {} - y: {}", x, y), 400.0, 300.0, 80.0);

    draw.end();
}
