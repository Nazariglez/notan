mod app;
mod glm;
mod graphics;
pub mod math;
pub mod res;
mod window;

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

pub mod prelude {
    pub use super::app::*;
    pub use super::graphics::{color::*, *};
    pub use super::res::*;
    pub use derive::nae_start;
}
