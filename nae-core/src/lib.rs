pub mod graphics;
pub mod log;
pub mod math;
pub mod resources;
pub mod window;

pub use graphics::*;
pub use resources::*;

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

pub trait BaseApp {
    type System: BaseSystem;
    fn system(&mut self) -> &mut Self::System;
}

pub trait BaseSystem {
    type Kind: BaseSystem;
    type Context2d: BaseContext2d;

    fn new(opts: BuilderOpts) -> Result<Self::Kind, String>;
    fn ctx2(&mut self) -> &mut Self::Context2d;
    fn swap_buffers(&mut self);
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
