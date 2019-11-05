mod app;
mod glm;
mod graphics;
pub mod math;
pub mod res;
mod window;

pub use app::{init, with_state};

pub mod prelude {
    pub use super::app::*;
    pub use super::graphics::{color::*, *};
    pub use super::res::*;
    pub use derive::nae_start;
}
