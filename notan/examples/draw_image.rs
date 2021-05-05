use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, DrawImages, DrawPattern, DrawShapes, Graphics, Plugins};
use notan::log;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    img: Texture,
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(init).draw(draw).build()
}

fn init(gfx: &mut Graphics) -> State {
    let img = TextureInfo::from_image(include_bytes!("assets/ferris.png")).unwrap();
    let texture = gfx.create_texture(img).unwrap();
    State { img: texture }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw2();
    draw.image(&state.img).position(250.0, 200.0);
    gfx.render(&draw);
}
