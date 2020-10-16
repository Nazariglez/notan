use notan::app::{App, Backend, Notan, WindowConfig};
use notan::window::WinitBackend;

fn main() -> Result<(), String> {
    Notan::init_with_backend(0, WinitBackend::new().unwrap())
        .set_config(&WindowConfig { cosas: 0 })
        .initialize(init)
        .update(update)
        .build()
}

fn init<B: Backend, S>(app: &mut App<B>, state: &mut S) {
    println!("hello...");
}
fn update<B: Backend>(app: &mut App<B>, state: &mut i32) {
    println!("{}", *state);

    if *state > 60 {
        app.exit();
    }

    *state += 1;
}
