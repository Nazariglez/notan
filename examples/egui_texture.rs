use notan::egui::{self, *};
use notan::prelude::*;

#[derive(AppState)]
struct State {
    egui_tex: egui::TextureId,
    img_size: egui::Vec2,
}

impl State {
    fn new(gfx: &mut Graphics) -> State {
        let texture = gfx
            .create_texture()
            .from_image(include_bytes!("assets/rust-logo-256x256.png"))
            .build()
            .unwrap();

        let img_size = egui::vec2(texture.width(), texture.height());
        let egui_texture_id = gfx.register_egui_texture(&texture).unwrap();

        Self {
            img_size,
            egui_tex: egui_texture_id,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let ctx = plugins
        .get_mut::<EguiPlugin>()
        .map(|mut plugin| plugin.create_context(Some(Color::BLACK)))
        .unwrap();

    egui::Window::new("Notan Texture").show(&ctx, |ui| {
        ui.image(state.egui_tex, state.img_size);
    });

    gfx.render(&ctx);
}
