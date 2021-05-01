use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
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

struct State {
    texture: Texture,
}

impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .set_plugin(StatsPlugin)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let image = TextureInfo::from_image(include_bytes!("assets/ferris.png")).unwrap();
    State {
        texture: gfx.create_texture(image).unwrap(),
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw2();

    draw.background(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.image(&state.texture)
        .color(Color::GREEN)
        .crop(
            (0.0, 0.0),
            (state.texture.width() * 0.5, state.texture.height() * 0.5),
        )
        // .alpha(0.5)
        .size(800.0, 600.0);

    draw.image(&state.texture)
        .color(Color::RED)
        // .alpha(0.5)
        .position(100.0, 100.0);

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
