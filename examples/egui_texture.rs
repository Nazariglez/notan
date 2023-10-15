use notan::egui::{self, *};
use notan::prelude::*;

#[derive(AppState)]
struct State {
    sized_texture: egui::SizedTexture,
}

impl State {
    fn new(gfx: &mut Graphics) -> State {
        let texture = gfx
            .create_texture()
            .from_image(include_bytes!("assets/rust-logo-256x256.png"))
            .with_premultiplied_alpha()
            .build()
            .unwrap();

        let sized_texture = gfx.egui_register_texture(&texture);

        Self { sized_texture }
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
    let mut output = plugins.egui(|ctx| {
        egui::Window::new("Notan Texture").show(ctx, |ui| {
            ui.image(state.sized_texture);
        });
    });

    output.clear_color(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&output);
    }
}
