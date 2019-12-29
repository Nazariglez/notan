use super::System;
use glutin::{
    dpi::LogicalSize, ContextBuilder, ControlFlow, Event, EventsLoop, PossiblyCurrent,
    WindowBuilder, WindowEvent, WindowedContext,
};
use nae_core::window::BaseWindow;
use nae_core::{BaseApp, BaseSystem};
use nae_glow::Context2d;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;

pub struct Window {
    pub(crate) win: WindowedContext<PossiblyCurrent>,
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
        let win_builder = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(LogicalSize::new(width as f64, height as f64));

        let win_ctx = ContextBuilder::new()
            .with_vsync(true)
            .with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 3),
                opengles_version: (2, 0),
            })
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(8)
            .build_windowed(win_builder, event_loop)
            .map_err(|e| format!("{}", e))?;

        let win_ctx = unsafe { win_ctx.make_current().unwrap() };

        Ok(Self {
            width,
            height,
            title: title.to_string(),
            fullscreen: false,
            win: win_ctx,
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

    let running = Rc::new(RefCell::new(true));
    loop {
        let is_running = running.clone();
        event_loop.poll_events(move |event| match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("hello");
                    *is_running.borrow_mut() = false;
                }
                _ => {}
            },
            _ => {}
        });

        cb(&mut app);

        if !*running.borrow() {
            break;
        }
    }
}
