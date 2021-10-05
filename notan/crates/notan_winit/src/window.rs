use glutin::window::Fullscreen::Borderless;
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use notan_app::WindowBackend;
use notan_app::WindowConfig;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct WinitWindowBackend {
    pub(crate) gl_ctx: ContextWrapper<PossiblyCurrent, Window>,
    pub(crate) scale_factor: f64,
    // win: WindowedContext<PossiblyCurrent>,
}

impl WindowBackend for WinitWindowBackend {
    fn set_size(&mut self, width: i32, height: i32) {
        self.window()
            .set_inner_size(LogicalSize::new(width, height));
    }

    fn size(&self) -> (i32, i32) {
        let inner = self.window().inner_size();
        let logical = inner.to_logical::<f64>(self.scale_factor);
        (logical.width as _, logical.height as _)
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        if enabled {
            let monitor = self.window().current_monitor();
            self.window().set_fullscreen(Some(Borderless(monitor)));
        } else {
            self.window().set_fullscreen(None);
        }
    }

    fn is_fullscreen(&self) -> bool {
        self.window().fullscreen().is_some()
    }

    fn dpi(&self) -> f64 {
        self.scale_factor
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

        let windowed_context = ContextBuilder::new()
            .with_vsync(config.vsync)
            .with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 3),
                opengles_version: (2, 0),
            })
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(config.multisampling)
            .build_windowed(builder, event_loop)
            .map_err(|e| format!("{}", e))?;

        let gl_ctx = unsafe { windowed_context.make_current().unwrap() };

        let monitor = gl_ctx.window().current_monitor();
        let scale_factor = monitor.as_ref().unwrap().scale_factor();
        if config.fullscreen {
            gl_ctx.window().set_fullscreen(Some(Borderless(monitor)));
        }

        Ok(Self {
            gl_ctx,
            scale_factor,
        })
    }

    pub(crate) fn window(&self) -> &Window {
        self.gl_ctx.window()
    }

    pub(crate) fn swap_buffers(&self) {
        self.gl_ctx.swap_buffers().unwrap();
    }
}
