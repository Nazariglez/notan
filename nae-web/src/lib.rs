mod file;
mod window;

use file::*;
use nae_core::graphics::BaseContext2d;
use nae_core::*;
use nae_glow::*;
use window::*;

pub struct System {
    window: Window,
    context2d: Context2d,
}

impl BaseSystem for System {
    type Kind = Self;
    type Context2d = Context2d;

    fn new(mut opts: BuilderOpts) -> Result<Self, String> {
        let win = window::Window::new(&opts.title, opts.width, opts.height)?;
        let ctx2 = Context2d::new(&win.canvas)?;
        Ok(Self {
            window: win,
            context2d: ctx2,
        })
    }

    fn ctx2(&mut self) -> &mut Self::Context2d {
        &mut self.context2d
    }
}
