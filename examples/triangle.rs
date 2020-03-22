use nae::prelude::*;
use nae::Draw;

#[nae::main]
fn main() {
    log::init();
    nae::init().draw(draw).build().unwrap();
}

fn draw(app: &mut App, _: &mut ()) {
    let mut gfx = app.gfx();
    let mut d2 = Draw::new(gfx).unwrap();

    d2.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    d2.color = Color::GREEN;
    d2.triangle(400.0, 100.0, 100.0, 500.0, 700.0, 500.0);
    d2.end();

    println!("Draw calls: {}", d2.gfx.draw_calls());
}
