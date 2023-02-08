use notan::egui::*;
use notan::prelude::*;

#[derive(Default, AppState)]
struct State {
    demo: egui_demo_lib::DemoWindows,
}

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default()
        .set_resizable(true)
        .set_size(1280, 1024)
        .set_vsync(true)
        .set_high_dpi(true)
        // enable lazy mode to only draw after an input
        .set_lazy_loop(true);

    notan::init_with(State::default)
        .add_config(win)
        .add_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut output = plugins.egui(|ctx| state.demo.ui(ctx));
    output.clear_color(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&output);
    }
}
