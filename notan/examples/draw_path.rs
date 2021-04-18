use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::graphics::*;
use notan::app::{App, AppBuilder, Graphics, Plugins};
use notan::log;
use notan::prelude::*;

#[notan::main]
fn main() -> Result<(), String> {
    notan::init().draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let mut builder = Path::builder();
    builder.begin(10.0, 10.0);
    builder.line_to(100.0, 100.0);
    builder.line_to(400.0, 500.0);
    builder.bezier_to(&Quadratic {
        ctrl: (450.0, 450.0),
        to: (300.0, 200.0),
    });
    builder.end(false);
    let path = builder.fill();

    let mut draw = gfx.create_draw();

    draw.begin(Some(&Color::new(0.1, 0.2, 0.3, 1.0)));
    draw.path(&path);
    draw.end();

    gfx.render(&draw);
}
