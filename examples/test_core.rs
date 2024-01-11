use notan::core::events;
use notan::prelude::*;
use notan_core::AppState;

struct State {}
impl AppState for State {}

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

fn on_init(_: &events::InitEvent, state: &mut State) {
    println!("On Init");
}

fn on_start_frame(_: &events::FrameStartEvent) {
    println!("On Start Frame");
}

fn on_update(_: &events::UpdateEvent) {
    println!("On Update");
}

fn on_draw(_: &events::FrameEndEvent) {
    println!("On Draw");
}

fn on_end_frame(_: &events::FrameEndEvent) {
    println!("On End Frame");
}

fn on_close(_: &events::CloseEvent) {
    println!("On Close");
}
