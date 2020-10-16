use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init().draw(draw).build().unwrap();
}

fn draw(app: &mut App, _: &mut ()) {
    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.blend_mode = BlendMode::ADD;

    draw.color = Color::GREEN;
    draw.circle(400.0, 225.0, 150.0);

    draw.color = Color::RED;
    draw.circle(325.0, 375.0, 150.0);

    draw.color = Color::BLUE;
    draw.circle(475.0, 375.0, 150.0);

    draw.end();
}
