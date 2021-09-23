use notan::prelude::*;

#[notan::main]
fn main() -> Result<(), String> {
    notan::init().set_config(DrawConfig).draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::new(0.1, 0.2, 0.3, 1.0));

    // set and additive color blend for everything draw from here
    draw.set_blend_mode(BlendMode::ADD);

    draw.circle(150.0)
        .position(400.0, 225.0)
        .color(Color::GREEN);

    draw.circle(150.0).position(325.0, 375.0).color(Color::RED);

    draw.circle(150.0).position(475.0, 375.0).color(Color::BLUE);

    gfx.render(&draw);
}
