pub mod math;
mod app;
mod window;
pub mod res;
mod graphics;
mod glm;

pub use app::init;

pub mod prelude {
    pub use super::app::*;
    pub use super::res::*;
    pub use super::graphics::{color::*, *};
}