use notan::app::App;
use notan::core::window::WindowConfig;

fn main() -> Result<(), String> {
    let window_config = WindowConfig::default()
        .with_title("Window Config Demo")
        .with_size(1026, 600)
        .with_resizable(true)
        .with_min_size(600, 400)
        .with_max_size(1200, 800);

    // TODO vsync, window_icon and taskbar_icon.

    // Check the documentation for more options
    // let window_config = WindowConfig::new()
    //     .set_title("Window Config Demo")
    //     .set_size(1026, 600) // window's size
    //     .set_vsync(true) // enable vsync
    //     .set_resizable(true) // window can be resized
    //     .set_min_size(600, 400) // Set a minimum window size
    //     .set_window_icon(Some(PathBuf::from("./examples/assets/rust.ico")))
    //     .set_taskbar_icon(Some(PathBuf::from("./examples/assets/rust.ico")));

    let app_config = App::config().with_window(window_config);

    notan::init().add_config(app_config)?.build()
}
