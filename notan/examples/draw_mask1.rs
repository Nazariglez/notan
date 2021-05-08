use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, Graphics, Plugins, *};
use notan::log;
use notan::prelude::*;

#[notan::main]
fn main() -> Result<(), String> {
    notan::init().draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let mut mask = gfx.create_draw();
    mask.rect((180.0, 180.0), (440.0, 240.0)).color_vertex(
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
    );

    let mut draw = gfx.create_draw();
    // draw.mask(mask, |draw| {
    //     draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0))
    //         .color(Color::RED);
    // });

    // mask.mask(draw);

    draw.mask(Some(&mask));
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0))
        .color(Color::RED);

    draw.mask(None);

    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0))
        .color(Color::YELLOW)
        .scale(0.5, 0.5);

    gfx.render(&draw);
}
