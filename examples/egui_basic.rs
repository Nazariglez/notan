use notan::egui::{self, *};
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().add_config(EguiConfig).draw(draw).build()
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins) {
    let mut plugin = plugins.get_mut::<EguiPlugin>().unwrap();

    let mut output = plugin.run(|ctx| {
        egui::SidePanel::left("side_panel").show(&ctx, |ui| {
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
    });

    output.clear_color(Color::BLACK);

    gfx.render(&output);
}
