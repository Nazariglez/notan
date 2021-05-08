use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, DrawImages, DrawShapes, Graphics, Plugins};
use notan::log;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    img1: Texture,
    img2: Texture,
    count: f32,
    multi: f32,
}

impl State {
    pub fn count(&mut self, value: f32) {
        if self.count >= 200.0 || self.count <= 0.0 {
            self.multi *= -1.0;
        }

        self.count += value * self.multi;
    }
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .update(|app: &mut App, state: &mut State| state.count(1.0))
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let img1 = TextureInfo::from_image(include_bytes!("assets/green_panel.png")).unwrap();
    let img2 = TextureInfo::from_image(include_bytes!("assets/grey_button.png")).unwrap();
    let texture1 = gfx.create_texture(img1).unwrap();
    let texture2 = gfx.create_texture(img2).unwrap();
    State {
        img1: texture1,
        img2: texture2,
        count: 1.0,
        multi: 1.0,
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.nine_slice(&state.img1).position(50.0, 50.0).size(
        state.img1.width() + state.count,
        state.img1.height() + state.count,
    );

    draw.nine_slice(&state.img2).position(450.0, 50.0).size(
        state.img2.width() + state.count,
        state.img2.height() + state.count,
    );

    draw.triangle((100.0, 100.0), (200.0, 100.0), (150.0, 200.0));

    gfx.render(&draw);
}
