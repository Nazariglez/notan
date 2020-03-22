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
    d2.color = Color::BLUE;
    d2.rect(-0.6, -0.6, 1.2, 1.2);
    d2.color = Color::GREEN;
    d2.triangle(-0.5, -0.5, 0.5, -0.5, 0.0, 0.5);
    d2.color = Color::RED;
    d2.triangle(-0.1, -0.1, 0.1, -0.1, 0.0, 0.1);
    for i in 0..100 {
        d2.color = Color::GREEN;
        d2.triangle(-0.5, -0.5, 0.5, -0.5, 0.0, 0.5);
        d2.color = Color::RED;
        d2.triangle(-0.1, -0.1, 0.1, -0.1, 0.0, 0.1);
    }
    d2.end();

    println!("Draw calls: {}", d2.gfx.draw_calls());
}
