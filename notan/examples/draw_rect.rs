use notan::prelude::*;

#[notan::main]
fn main() -> Result<(), String> {
    notan::init().set_config(DrawConfig).draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.rect((100.0, 100.0), (600.0, 400.0));
    gfx.render(&draw);
}
