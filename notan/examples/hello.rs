use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

struct State {
    text: Option<Asset<Text>>,
    text2: Asset<JS>,
}

impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_loader(create_text_loader())
        .add_loader(create_js_loader())
        .update(update)
        .build();

    Ok(())
}

fn setup(assets: &mut AssetManager) -> State {
    let mut state = State {
        text: Some(assets.load_asset("hello.html").unwrap()),
        text2: assets.load_asset("hello/hello.js").unwrap(),
    };

    state
}

fn update(state: &mut State) {
    // log::info!("{:?}", state.text.as_ref().unwrap().lock().is_some());
    let can_unwrap = match &state.text {
        Some(asset) => asset.lock().is_some(),
        _ => false,
    };

    if can_unwrap {
        let asset = state.text.take().unwrap();
        let a = asset.clone();
        log::info!("{:?}", asset.lock());
        let inner_asset = asset.unwrap();
        log::info!("{:?}", inner_asset);
    }

    match state.text2.lock() {
        Some(a) => {
            log::info!("{:?}", a);
            panic!();
        }
        _ => {}
    }
    // if let Some(opt_text) = &state.text {
    //     // let text = match opt_text.lock() {
    //     //     Some(asset) => asset,
    //     //     _ => return,
    //     // };
    //     //
    //     if !opt_text.is_loaded() {
    //         return;
    //     }
    //
    //     let text = opt_text.unwrap();
    //     log::info!("{:#?}", text);
    //     panic!();
    // }
}

fn create_text_loader() -> Loader {
    Loader::new()
        .use_parser(text_files_parser)
        .from_extension("html")
}

fn text_files_parser(id: &str, bytes: Vec<u8>) -> Result<Text, String> {
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| format!("Cannot parse file {} to UTF8 text.", id))?
        .to_string();

    Ok(Text(text))
}

#[derive(Debug)]
struct Text(String);

impl Drop for Text {
    fn drop(&mut self) {
        log::info!("bye bye");
    }
}

fn create_js_loader() -> Loader {
    Loader::new()
        .use_parser(js_files_parser)
        .from_extension("js")
}

fn js_files_parser(id: &str, bytes: Vec<u8>) -> Result<JS, String> {
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| format!("Cannot parse file {} to UTF8 text.", id))?
        .to_string();

    Ok(JS(text))
}

#[derive(Debug)]
struct JS(String);

impl Drop for JS {
    fn drop(&mut self) {
        log::info!("bye bye js!");
    }
}
