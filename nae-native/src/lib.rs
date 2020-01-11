mod window;

use futures::{future, Future};
pub use nae_core::*;
pub use nae_glow::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
pub use window::*;
use winit::event_loop::EventLoop;

/// Read the content of a file and return a future with the content
pub fn load_file(path: &str) -> impl Future<Item = Vec<u8>, Error = String> {
    future::result(load_from_disk(path)).map_err(|e| e.to_string())
}

fn load_from_disk(path: impl AsRef<Path>) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = Vec::new();
    File::open(path)?.read_to_end(&mut buf)?;
    Ok(buf)
}

pub struct System {
    window: Window,
    context2d: Context2d,
    pub(crate) event_loop: Option<EventLoop<()>>,
    events: EventIterator,
}

impl BaseSystem for System {
    type Kind = Self;
    type Context2d = Context2d;

    fn new(mut opts: BuilderOpts) -> Result<Self, String> {
        let event_loop = EventLoop::new();
        let win = window::Window::new(&opts.title, opts.width, opts.height, &event_loop)?;
        let ctx2 = Context2d::new(&win.win)?;
        Ok(Self {
            window: win,
            context2d: ctx2,
            event_loop: Some(event_loop),
            events: EventIterator::new(),
        })
    }

    fn ctx2(&mut self) -> &mut Self::Context2d {
        &mut self.context2d
    }

    fn events(&mut self) -> &mut EventIterator {
        &mut self.events
    }
}
