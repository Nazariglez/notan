use notan::app::config::WindowConfig;
use notan::prelude::*;

#[notan::main]
fn main() -> Result<(), String> {
    // Check the documentation for more options
    let window_config = WindowConfig::new()
        .title("Window Config Demo")
        .size(1026, 800) // window's size
        .vsync() // enable vsync
        .resizable() // window can be resized
        .min_size(600, 400); // Set a minimum window size

    notan::init().set_config(window_config).build()
}
