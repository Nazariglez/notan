use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, Graphics, Plugins};
use notan::log;
use notan::prelude::*;

#[notan::main]
fn main() -> Result<(), String> {
    if let Err(e) = notan::init().draw(draw).build() {
        log::error!("{}", e);
    }

    Ok(())
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();

    draw.begin(Some(&Color::new(0.1, 0.2, 0.3, 1.0)));
    draw.set_color(&Color::GREEN);
    // draw.triangle(400.0, 100.0, 100.0, 500.0, 700.0, 500.0);
    draw.triangle(0.5, 1.0, 0.0, 0.0, 1.0, 0.0);
    draw.end();

    gfx.render(&draw);
}
