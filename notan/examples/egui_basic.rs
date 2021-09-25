use notan::egui::{self, *};
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init()
        .set_config(EguiConfig)
        .set_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));
    gfx.render(&draw);

    let mut egui = plugins.get_mut::<EguiPlugin>().unwrap();

    egui.begin_frame();
    egui::SidePanel::left("my_side_panel").show(&egui, |ui| {
        ui.heading("Hello World!");
        if ui.button("Quit").clicked() {
            app.exit();
        }

        egui::ComboBox::from_label("Version")
            .width(150.0)
            .selected_text("foo")
            .show_ui(ui, |ui| {
                egui::CollapsingHeader::new("Dev")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label("contents");
                    });
            });
    });
    let renderer = egui.end_frame();

    gfx.render(&renderer);
}
