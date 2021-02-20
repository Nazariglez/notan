use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

//TODO
//
// clone examples of https://tsherif.github.io/picogl.js/
//
//
// TODO

struct State {
    list: AssetList,
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
    let state = State {
        list: assets.load_list(&["hello.html", "hello/hello.js"]).unwrap(),
    };

    state
}

fn update(state: &mut State) {
    log::info!("progress: {:?}", state.list.progress());
    if state.list.is_loaded() {
        let js = state.list.take::<Text>("hello.html").unwrap();
        log::info!("{:?}", js);
    }
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
