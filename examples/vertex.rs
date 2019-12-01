use nae::prelude::*;

#[nae_start]
fn main() {
    nae::init().draw(draw).build().unwrap();
}

fn draw(app: &mut App, _: &mut ()) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.vertex(&[
        Vertex::new(400.0, 100.0, Color::RED),
        Vertex::new(100.0, 500.0, Color::GREEN),
        Vertex::new(700.0, 500.0, Color::BLUE),
    ]);
    draw.end();
}
