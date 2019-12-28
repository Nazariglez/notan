pub use ::log::{debug, error, info, trace, warn, Level};

#[cfg(target_arch = "wasm32")]
pub fn init(level: Level) {
    console_log::init_with_level(level);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn init(level: Level) {
    simple_logger::init_with_level(level);
}
