mod file;
mod window;

pub use file::*;
use nae_core::graphics::BaseContext2d;
pub use nae_core::*;
pub use nae_glow::*;
use std::panic;
pub use window::*;

pub struct System {
    window: Window,
    context2d: Context2d,
    events: EventIterator,
    mouse_ctx: Option<MouseContext>,
    keyboard_ctx: Option<KeyboardContext>,
}

impl BaseSystem for System {
    type Kind = Self;
    type Context2d = Context2d;

    fn new(mut opts: BuilderOpts) -> Result<Self, String> {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let win = window::Window::new(&opts)?;
        let ctx2 = Context2d::new(&win.canvas)?;
        Ok(Self {
            window: win,
            context2d: ctx2,
            events: EventIterator::new(),
            mouse_ctx: None,
            keyboard_ctx: None,
        })
    }

    fn ctx2(&mut self) -> &mut Self::Context2d {
        &mut self.context2d
    }

    fn events(&mut self) -> &mut EventIterator {
        &mut self.events
    }
}
