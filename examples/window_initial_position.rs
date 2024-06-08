use notan::app::App;
use notan::core::window::WindowConfig;

fn main() -> Result<(), String> {
    let win = WindowConfig::default().with_position(100, 200);
    notan::init()
        .add_config(App::config().with_window(win))?
        .build()
}
