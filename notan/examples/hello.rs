use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

#[derive(Default)]
struct State {
    manager: AssetManager,
    count: u32,
}

impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    log::init();

    notan::init_with(State::default())
        .set_config(WindowConfig::new().size(1200, 800))
        .initialize(|state: &mut State| {
            state.manager.add_loader::<BlobLoader>();
            state.manager.load_asset::<Blob>("hello.html");
        })
        .update(update)
        .build();

    Ok(())
}

fn update(app: &mut App, state: &mut State) {
    state.manager.tick();
}
