use notan::egui::{self, *};
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().set_config(EguiConfig).draw(draw).build()
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins) {
    // Get the EGUI plugin that contains egui::CtxRef
    let mut plugin = plugins.get_mut::<EguiPlugin>().unwrap();

    // Create a EguiContext to render the frame. We can pass a color to clear the frame if we want
    let egui_ctx = plugin.create_context(Some(Color::BLACK));

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

    // Draw the context to the screen or to a render texture
    gfx.render(&egui_ctx);
}
