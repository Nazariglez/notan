mod app;
mod input;
pub mod m2d;
mod random;
mod res;
pub mod tween;

pub use nae_derive::main;

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

pub use app::{init, init_with, App};

pub mod prelude {
    pub use super::app::*;
    pub use super::m2d;
    pub use super::random::*;
    pub use super::res::*;
    pub use super::tween;
    pub use backend::*;
    pub use nae_core::*;
}
