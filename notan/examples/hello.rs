use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

struct State {
}

impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_loader(create_text_loader())
        .build();

    Ok(())
}

fn create_text_loader() -> Loader {
    Loader::new()
        .use_parser(text_files_parser)
        .from_extension("html")
        .output::<String>()
}

fn setup() -> State {
    let mut state = State {
    };

    // let ss = state.assets.load_asset::<String>("hello.html").unwrap();
    // let ss = state
    //     .assets
    //     .load_asset::<String>("renderer_triangle.html")
    //     .unwrap();

    state
}

fn text_files_parser(id: &str, bytes: Vec<u8>, storage: &mut AssetStorage2) -> Result<(), String> {
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| format!("Cannot parse file {} to UTF8 text.", id))?
        .to_string();

    log::info!("{}", text);
    storage.parse::<String>(id, text);
    Ok(())
}
