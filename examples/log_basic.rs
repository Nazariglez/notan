use notan::log;
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    // By default the log level is Debug for debug builds, and Warn for release builds.

    // We'll override to debug always
    let level = log::LevelFilter::Debug;

    // We use the default config with a custom log level
    let log_config = log::LogConfig::default().level(level);

    notan::init()
        .add_config(log_config)
        .initialize(start)
        .build()
}

fn start() {
    log::debug!("Hello, this is a debug log...");
    log::info!("And this is a info log!");
    log::warn!("I'm warning you");
    log::error!("I'm an error, I told you...");
}
