use super::System;
use glutin::{dpi::LogicalSize, ContextBuilder, PossiblyCurrent, WindowedContext};
use nae_core::window::BaseWindow;
use nae_core::{BaseApp, BaseSystem};
use nae_glow::Context2d;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub struct Window {
    pub(crate) win: WindowedContext<PossiblyCurrent>,
    title: String,
    width: i32,
    height: i32,
    fullscreen: bool,
    dpi: f32,
}

impl Window {
    pub(crate) fn new(
        title: &str,
        width: i32,
        height: i32,
        event_loop: &EventLoop<()>,
    ) -> Result<Self, String> {
        let win_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width as f64, height as f64));

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

        let win = unsafe { win_ctx.make_current().unwrap() };
        let dpi = win.window().scale_factor() as f32;

        Ok(Self {
            width,
            height,
            title: title.to_string(),
            fullscreen: false,
            win,
            dpi,
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

    fn dpi(&self) -> f32 {
        self.dpi
    }
}

pub fn run<A, F>(mut app: A, mut update: F)
//TODO new callback for draw
where
    A: BaseApp<System = System> + 'static,
    F: FnMut(&mut A) + 'static,
{
    let mut event_loop = app.system().event_loop.take().unwrap();
    let mut running = true;
    event_loop.run(move |event, target, mut control| {
        if !running {
            return;
        }
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    running = false;
                    *control = ControlFlow::Exit;
                    return;
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => {
                    println!("scale_factor: {} {:?}", scale_factor, new_inner_size);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                update(&mut app);
            }
            _ => {}
        }

        let mut time = Instant::now();
        time = time + Duration::from_secs_f32(1.0 / 60.0);
        *control = ControlFlow::WaitUntil(time);
        //            *control = ControlFlow::Poll;
    });
}
