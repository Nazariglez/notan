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
    let mut draw = gfx.create_draw();

    draw.begin(Some(&Color::new(0.1, 0.2, 0.3, 1.0)));

    //triangle
    draw.vertex_color(
        &[
            VertexColor::new(400.0, 100.0, Color::new(1.0, 0.1, 0.2, 1.0)),
            VertexColor::new(100.0, 500.0, Color::new(0.1, 1.0, 0.2, 1.0)),
            VertexColor::new(700.0, 500.0, Color::new(0.1, 0.2, 1.0, 1.0)),
        ],
        &[0, 1, 2],
    );

    draw.end();

    gfx.render(&draw);
}
