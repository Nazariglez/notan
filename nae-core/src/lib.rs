pub mod logger;
pub mod math;
pub mod graphics;
pub mod resources;

pub use logger::{debug, error, info, trace, warn};
use crate::graphics::BaseContext2d;

pub trait BaseApp {
    type Graphics: BaseContext2d;

    fn graphics(&mut self) -> &mut Self::Graphics;
}

#[cfg(target_arch = "wasm32")]
pub fn date_now() -> u64 {
    js_sys::Date::now() as u64
}

#[cfg(not(target_arch = "wasm32"))]
pub fn date_now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
