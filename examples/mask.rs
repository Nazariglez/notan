use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init().draw(draw).build().unwrap();
}

fn draw(app: &mut App, _: &mut ()) {
    let draw = app.draw2();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.start_mask(|draw| {
        draw.rect(180.0, 180.0, 440.0, 240.0);
    });

    draw.color = Color::RED;
    draw.triangle(400.0, 100.0, 100.0, 500.0, 700.0, 500.0);

    draw.end_mask();
    draw.end();
}
