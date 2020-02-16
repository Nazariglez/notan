#[cfg(not(feature = "sdl"))]
mod window_winit;

#[cfg(feature = "sdl")]
mod window_sdl;

use futures::{future, Future};
use nae_core::window::BaseWindow;
pub use nae_core::*;
pub use nae_glow::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use winit::event_loop::EventLoop;

pub(crate) trait ToNaeValue {
    type Kind;

    fn to_nae(&self) -> Self::Kind;
}

#[cfg(not(feature = "sdl"))]
pub use window_winit::*;

#[cfg(feature = "sdl")]
pub use window_sdl::*;

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
    events: EventIterator,

    #[cfg(not(feature = "sdl"))]
    pub(crate) event_loop: Option<EventLoop<()>>,
}

impl BaseSystem for System {
    type Kind = Self;
    type Context2d = Context2d;

    #[cfg(not(feature = "sdl"))]
    fn new(mut opts: BuilderOpts) -> Result<Self, String> {
        let event_loop = EventLoop::new();
        let win = Window::new(&opts, &event_loop)?;
        let ctx2 = Context2d::new(&win.win)?;
        Ok(Self {
            window: win,
            context2d: ctx2,
            event_loop: Some(event_loop),
            events: EventIterator::new(),
        })
    }

    #[cfg(feature = "sdl")]
    fn new(mut opts: BuilderOpts) -> Result<Self, String> {
        let win = Window::new(&opts)?;
        let ctx2 = Context2d::new(&win.win)?;
        Ok(Self {
            window: win,
            context2d: ctx2,
            events: EventIterator::new(),
        })
    }

    fn ctx2(&mut self) -> &mut Self::Context2d {
        &mut self.context2d
    }

    fn events(&mut self) -> &mut EventIterator {
        &mut self.events
    }

    fn width(&self) -> f32 {
        self.window.width() as _
    }

    fn height(&self) -> f32 {
        self.window.height() as _
    }

    fn dpi(&self) -> f32 {
        self.window.dpi()
    }

    fn set_fullscreen(&mut self, full: bool) {
        self.window.set_fullscreen(full);
    }

    fn fullscreen(&self) -> bool {
        self.window.fullscreen()
    }
}
