pub use ::log::{debug, error, info, trace, warn};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(s: &str);
}

#[cfg(target_arch = "wasm32")]
pub use ::log::Level;

#[cfg(not(target_arch = "wasm32"))]
pub use ::log::LevelFilter as Level;

#[cfg(target_arch = "wasm32")]
#[inline(always)]
pub fn init_with_level(level: Level) {
    if let Err(e) = console_log::init_with_level(level) {
        console_error(&format!("Error initializing logger: {}", e));
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
pub fn init_with_level(level: Level) {
    if let Err(e) = simple_logger::SimpleLogger::new().with_level(level).init() {
        println!("Error initializing logger: {}", e);
    }
}

#[cfg(target_arch = "wasm32")]
#[inline(always)]
pub fn init() {
    if let Err(e) = console_log::init_with_level(Level::Trace) {
        console_error(&format!("Error initializing logger: {}", e));
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
pub fn init() {
    if let Err(e) = simple_logger::SimpleLogger::new().init() {
        println!("Error initializing logger: {}", e);
    }
}
