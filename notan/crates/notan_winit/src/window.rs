use notan_app::WindowBackend;
// use winit::{WindowedContext, PossiblyCurrent};
use glutin::window::Fullscreen::Borderless;
use notan_app::config::WindowConfig;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
// use glutin::{WindowedContext, PossiblyCurrent};

pub struct WinitWindowBackend {
    pub(crate) window: Window,
    // win: WindowedContext<PossiblyCurrent>,
}

impl WindowBackend for WinitWindowBackend {
    fn set_size(&mut self, width: i32, height: i32) {
        self.window.set_inner_size(LogicalSize::new(width, height));
    }

    fn size(&self) -> (i32, i32) {
        let inner = self.window.inner_size();
        (inner.width as _, inner.height as _)
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        if enabled {
            let monitor = self.window.current_monitor();
            self.window.set_fullscreen(Some(Borderless(monitor)));
        } else {
            self.window.set_fullscreen(None);
        }
    }

    fn is_fullscreen(&self) -> bool {
        self.window.fullscreen().is_some()
    }
}

impl WinitWindowBackend {
    pub(crate) fn new(config: WindowConfig, event_loop: &EventLoop<()>) -> Result<Self, String> {
        let mut builder = WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(LogicalSize::new(config.width, config.height))
            .with_maximized(config.maximized)
            .with_resizable(config.resizable);

        if let Some((w, h)) = config.min_size {
            builder = builder.with_min_inner_size(LogicalSize::new(w, h));
        }

        if let Some((w, h)) = config.max_size {
            builder = builder.with_max_inner_size(LogicalSize::new(w, h));
        }

        let mut window = builder.build(&event_loop).map_err(|e| e.to_string())?;

        if config.fullscreen {
            let monitor = window.current_monitor();
            window.set_fullscreen(Some(Borderless(monitor)));
        }

        Ok(Self { window })
    }
}
