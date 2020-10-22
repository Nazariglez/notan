use notan::app::{App, WindowConfig};
use notan::log;
use notan::prelude::*;

#[notan::main]
fn main() -> Result<(), String> {
    log::init();
    notan::init_with(0)
        .set_config(&WindowConfig { cosas: 0 })
        .initialize(init)
        .update(update)
        .build()
}

fn init<B: Backend, S>(app: &mut App<B>, state: &mut S) {
    println!("hello...");
}

fn update<B: Backend>(app: &mut App<B>, state: &mut i32) {
    *state += 1;
    // let (width, height) = app.window().size();
    // if width < 1200 {
    //     app.window().set_size(width + 1, height);
    // }
    if *state == 60 {
        log::info!("here!");
        app.window().set_fullscreen(true);
    }
}
