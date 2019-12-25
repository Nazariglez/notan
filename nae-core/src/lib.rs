pub mod graphics;
pub mod logger;
pub mod math;
pub mod resources;
pub mod window;

use crate::graphics::BaseContext2d;
pub use logger::{debug, error, info, trace, warn};

pub struct BuilderOpts {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
}

impl Default for BuilderOpts {
    fn default() -> Self {
        Self {
            title: String::from("Nae App"),
            width: 800,
            height: 600,
            fullscreen: false,
        }
    }
}

pub trait BaseSystem {
    type Kind: BaseSystem;
    type Context2d: BaseContext2d;

    fn new(opts: BuilderOpts) -> Result<Self::Kind, String>;
    fn ctx2(&mut self) -> &mut Self::Context2d;
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
