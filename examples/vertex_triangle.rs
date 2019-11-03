use nae::prelude::*;

#[nae_start]
fn main() {
    nae::init({}).draw(on_draw).build().unwrap();
}

fn on_draw(app: &mut App, state: ()) {
    let gfx = &mut app.graphics;
    gfx.begin()
        .clear(color::rgba(0.1, 0.2, 0.3, 1.0))
        .set_color(color::Color::Green)
        .vertex(&[
            graphics::Vertex::new(600.0, 200.0, Color::Red),
            graphics::Vertex::new(700.0, 300.0, Color::Green),
            graphics::Vertex::new(500.0, 300.0, Color::Blue),
        ])
        .end();
}
