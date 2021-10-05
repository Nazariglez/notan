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
            .from_image(include_bytes!("assets/ferris.png"))
            .build()
            .unwrap();

        let img_size = egui::vec2(texture.width(), texture.height());
        let egui_texture_id = texture.egui_id(gfx).unwrap(); //gfx.register_egui_texture(&texture).unwrap();

        Self {
            img_size,
            egui_tex: egui_texture_id,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .set_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    // Get the EGUI plugin that contains egui::CtxRef
    let mut plugin = plugins.get_mut::<EguiPlugin>().unwrap();

    // Create a EguiContext to render the frame. We can pass a color to clear the frame if we want
    let egui_ctx = plugin.create_context(Some(Color::WHITE));

    // Use EGUI as usual passing the context
    egui::SidePanel::left("side_panel").show(&egui_ctx, |ui| {
        ui.heading("Egui Plugin Example");

        ui.separator();
        if ui.button("Quit").clicked() {
            app.exit();
        }

        ui.separator();
        ui.label("Welcome to a basic example of how to use Egui with notan.");

        ui.separator();
        ui.label("Check the source code to learn more about how it works");
    });

    egui::Window::new("NativeTextureDisplay")
        .resizable(true)
        .show(&egui_ctx, |ui| {
            ui.image(state.egui_tex, state.img_size);
        });

    // Draw the context to the screen or to a render texture
    gfx.render(&egui_ctx);
}
