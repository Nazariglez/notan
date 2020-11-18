use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

#[derive(Default)]
struct State {
    count: u32,
    hello: Asset<Vec<u8>>,
}

impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    log::init();

    notan::init_with(init)
        .set_config(WindowConfig::new().size(1200, 800))
        .update(update)
        .build();

    Ok(())
}

fn init(app: &mut App, assets: &mut AssetManager) -> State {
    State {
        count: 0,
        hello: assets.load_asset::<Vec<u8>>("hello.html").unwrap(),
    }
}

fn update(app: &mut App, state: &mut State) {
    log::info!("{:?}", state.hello.is_loaded());
}
