use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    // Check the documentation for more options
    let window_config = WindowConfig::new()
        .title("Window Config Demo")
        .size(1026, 600) // window's size
        .vsync() // enable vsync
        .resizable() // window can be resized
        .min_size(600, 400); // Set a minimum window size

    notan::init().add_config(window_config).build()
}
