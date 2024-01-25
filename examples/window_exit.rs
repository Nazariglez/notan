use std::ptr::write;
use notan::app::App;
use notan::core::events::UpdateEvent;
use notan::prelude::*;

#[derive(AppState)]
struct State(usize);


fn main() -> Result<(), String> {
    notan::init_with(|| Ok(State(0)))
        .add_config(App::config())?
        .on(update)
        .build()
}

fn update(_: &UpdateEvent, app: &mut App, state: &mut State) {
    state.0 += 1;
    println!("up {}", state.0);
    // Closes the App pressing the Escape key.
    // On browsers the requestAnimationFrame will stop but the canvas will still be visible
    //if app.keyboard.was_pressed(KeyCode::Escape) {
    //     app.exit();
    //}

    if state.0 == 3 {
        app.exit();
    }
}
