use glutin::dpi::LogicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::Fullscreen::Borderless;
use glutin::window::{CursorGrabMode, CursorIcon as WCursorIcon, Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use notan_app::WindowConfig;
use notan_app::{CursorIcon, WindowBackend};

pub struct WinitWindowBackend {
    pub(crate) gl_ctx: ContextWrapper<PossiblyCurrent, Window>,
    pub(crate) scale_factor: f64,
    pub(crate) lazy: bool,
    cursor: CursorIcon,
    captured: bool,
    visible: bool,
    high_dpi: bool,
    is_always_on_top: bool,
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

    fn set_always_on_top(&mut self, enabled: bool) {
        self.window().set_always_on_top(enabled);
        self.is_always_on_top = enabled;
    }

    fn is_always_on_top(&self) -> bool {
        self.is_always_on_top
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
        if cfg!(target_os = "macos") && !self.high_dpi {
            return 1.0;
        }

        self.scale_factor
    }

    fn set_lazy_loop(&mut self, lazy: bool) {
        self.lazy = lazy;
        if !self.lazy {
            self.request_frame();
        }
    }

    fn lazy_loop(&self) -> bool {
        self.lazy
    }

    fn request_frame(&mut self) {
        if self.lazy {
            self.gl_ctx.window().request_redraw();
        }
    }

    fn set_cursor(&mut self, cursor: CursorIcon) {
        if cursor != self.cursor {
            self.cursor = cursor;
            match winit_cursor(cursor) {
                None => {
                    self.window().set_cursor_visible(false);
                }
                Some(icon) => {
                    self.window().set_cursor_visible(true);
                    self.window().set_cursor_icon(icon);
                }
            }
        }
    }

    fn cursor(&self) -> CursorIcon {
        self.cursor
    }

    fn set_capture_cursor(&mut self, capture: bool) {
        if capture == self.captured {
            return;
        }

        let is_macos = cfg!(target_os = "macos");
        if is_macos {
            log::warn!("Capture cursor is not implemented yet on MacOS. Awaiting for Winit to implement it.");
            return;
        }

        let mode = if capture {
            CursorGrabMode::Confined
        } else {
            CursorGrabMode::None
        };

        if self.window().set_cursor_grab(mode).is_ok() {
            self.captured = capture;
        }
    }

    fn capture_cursor(&self) -> bool {
        self.captured
    }

    fn set_visible(&mut self, visible: bool) {
        if self.visible != visible {
            self.visible = visible;
            self.window().set_visible(visible);
        }
    }

    fn visible(&self) -> bool {
        self.visible
    }
}

impl WinitWindowBackend {
    pub(crate) fn new(config: WindowConfig, event_loop: &EventLoop<()>) -> Result<Self, String> {
        let mut builder = WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(LogicalSize::new(config.width, config.height))
            .with_maximized(config.maximized)
            .with_resizable(config.resizable)
            .with_transparent(config.transparent)
            .with_always_on_top(config.always_on_top)
            .with_visible(config.visible)
            .with_decorations(config.decorations);

        #[cfg(target_os = "macos")]
        {
            use glutin::platform::macos::WindowBuilderExtMacOS;
            builder = builder.with_disallow_hidpi(!config.high_dpi);
        }

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
                opengles_version: (3, 0),
            })
            .with_srgb(true)
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(config.multisampling)
            .build_windowed(builder, event_loop)
            .map_err(|e| format!("{}", e))?;

        let gl_ctx = unsafe { windowed_context.make_current().unwrap() };

        let monitor = gl_ctx.window().current_monitor();
        let scale_factor = monitor.as_ref().map_or(1.0, |m| m.scale_factor());
        if config.fullscreen {
            gl_ctx.window().set_fullscreen(Some(Borderless(monitor)));
        }

        Ok(Self {
            gl_ctx,
            scale_factor,
            lazy: config.lazy_loop,
            cursor: CursorIcon::Default,
            captured: false,
            visible: config.visible,
            high_dpi: config.high_dpi,
            is_always_on_top: false,
        })
    }

    pub(crate) fn window(&self) -> &Window {
        self.gl_ctx.window()
    }

    pub(crate) fn swap_buffers(&self) {
        self.gl_ctx.swap_buffers().unwrap();
    }
}

fn winit_cursor(cursor: CursorIcon) -> Option<WCursorIcon> {
    Some(match cursor {
        CursorIcon::None => return None,
        CursorIcon::Default => WCursorIcon::Default,
        CursorIcon::ContextMenu => WCursorIcon::ContextMenu,
        CursorIcon::Help => WCursorIcon::Help,
        CursorIcon::PointingHand => WCursorIcon::Hand,
        CursorIcon::Progress => WCursorIcon::Progress,
        CursorIcon::Wait => WCursorIcon::Wait,
        CursorIcon::Cell => WCursorIcon::Cell,
        CursorIcon::Crosshair => WCursorIcon::Crosshair,
        CursorIcon::Text => WCursorIcon::Text,
        CursorIcon::VerticalText => WCursorIcon::VerticalText,
        CursorIcon::Alias => WCursorIcon::Alias,
        CursorIcon::Copy => WCursorIcon::Copy,
        CursorIcon::Move => WCursorIcon::Move,
        CursorIcon::NoDrop => WCursorIcon::NoDrop,
        CursorIcon::NotAllowed => WCursorIcon::NotAllowed,
        CursorIcon::Grab => WCursorIcon::Grab,
        CursorIcon::Grabbing => WCursorIcon::Grabbing,
        CursorIcon::AllScroll => WCursorIcon::AllScroll,
        CursorIcon::ResizeHorizontal => WCursorIcon::EwResize,
        CursorIcon::ResizeNeSw => WCursorIcon::NeswResize,
        CursorIcon::ResizeNwSe => WCursorIcon::NwseResize,
        CursorIcon::ResizeVertical => WCursorIcon::NsResize,
        CursorIcon::ZoomIn => WCursorIcon::ZoomIn,
        CursorIcon::ZoomOut => WCursorIcon::ZoomOut,
        CursorIcon::ResizeEast => WCursorIcon::EResize,
        CursorIcon::ResizeSouthEast => WCursorIcon::SeResize,
        CursorIcon::ResizeSouth => WCursorIcon::SResize,
        CursorIcon::ResizeSouthWest => WCursorIcon::SwResize,
        CursorIcon::ResizeWest => WCursorIcon::WResize,
        CursorIcon::ResizeNorthWest => WCursorIcon::NwResize,
        CursorIcon::ResizeNorth => WCursorIcon::NResize,
        CursorIcon::ResizeNorthEast => WCursorIcon::NeResize,
        CursorIcon::ResizeColumn => WCursorIcon::ColResize,
        CursorIcon::ResizeRow => WCursorIcon::RowResize,
    })
}
