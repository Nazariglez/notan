use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().set_config(DrawConfig).draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    // Draw a rectangle as mask
    let mut mask = gfx.create_draw();
    mask.rect((180.0, 180.0), (440.0, 240.0));

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // Draw a triangle inside the mask, it will be displayed trimmed by the mask rectangle
    draw.mask(Some(&mask));
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0))
        .color(Color::RED);

    draw.mask(None);

    // Draw a normal triangle outside the mask
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0))
        .color(Color::YELLOW)
        .scale(0.5, 0.5);

    gfx.render(&draw);
}
