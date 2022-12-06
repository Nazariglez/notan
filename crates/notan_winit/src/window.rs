use std::num::NonZeroU32;
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event_loop::EventLoop;
use winit::window::Fullscreen::Borderless;
use winit::window::{CursorGrabMode, CursorIcon as WCursorIcon, Window, WindowBuilder};
// use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use glutin::config::{Api, Config, ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, GlProfile, NotCurrentGlContextSurfaceAccessor, Version,
};
use glutin::display::{Display, GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface};
use glutin_winit::{ApiPrefence, DisplayBuilder};
use notan_app::WindowConfig;
use notan_app::{CursorIcon, WindowBackend};
use raw_window_handle::HasRawWindowHandle;

pub struct GlWindow {
    // XXX the surface must be dropped before the window.
    pub surface: Surface<WindowSurface>,

    pub window: Window,
}

impl GlWindow {
    pub fn new(window: Window, config: &Config) -> Self {
        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.raw_window_handle();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new()
            .with_srgb(Some(true))
            .build(
                raw_window_handle,
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            );

        let surface = unsafe {
            config
                .display()
                .create_window_surface(config, &attrs)
                .unwrap()
        };

        Self { window, surface }
    }
}

pub struct WinitWindowBackend {
    // pub(crate) gl_ctx: ContextWrapper<PossiblyCurrent, Window>,
    pub(crate) gl_display: Display,
    pub(crate) gl_win: GlWindow,
    pub(crate) scale_factor: f64,
    pub(crate) lazy: bool,
    cursor: CursorIcon,
    captured: bool,
    visible: bool,
    high_dpi: bool,
    is_always_on_top: bool,
    mouse_passthrough: bool,
}

impl WindowBackend for WinitWindowBackend {
    fn id(&self) -> u64 {
        self.window().id().into()
    }

    fn set_size(&mut self, width: i32, height: i32) {
        self.window()
            .set_inner_size(LogicalSize::new(width, height));
    }

    fn size(&self) -> (i32, i32) {
        let inner = self.window().inner_size();
        let logical = inner.to_logical::<f64>(self.scale_factor);
        (logical.width as _, logical.height as _)
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.window()
            .set_outer_position(PhysicalPosition::new(x, y));
    }

    fn position(&self) -> (i32, i32) {
        let position = self.window().outer_position().unwrap_or_default();
        (position.x, position.y)
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
            // self.gl_ctx.window().request_redraw();
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

    fn mouse_passthrough(&mut self) -> bool {
        self.mouse_passthrough
    }

    fn set_mouse_passthrough(&mut self, pass_through: bool) {
        self.mouse_passthrough = pass_through;

        // self.gl_ctx
        //     .window()
        //     .set_cursor_hittest(!pass_through)
        //     .unwrap();
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
            use winit::platform::macos::WindowBuilderExtMacOS;
            builder = builder.with_disallow_hidpi(!config.high_dpi);
        }

        if let Some((w, h)) = config.min_size {
            builder = builder.with_min_inner_size(LogicalSize::new(w, h));
        }

        if let Some((w, h)) = config.max_size {
            builder = builder.with_max_inner_size(LogicalSize::new(w, h));
        }

        let mut template = ConfigTemplateBuilder::new()
            .with_api(Api::GLES3)
            .with_transparency(config.transparent);

        if config.multisampling > 0 {
            template = template.with_multisampling(config.multisampling);
        }

        let (mut window, gl_config) = DisplayBuilder::new()
            .with_window_builder(Some(builder))
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|accum, conf| {
                        let next_srgb = conf.srgb_capable();
                        let next_transparency = conf.supports_transparency().unwrap_or(false);
                        let more_samples = conf.num_samples() > accum.num_samples();

                        // value of transparency for the priority check
                        let transparency_check = if config.transparent {
                            next_transparency
                        } else {
                            true
                        };

                        // priority 1: supports srgba, transparency and has more samples than current one
                        let full_support = next_srgb && transparency_check && more_samples;

                        // priority 2: we don't care about transparency if it's not supported by next config
                        let srgba_plus_samples = next_srgb && more_samples;

                        // priority 3: if it supports srgba is enough
                        let only_srgba = next_srgb;

                        // select the config in order of priority
                        let select_config = full_support || srgba_plus_samples || only_srgba;

                        if select_config {
                            conf
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .map_err(|e| e.to_string())?;

        let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(raw_window_handle);

        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(Some(Version::new(3, 0))))
            .build(raw_window_handle);

        let mut not_current_gl_context = Some(unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        });

        //GlWindow?
        let gl_window = GlWindow::new(window.unwrap(), &gl_config);
        // Make it current.
        let gl_context = not_current_gl_context
            .take()
            .unwrap()
            .make_current(&gl_window.surface)
            .unwrap();

        // Try setting vsync.
        if config.vsync {
            if let Err(res) = gl_window
                .surface
                .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
            {
                log::error!("Error setting vsync: {:?}", res);
            }
        }

        // let windowed_context = ContextBuilder::new()
        //     .with_vsync(config.vsync)
        //     .with_gl(glutin::GlRequest::GlThenGles {
        //         opengl_version: (3, 3),
        //         opengles_version: (3, 0),
        //     })
        //     .with_srgb(true)
        //     .with_gl_profile(glutin::GlProfile::Core)
        //     .with_multisampling(config.multisampling)
        //     .build_windowed(builder, event_loop)
        //     .map_err(|e| format!("{}", e))?;

        // let gl_ctx = unsafe { windowed_context.make_current().unwrap() };
        //
        // if template.mouse_passthrough {
        //     gl_ctx.window().set_cursor_hittest(false).unwrap();
        // }
        //
        // let monitor = gl_ctx.window().current_monitor();
        // let scale_factor = monitor.as_ref().map_or(1.0, |m| m.scale_factor());
        // if template.fullscreen {
        //     gl_ctx.window().set_fullscreen(Some(Borderless(monitor)));
        // }

        Ok(Self {
            // gl_ctx,
            gl_display,
            gl_win: gl_window,
            scale_factor: 1.0, //TODO
            lazy: false,       //template.lazy_loop,
            cursor: CursorIcon::Default,
            captured: false,
            visible: false,  //template.visible,
            high_dpi: false, //template.high_dpi,
            is_always_on_top: false,
            mouse_passthrough: false, //template.mouse_passthrough,
        })
    }

    pub(crate) fn window(&self) -> &Window {
        // self.gl_ctx.window()
        &self.gl_win.window
    }

    pub(crate) fn swap_buffers(&self) {
        // self.gl_ctx.swap_buffers().unwrap();
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
