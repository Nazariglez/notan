pub mod graphics;
pub mod logger;
pub mod math;
pub mod resources;
pub mod window;

use crate::graphics::BaseContext2d;
pub use logger::{debug, error, info, trace, warn};

pub struct BuilderOpts<S, A>
where
    S: 'static,
    A: BaseApp,
{
    pub state_cb: fn(&mut A) -> S,
    pub draw_callback: Option<fn(&mut A, &mut S)>,
    pub update_callback: Option<fn(&mut A, &mut S)>,
    pub start_callback: Option<fn(&mut A, &mut S)>,
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
}

impl<A> Default for BuilderOpts<(), A>
where
    A: BaseApp,
{
    fn default() -> Self {
        Self {
            state_cb: |_| {},
            draw_callback: None,
            update_callback: None,
            start_callback: None,
            title: String::from("Nae App"),
            width: 800,
            height: 600,
            fullscreen: false,
        }
    }
}

pub trait BaseApp {
    type Kind: BaseApp;
    type Graphics: BaseContext2d;

    fn build<S>(opts: BuilderOpts<S, Self::Kind>) -> Result<(), String>;
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
