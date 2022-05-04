use notan::log;
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init()
        .add_config(log::LogConfig::debug())
        .initialize(start)
        .build()
}

fn start() {
    log::debug!("Hello, this is a debug log...");
    log::info!("And this is a info log!");
    log::warn!("I'm warning you");
    log::error!("I'm an error, I told you...");
}
