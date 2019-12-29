use super::System;
use nae_core::window::BaseWindow;
use nae_core::{BaseApp, BaseSystem};
use nae_glow::Context2d;
use winit;
use winit::{dpi::LogicalSize, ControlFlow, EventsLoop, WindowBuilder};

pub struct Window {
    win: winit::Window,
    title: String,
    width: i32,
    height: i32,
    fullscreen: bool,
}

impl Window {
    pub(crate) fn new(
        title: &str,
        width: i32,
        height: i32,
        event_loop: &EventsLoop,
    ) -> Result<Self, String> {
        let win = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(LogicalSize::new(width as f64, height as f64))
            .build(&event_loop)
            .map_err(|e| format!("{}", e))?;

        Ok(Self {
            width,
            height,
            title: title.to_string(),
            fullscreen: false,
            win,
        })
    }
}

impl BaseWindow for Window {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    fn title(&self) -> &str {
        &self.title
    }
}

pub fn run<A, F>(app: A, callback: F)
where
    A: BaseApp<System = System>,
    F: FnMut(&mut A) + 'static,
{
    let mut app = app;
    let mut cb = callback;
    let mut event_loop = app.system().event_loop.take().unwrap();

    event_loop.run_forever(move |event| {
        cb(&mut app);

        ControlFlow::Continue
    });
}
