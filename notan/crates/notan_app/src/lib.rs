pub mod empty;
pub mod prelude;

mod app;
mod backend;
mod builder;

mod events;
pub mod keyboard;
pub mod mouse;

pub use app::*;
pub use backend::*;
pub use events::*;

pub use builder::*;

pub trait AppConfig<B: Backend, S> {
    fn apply(&self, builder: &mut AppBuilder<S, B>);
}

pub struct WindowConfig {
    pub cosas: i32,
}

impl<B: Backend, S> AppConfig<B, S> for WindowConfig {
    fn apply(&self, builder: &mut AppBuilder<S, B>) {
        builder.window = "NOP!".to_string();
    }
}
