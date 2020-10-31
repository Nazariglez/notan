use notan::app::plugins::{Plugin, Plugins};
use notan::app::{App, AppState};
use notan::log;
use notan::prelude::*;

struct PP(i32);
struct PA(i32);

impl Plugin for PP {
    // fn init<B: Backend>(app: &mut App<B>) -> Result<(), String> {
    //     let pp = app.plugins.get::<PP>();
    //     let pa = app.plugins.get::<PA>();
    //     Ok(())
    // }
}

impl Plugin for PA {}

struct State(i32);
impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    log::init();
    notan::init_with(State(0))
        .set_plugin(PP(0))
        // .set_config(&WindowConfig { cosas: 0 })
        .initialize(|| log::info!("ok..."))
        // .update(update)
        .update(update2)
        .build()
}
fn init() {
    log::info!("without params...");
}

fn update(app: &mut App, state: &mut State) {
    log::info!("with app and state");
}

fn update2(app: &mut App, plugins: &mut Plugins) {}
// fn init<S>(app: &mut App, state: &mut S) {
//     println!("hello...");
// }
//
// fn update<S>(app: &mut App, state: &mut S) {
//     // *state += 1;
//     // // let (width, height) = app.window().size();
//     // // if width < 1200 {
//     // //     app.window().set_size(width + 1, height);
//     // // }
//     // if *state == 60 {
//     //     log::info!("here!");
//     //     app.window().set_fullscreen(true);
//     // }
//     // log::info!("{{x: {},y: {}}}", app.mouse.x, app.mouse.y);
//     log::info!(
//         "space was pressed {:?}",
//         app.keyboard
//             .was_pressed(notan::app::keyboard::KeyCode::Space)
//     );
// }
