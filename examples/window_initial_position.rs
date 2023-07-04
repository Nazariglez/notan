use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init).build()
}

fn init(app: &mut App) {
    // A position of (0, 0) puts the top-left of the window on the top-left of the main monitor.
    // Increasing the first number (x position) moves the window to the right; increasing the second (y) moves it down. Decreasing does the opposite.
    app.window().set_position(1000, 50);
}