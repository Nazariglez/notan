use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

#[derive(Default)]
struct State {
    count: u32,
    hello: Option<Asset<Blob>>,
}

impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    log::init();

    notan::init_with(State::default())
        .set_config(WindowConfig::new().size(1200, 800))
        .add_loader::<BlobLoader>()
        .initialize(|assets: &mut AssetManager, state: &mut State| {
            state.hello = Some(assets.load_asset::<Blob>("hello.html").unwrap());
        })
        .update(update)
        .build();

    Ok(())
}

fn update(app: &mut App, state: &mut State) {
    if let Some(asset) = &state.hello {
        log::info!("{:?}", asset.is_loaded());
    }
}
