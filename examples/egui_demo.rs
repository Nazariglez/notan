use notan::egui::{self, *};
use notan::prelude::*;

#[derive(Default, AppState)]
struct State {
    demo: egui_demo_lib::DemoWindows,
}

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default().resizable().vsync().size(1280, 1024);

    notan::init_with(State::default)
        .set_config(win)
        .set_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut plugin = plugins.get_mut::<EguiPlugin>().unwrap();

    let mut output = plugin.run(|ctx| state.demo.ui(ctx));
    output.clear_color(Color::BLACK);

    gfx.render(&output);
}
