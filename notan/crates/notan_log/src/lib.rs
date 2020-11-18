pub use ::log::{debug, error, info, trace, warn};

#[cfg(target_arch = "wasm32")]
pub use ::log::Level;

#[cfg(not(target_arch = "wasm32"))]
pub use ::log::LevelFilter as Level;

#[cfg(target_arch = "wasm32")]
#[inline(always)]
pub fn init_with_level(level: Level) {
    console_log::init_with_level(level).expect("Error initializing logger");
}

#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
pub fn init_with_level(level: Level) {
    simple_logger::SimpleLogger::new()
        .with_level(level)
        .init()
        .expect("Error initializing logger");
}

#[cfg(target_arch = "wasm32")]
#[inline(always)]
pub fn init() {
    console_log::init_with_level(Level::Trace).expect("Error initializing logger");
}

#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
pub fn init() {
    simple_logger::SimpleLogger::new()
        .init()
        .expect("Error initializing logger");
}
