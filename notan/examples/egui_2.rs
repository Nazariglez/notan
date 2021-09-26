use notan::app::config::WindowConfig;
use notan::egui::{self, *};
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init()
        .set_config(WindowConfig::new().multisampling(8))
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
    let mut name = "My Super Name".to_string();
    let mut age = 34;

    egui::CentralPanel::default().show(&egui, |ui| {
        ui.heading("My egui Application");
        ui.horizontal(|ui| {
            ui.label("Your name: ");
            ui.text_edit_singleline(&mut name);
        });
        ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
        if ui.button("Click each year").clicked() {
            age += 1;
        }
        ui.label(format!("Hello '{}', age {}", name, age));
    });
    let renderer = egui.end_frame();

    gfx.render(&renderer);
}
