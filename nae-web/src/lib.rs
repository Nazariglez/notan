mod window;

use nae_core::graphics::BaseContext2d;
use nae_core::*;
use nae_glow::*;
use window::*;

pub struct App {
    window: Window,
    context2d: Context2d,
}

impl BaseApp for App {
    type Kind = Self;
    type Graphics = Context2d;

    fn build<S>(mut opts: BuilderOpts<S, App>) -> Result<(), String> {
        let win = window::Window::new(&opts.title, opts.width, opts.height)?;
        let ctx2 = Context2d::new(&win.canvas)?;
        let mut app = App {
            window: win,
            context2d: ctx2,
        };

        let mut state = (opts.state_cb)(&mut app);
        let draw_cb = opts.draw_callback.take().unwrap_or(|_, _| {});
        let update_cb = opts.update_callback.take().unwrap_or(|_, _| {});
        let start_cb = opts.start_callback.take().unwrap_or(|_, _| {});

        start_cb(&mut app, &mut state);

        Ok(())
    }

    fn graphics(&mut self) -> &mut Self::Graphics {
        &mut self.context2d
    }
}
