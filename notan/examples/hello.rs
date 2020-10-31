use notan::app::config::WindowConfig;
use notan::app::{App, Plugins};
use notan::log;
use notan::prelude::*;

struct State(i32);
impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    log::init();
    notan::init_with(State(0))
        .set_config(WindowConfig::new().size(1200, 800))
        .initialize(|| log::info!("ok..."))
        .update(update)
        .build()
}

fn update(app: &mut App, state: &mut State) {
    // log::info!("with app and state");
}
