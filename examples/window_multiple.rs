use notan::app::App;
use notan::core::events::*;
use notan::core::window::WindowConfig;
use notan::prelude::*;

fn main() -> Result<(), String> {
    notan::init()
        .add_config(App::config())?
        .once(open_windows)
        .build()
}

fn open_windows(_: &InitEvent, app: &mut App) {
    (0..3).for_each(|n| {
        let _ = app.create_window(
            WindowConfig::default()
                .with_title(&format!("Window {}", n))
                .with_size(300 + n * 230, 300 + n * 200)
                .with_position((100 + n * 150) as _, (100 + n * 130) as _),
        )
        .unwrap();
    });
}
