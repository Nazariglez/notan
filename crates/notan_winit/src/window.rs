use std::path::PathBuf;

use crate::gl_manager::GlManager;
use notan_app::WindowConfig;
use notan_app::{CursorIcon, WindowBackend};
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event_loop::EventLoop;
use winit::window::Fullscreen::Borderless;
use winit::window::{CursorGrabMode, CursorIcon as WCursorIcon, Icon, Window, WindowBuilder};

pub struct WinitWindowBackend {
    pub(crate) gl_manager: GlManager,
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

    fn set_size(&mut self, width: u32, height: u32) {
        self.window()
            .set_inner_size(LogicalSize::new(width, height));
    }

    fn size(&self) -> (u32, u32) {
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

    fn set_always_on_top(&mut self, enabled: bool) {
        self.window().set_always_on_top(enabled);
        self.is_always_on_top = enabled;
    }

    fn is_always_on_top(&self) -> bool {
        self.is_always_on_top
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
            self.window().request_redraw();
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

    fn set_mouse_passthrough(&mut self, pass_through: bool) {
        self.mouse_passthrough = pass_through;
        self.gl_manager.set_cursor_hittest(!pass_through).unwrap();
    }

    fn mouse_passthrough(&mut self) -> bool {
        self.mouse_passthrough
    }
}

fn load_icon(path: &Option<PathBuf>) -> Option<Icon> {
    match path {
        Some(path) => {
            let (icon_rgba, icon_width, icon_height) = {
                let image = image::open(path)
                    .expect("Failed to open icon path")
                    .into_rgba8();
                let (width, height) = image.dimensions();
                let rgba = image.into_raw();
                (rgba, width, height)
            };
            Some(Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon"))
        }
        None => None,
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
            .with_decorations(config.decorations)
            .with_window_icon(load_icon(&config.window_icon_path));

        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::WindowBuilderExtWindows;
            builder = builder.with_taskbar_icon(load_icon(&config.taskbar_icon_path));
        }

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

        let gl_manager = GlManager::new(builder, event_loop, &config)?;

        // Try setting vsync.
        if let Err(e) = gl_manager.enable_vsync(config.vsync) {
            // Should we send up the error if vsync fails?
            // how about if drivers invalidates the vsync option?
            // I think that the app should run no matter if vsync
            // is enabled or not
            log::error!("Error setting vsync to {}: {:?}", config.vsync, e);
        }

        if config.mouse_passthrough {
            gl_manager.set_cursor_hittest(false)?;
        }

        let scale_factor = gl_manager.scale_factor();
        if config.fullscreen {
            gl_manager.set_fullscreen(config.fullscreen);
        }

        Ok(Self {
            gl_manager,
            scale_factor,
            lazy: config.lazy_loop,
            cursor: CursorIcon::Default,
            captured: false,
            visible: config.visible,
            high_dpi: config.high_dpi,
            is_always_on_top: false,
            mouse_passthrough: config.mouse_passthrough,
        })
    }

    pub(crate) fn window(&self) -> &Window {
        &self.gl_manager.window
    }

    pub(crate) fn swap_buffers(&self) {
        self.gl_manager.swap_buffers().unwrap();
    }

    pub(crate) fn resize(&self, width: u32, height: u32) {
        self.gl_manager.resize(width, height);
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
