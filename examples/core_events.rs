use notan::core::events::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {}

fn main() -> Result<(), String> {
    notan::init_with(|| Ok(State {}))
        .once(on_init)
        .on(on_start_frame)
        .on(on_update)
        .on(on_draw)
        .on(on_end_frame)
        .on(on_close)
        .build()
}

fn on_init(_: &InitEvent, _state: &mut State) {
    println!("On Init");
}

fn on_start_frame(_: &FrameStartEvent) {
    println!("On Start Frame");
}

fn on_update(_: &UpdateEvent) {
    println!("On Update");
}

fn on_draw(_: &FrameEndEvent) {
    println!("On Draw");
}

fn on_end_frame(_: &FrameEndEvent) {
    println!("On End Frame");
}

fn on_close(_: &CloseEvent) {
    println!("On Close");
}
