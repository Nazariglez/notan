use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

struct State {
    text: Asset<String>,
}

impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_loader(create_text_loader())
        .update(update)
        .build();

    Ok(())
}

fn setup(assets: &mut AssetManager) -> State {
    let mut state = State {
        text: assets.load_asset("hello.html").unwrap(),
    };

    state
}

fn update(state: &mut State) {}

fn create_text_loader() -> Loader {
    Loader::new()
        .use_parser(text_files_parser)
        .from_extension("html")
        .output::<String>()
}

fn text_files_parser(id: &str, bytes: Vec<u8>, storage: &mut AssetStorage) -> Result<(), String> {
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| format!("Cannot parse file {} to UTF8 text.", id))?
        .to_string();

    storage.parse::<String>(id, text)
}
