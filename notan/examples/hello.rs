use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

#[derive(Default)]
struct State {
    count: u32,
    hello: Asset<Vec<u8>>,
    list: AssetList,
}

impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .set_config(WindowConfig::new().size(1200, 800))
        .update(update)
        .build();

    Ok(())
}

fn setup(assets: &mut AssetManager) -> State {
    State {
        count: 0,
        hello: assets.load_asset::<Vec<u8>>("hello.html").unwrap(),
        list: assets.load_list(&["hello.html", "hell.html"]).unwrap(),
    }
}

fn update(app: &mut App, state: &mut State) {
    log::info!(
        "asset: {:?} -> list: {:?} - {:?}",
        state.hello.is_loaded(),
        state.list.is_loaded(),
        state.list.progress()
    );

    state.count += 1;
}
