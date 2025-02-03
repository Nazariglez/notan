use std::sync::Arc;

use notan::egui::{self, *};
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::new()
        .set_vsync(true)
        .set_lazy_loop(true)
        .set_high_dpi(true);

    notan::init()
        .add_config(win)
        .add_config(EguiConfig)
        .initialize(initialize)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, plugins: &mut Plugins) {
    let mut output = plugins.egui(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui using custom fonts");
            ui.label("This text is using a custom font");
        });
    });

    output.clear_color(Color::BLACK);
    gfx.render(&output);
}

// Initialize callback is called just once after setup and before the app's loop
fn initialize(plugins: &mut Plugins) {
    plugins.egui(setup);
}

// from https://github.com/emilk/egui/blob/master/examples/custom_font/src/main.rs#L17-L46
fn setup(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "./assets/Ubuntu-B.ttf"
        ))),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
