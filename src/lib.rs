mod app;
//mod glm;
mod graphics;
pub mod math;
pub mod res;
mod window;

pub use nae_derive::main;
pub use nae_extras as extras;

/*
  TODO think about a plugin trait?
    something like .plugins(vec![Tween])
    and:
        trait Plugin {
            pre_start_hook: Option<fn(&mut App, &mut State, plugin: P) -> Result<(), String>>,
            and post_start, pre_update, post_update, etc...
        }
    ----
    this allow to alter the event cycle without change the original code
*/

pub use app::{init, with_state};

pub fn log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}

pub mod prelude {
    pub use super::app::*;
    pub use super::graphics::{color::*, shader::*, *};
    pub use super::log;
    pub use super::res::*;
    pub use nae_core::*;
}
