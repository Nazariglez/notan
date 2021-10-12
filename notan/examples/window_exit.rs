use notan::app::keyboard::KeyCode;
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().update(update).build()
}

fn update(app: &mut App) {
    // Closes the App pressing the Escape key.
    // On browsers the requestAnimationFrame will stop but the canvas will still be visible
    if app.keyboard.was_pressed(KeyCode::Escape) {
        app.exit();
    }
}
