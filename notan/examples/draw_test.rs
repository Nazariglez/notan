use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::graphics::Path;
use notan::app::{App, AppBuilder, AppFlow, Graphics, Plugins, *};
use notan::log;
use notan::prelude::*;
use notan::{fragment_shader, vertex_shader};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "sb")]
    fn sb();

    #[wasm_bindgen(js_name = "se")]
    fn se();
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init().set_plugin(StatsPlugin).draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw2();
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0))
        .color(Color::RED)
        .stroke(10.0);
    gfx.render(&draw);
}

struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn pre_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
        sb();
        Ok(Default::default())
    }

    fn post_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
        se();
        Ok(Default::default())
    }
}
