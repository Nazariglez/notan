use notan::egui::*;
use notan::prelude::*;

#[derive(Default, AppState)]
struct State {
    demo: egui_demo_lib::DemoWindows,
}

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default()
        .resizable()
        .size(1280, 1024)
        .vsync()
        // enable lazy mode to only draw after an input
        .lazy_loop();

    notan::init_with(State::default)
        .add_config(win)
        .add_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut output = plugins.egui(|ctx| state.demo.ui(ctx));
    output.clear_color(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&output);

        // using the lazy loop we can check if egui demo needs repaint
        // to ask for the next frame in case it's drawing an animation
        app.window().request_frame();
    }
}
