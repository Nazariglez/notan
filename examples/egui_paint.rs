use notan::egui::{self, *};
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().add_config(EguiConfig).draw(draw).build()
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins) {
    let mut output = plugins.egui(|ctx| {
        egui::Window::new("Paint").show(ctx, |ui| {
            let rect = egui::Rect {
                min: egui::Pos2::new(0.0, 0.0),
                max: egui::Pos2::new(gfx.size().0 as _, gfx.size().1 as _),
            };

            let callback = egui::PaintCallback {
                rect,
                callback: std::sync::Arc::new(move |info, ctx| {
                    if let Some(device) = ctx.downcast_ref::<Device>() {
                        let mut renderer =
                            Renderer::new(info.viewport.max.x as _, info.viewport.max.y as _);
                        renderer.begin(Some(&ClearOptions::color(Color::ORANGE)));
                        renderer.end();

                        println!("Yes the callback works...");
                        // device.render(renderer.commands());
                    }
                }),
            };

            ui.painter().add(callback);
        });
    });

    output.clear_color(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&output);
    }
}
