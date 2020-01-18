use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init().draw(draw).build().unwrap();
}

fn draw(app: &mut App, _: &mut ()) {
    let (x, y) = app.mouse.position();

    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    //Draw circle on the mouse position
    draw.set_color(Color::RED);
    draw.circle(x, y, 10.0);

    //Draw mouse position
    draw.set_color(Color::WHITE);
    draw.text_ext(
        &format!("x: {} - y: {}", x, y),
        400.0,
        300.0,
        80.0,
        HorizontalAlign::Center,
        VerticalAlign::Center,
        None,
    );

    draw.end();
}
