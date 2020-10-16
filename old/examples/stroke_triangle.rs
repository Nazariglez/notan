use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init().draw(draw).build().unwrap();
}

fn draw(app: &mut App, _: &mut ()) {
    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    draw.color = Color::GREEN;
    draw.stroke_triangle(400.0, 100.0, 100.0, 500.0, 700.0, 500.0, 10.0);
    draw.end();
}
