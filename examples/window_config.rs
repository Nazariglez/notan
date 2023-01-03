use std::path::PathBuf;

use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    // Check the documentation for more options
    let window_config = WindowConfig::new()
        .title("Window Config Demo")
        .size(1026, 600) // window's size
        .vsync(true) // enable vsync
        .resizable(true) // window can be resized
        .min_size(600, 400) // Set a minimum window size
        .window_icon(Some(PathBuf::from("./examples/assets/rust.ico")))
        .taskbar_icon(Some(PathBuf::from("./examples/assets/rust.ico")));

    notan::init().add_config(window_config).build()
}
