pub use ::log::{debug, error, info, trace, warn, Level};

#[cfg(target_arch = "wasm32")]
pub fn init_with_level(level: Level) {
    console_log::init_with_level(level);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn init_with_level(level: Level) {
    simple_logger::init_with_level(level);
}

#[cfg(target_arch = "wasm32")]
pub fn init() {
    console_log::init_with_level(Level::Trace);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn init() {
    simple_logger::init();
}
