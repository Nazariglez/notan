use std::path::PathBuf;

use crate::gl_manager::GlManager;
use notan_app::WindowConfig;
use notan_app::{CursorIcon, WindowBackend};
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event_loop::EventLoop;
use winit::window::Fullscreen::Borderless;
use winit::window::{
    CursorGrabMode, CursorIcon as WCursorIcon, Icon, Window, WindowBuilder, WindowLevel,
};

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
    title: String,
    use_touch_as_mouse: bool,
}

impl WindowBackend for WinitWindowBackend {
    fn capture_cursor(&self) -> bool {
        self.captured
    }

    fn cursor(&self) -> CursorIcon {
        self.cursor
    }

    fn dpi(&self) -> f64 {
        if cfg!(target_os = "macos") && !self.high_dpi {
            return 1.0;
        }

        self.scale_factor
    }

    fn id(&self) -> u64 {
        self.window().id().into()
    }

    fn is_always_on_top(&self) -> bool {
        self.is_always_on_top
    }

    fn is_fullscreen(&self) -> bool {
        self.window().fullscreen().is_some()
    }

    fn lazy_loop(&self) -> bool {
        self.lazy
    }

    fn mouse_passthrough(&mut self) -> bool {
        self.mouse_passthrough
    }

    fn position(&self) -> (i32, i32) {
        let position = self.window().outer_position().unwrap_or_default();
        (position.x, position.y)
    }

    fn request_frame(&mut self) {
        if self.lazy {
            self.window().request_redraw();
        }
    }

    fn screen_size(&self) -> (i32, i32) {
        self.window()
            .current_monitor()
            .map(|m| {
                let logical = m.size().to_logical::<f64>(self.scale_factor);
                (logical.width as _, logical.height as _)
            })
            .unwrap_or((0, 0))
    }

    fn set_always_on_top(&mut self, enabled: bool) {
        let level = if enabled {
            WindowLevel::AlwaysOnTop
        } else {
            WindowLevel::Normal
        };
        self.window().set_window_level(level);
        self.is_always_on_top = enabled;
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

    fn set_cursor_position(&mut self, x: u32, y: u32) {
        if let Err(e) = self.window().set_cursor_position(PhysicalPosition::new(x, y)) {
            log::error!("Error setting mouse cursor position to x: {0} y: {1} error: {2}", x, y, e);
        }
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        if enabled {
            let monitor = self.window().current_monitor();
            self.window().set_fullscreen(Some(Borderless(monitor)));
        } else {
            self.window().set_fullscreen(None);
        }
    }

    fn set_lazy_loop(&mut self, lazy: bool) {
        self.lazy = lazy;
        if !self.lazy {
            self.request_frame();
        }
    }

    fn set_mouse_passthrough(&mut self, pass_through: bool) {
        self.mouse_passthrough = pass_through;
        self.gl_manager.set_cursor_hittest(!pass_through).unwrap();
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.window()
            .set_outer_position(PhysicalPosition::new(x, y));
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.window()
            .set_inner_size(LogicalSize::new(width, height));
    }

    fn set_visible(&mut self, visible: bool) {
        if self.visible != visible {
            self.visible = visible;
            self.window().set_visible(visible);
        }
    }

    fn size(&self) -> (u32, u32) {
        let inner = self.window().inner_size();
        let logical = inner.to_logical::<f64>(self.scale_factor);
        (logical.width as _, logical.height as _)
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.window().set_title(&self.title);
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn set_touch_as_mouse(&mut self, enable: bool) {
        self.use_touch_as_mouse = enable;
    }

    fn touch_as_mouse(&self) -> bool {
        self.use_touch_as_mouse
    }
}

fn load_icon(path: &Option<PathBuf>, data: &Option<&'static [u8]>) -> Option<Icon> {
    match (path, data) {
        (Some(path), None) => Some(load_icon_from_path(path)), // Handle Path
        (None, Some(data)) => Some(load_icon_from_data(data)), // Handle Data
        (Some(_), Some(data)) => {
            // Handle User Passing Both
            log::warn!("Creating Icon from Data. You have set a path and data for your Icon, please choose only one!");
            Some(load_icon_from_data(data))
        }
        (None, None) => None,
    }
}

fn load_icon_from_path(path: &PathBuf) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn load_icon_from_data(data: &'static [u8]) -> Icon {
    let icon_data = image::load_from_memory(data)
        .expect("Failed to Create Icon from Data")
        .into_rgba8();

    let icon_width = icon_data.width();
    let icon_height = icon_data.height();
    let icon_rgba = icon_data.into_raw();

    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

impl WinitWindowBackend {
    pub(crate) fn new(config: WindowConfig, event_loop: &EventLoop<()>) -> Result<Self, String> {
        let level = if config.always_on_top {
            WindowLevel::AlwaysOnTop
        } else {
            WindowLevel::Normal
        };
        let mut builder = WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(LogicalSize::new(config.width, config.height))
            .with_maximized(config.maximized)
            .with_resizable(config.resizable)
            .with_transparent(config.transparent)
            .with_window_level(level)
            .with_visible(config.visible)
            .with_decorations(config.decorations)
            .with_window_icon(load_icon(
                &config.window_icon_path,
                &config.window_icon_data,
            ));

        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::WindowBuilderExtWindows;
            builder = builder.with_taskbar_icon(load_icon(
                &config.taskbar_icon_path,
                &config.taskbar_icon_data,
            ));
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

        let WindowConfig {
            lazy_loop,
            visible,
            high_dpi,
            title,
            mouse_passthrough,
            ..
        } = config;

        Ok(Self {
            gl_manager,
            scale_factor,
            lazy: lazy_loop,
            cursor: CursorIcon::Default,
            captured: false,
            visible,
            high_dpi,
            is_always_on_top: false,
            mouse_passthrough,
            title,
            use_touch_as_mouse: false,
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
