use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

struct State(i32);
impl AppState for State {}

// struct PP(i32);
// impl Plugin for PP {
//     fn init(&mut self, app: &mut App) -> Result<notan_app::AppFlow, String> {
//         println!("here...");
//         Ok(Default::default())
//     }
// }

#[notan::main]
fn main() -> Result<(), String> {
    log::init();

    let app = notan::init_with(State(0))
        // .set_plugin(PP(10))
        .set_config(WindowConfig::new().size(1200, 800))
        .initialize(|| log::info!("ok..."))
        .event(|evt| log::info!("{:?}", evt))
        .update(update);

    // app.build();
    //
    let mut manager = LoadManager::new();
    manager.add_loader::<BlobLoader>();
    // let b = manager.load_asset::<Blob>("lel.blob")?;
    //log::info!("{:?}", b.lock());
    manager.load(&["lel.txt"]);
    log::info!("{:?}", manager.get::<Blob>("lel.txt").unwrap().lock());

    // let blob = manager.get::<Blob>("blob");
    // log::info!("{:?}", blob);

    Ok(())
}

fn update(app: &mut App, plugins: &mut Plugins) {
    // let pp = plugins.get::<PP>().unwrap();
    // log::info!("with app and state {}", pp.0);
}
