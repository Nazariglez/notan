use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    // Check the documentation for more options
    let window_config = WindowConfig::new()
        .title("Window Icon Data Demo")
        .set_window_icon_data(Some(include_bytes!("./assets/rust.ico")))
        .set_taskbar_icon_data(Some(include_bytes!("./assets/rust.ico")));

    notan::init().add_config(window_config).build()
}
