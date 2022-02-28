use notan::egui::{self, *};
use notan::prelude::*;

#[derive(AppState)]
struct State {
    tex_id: egui::TextureId,
    img_size: egui::Vec2,
}

impl State {
    fn new(gfx: &mut Graphics) -> State {
        let texture = gfx
            .create_texture()
            .from_image(include_bytes!("assets/rust-logo-256x256.png"))
            .with_premultiplied_alpha()
            .build()
            .unwrap();

        let img_size: egui::Vec2 = texture.size().into();
        let tex_id = gfx.egui_register_texture(&texture);

        Self { img_size, tex_id }
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
    let mut plugin = plugins.get_mut::<EguiPlugin>().unwrap();

    let mut output = plugin.run(|ctx| {
        egui::Window::new("Notan Texture").show(ctx, |ui| {
            ui.image(state.tex_id, state.img_size);
        });
    });

    output.clear_color(Color::BLACK);

    gfx.render(&output);
}
