use notan::prelude::*;
use notan::app::{App, Notan, WindowConfig};
use notan::window::WinitBackend;

fn main() -> Result<(), String> {
    // Notan::init_with_backend(0, WinitBackend::new().unwrap())
    Notan::init_with(0)
        .set_config(&WindowConfig { cosas: 0 })
        .initialize(init)
        .update(update)
        .build()
}

fn init<B: Backend, S>(app: &mut App<B>, state: &mut S) {
    println!("hello...");
}
fn update<B: Backend>(app: &mut App<B>, state: &mut i32) {
    // println!("{}", *state);
    //
    // if *state > 1000060 {
    //     app.exit();
    // }
    //
    // *state += 1;
}
