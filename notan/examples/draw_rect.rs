use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, Graphics, Plugins};
use notan::log;
use notan::prelude::*;

#[notan::main]
fn main() -> Result<(), String> {
    notan::init().draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();

    draw.begin(Some(&Color::new(0.1, 0.2, 0.3, 1.0)));
    draw.rect(0.0, 0.0, 1.0, 1.0);
    draw.end();

    gfx.render(&draw);
}
