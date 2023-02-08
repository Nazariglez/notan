use std::path::PathBuf;

use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    // Check the documentation for more options
    let window_config = WindowConfig::new()
        .set_title("Window Config Demo")
        .set_size(1026, 600) // window's size
        .set_vsync(true) // enable vsync
        .set_resizable(true) // window can be resized
        .set_min_size(600, 400) // Set a minimum window size
        .set_window_icon(Some(PathBuf::from("./examples/assets/rust.ico")))
        .set_taskbar_icon(Some(PathBuf::from("./examples/assets/rust.ico")));

    notan::init().add_config(window_config).build()
}
