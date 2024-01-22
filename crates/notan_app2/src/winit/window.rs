use super::utils::{cursor_id, win_id};
use notan_core::window::{CursorIcon, NotanWindow, WindowConfig, WindowId};
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, WindowHandle,
};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Fullscreen, Window as RawWindow, WindowBuilder};

pub struct Window {
    id: WindowId,
    raw: RawWindow,
    title: String,
    cursor: CursorIcon,
    transparent: bool,
    min_size: Option<(u32, u32)>,
    max_size: Option<(u32, u32)>,
}

impl Window {
    pub(crate) fn new(
        event_loop: &EventLoopWindowTarget<()>,
        attrs: WindowConfig,
    ) -> Result<Self, String> {
        let WindowConfig {
            size,
            min_size,
            max_size,
            position,
            resizable,
            title,
            fullscreen,
            maximized,
            visible,
            transparent,
        } = attrs;
        let mut builder = WindowBuilder::default()
            .with_title(&title)
            .with_resizable(resizable)
            .with_maximized(maximized)
            .with_transparent(transparent)
            .with_visible(visible);

        #[cfg(all(not(target_os = "ios"), not(target_os = "android")))]
        if let Some((w, h)) = size {
            builder = builder.with_inner_size(LogicalSize::new(w, h));
        }

        #[cfg(all(not(target_os = "ios"), not(target_os = "android")))]
        if let Some((w, h)) = min_size {
            builder = builder.with_min_inner_size(LogicalSize::new(w, h));
        }

        #[cfg(all(not(target_os = "ios"), not(target_os = "android")))]
        if let Some((w, h)) = max_size {
            builder = builder.with_max_inner_size(LogicalSize::new(w, h));
        }

        #[cfg(all(not(target_os = "ios"), not(target_os = "android")))]
        if let Some((x, y)) = position {
            builder = builder.with_position(PhysicalPosition::new(x, y));
        }

        let raw = builder.build(event_loop).map_err(|err| err.to_string())?;
        let id = win_id(raw.id());
        let cursor = CursorIcon::Default;
        let mut win = Window {
            id,
            raw,
            title,
            cursor,
            transparent,
            min_size: None,
            max_size: None,
        };
        if fullscreen {
            win.set_fullscreen(true);
        }
        Ok(win)
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.raw.window_handle()
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.raw.display_handle()
    }
}

impl NotanWindow for Window {
    fn id(&self) -> WindowId {
        self.id
    }

    fn physical_size(&self) -> (u32, u32) {
        let size = self.raw.inner_size();
        (size.width, size.height)
    }

    fn size(&self) -> (u32, u32) {
        let scale_factor = self.raw.scale_factor();
        let size = self.raw.inner_size().to_logical::<u32>(scale_factor);
        (size.width, size.height)
    }

    fn width(&self) -> u32 {
        let (w, _) = self.size();
        w
    }

    fn height(&self) -> u32 {
        let (_, h) = self.size();
        h
    }

    fn set_size(&mut self, width: u32, height: u32) {
        let _ = self.raw.request_inner_size(LogicalSize::new(width, height));
        // TODO it worth to log the result here?
    }

    fn scale(&self) -> f64 {
        self.raw.scale_factor()
    }

    fn position(&self) -> Result<(i32, i32), String> {
        let pos = self
            .raw
            .outer_position()
            .map_err(|err| err.to_string())?
            .to_logical::<i32>(self.scale());

        Ok(pos.into())
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.raw.set_outer_position(LogicalPosition::new(x, y));
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.raw.set_title(&self.title);
    }

    fn fullscreen(&self) -> bool {
        self.raw.fullscreen().is_some()
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        let mode = fullscreen.then(|| Fullscreen::Borderless(self.raw.current_monitor()));
        self.raw.set_fullscreen(mode);
    }

    fn request_focus(&mut self) {
        self.raw.focus_window();
    }

    fn has_focus(&self) -> bool {
        self.raw.has_focus()
    }

    fn set_cursor_icon(&mut self, cursor: CursorIcon) {
        if cursor != self.cursor {
            self.cursor = cursor;
            match cursor_id(cursor) {
                None => {
                    self.raw.set_cursor_visible(false);
                }
                Some(icon) => {
                    self.raw.set_cursor_visible(true);
                    self.raw.set_cursor_icon(icon);
                }
            }
        }
    }

    fn cursor(&self) -> CursorIcon {
        self.cursor
    }

    fn set_maximized(&mut self, maximized: bool) {
        self.raw.set_maximized(maximized);
    }

    fn maximized(&self) -> bool {
        self.raw.is_maximized()
    }

    fn set_minimized(&mut self, minimized: bool) {
        self.raw.set_minimized(minimized);
    }

    fn minimized(&self) -> bool {
        self.raw.is_minimized().unwrap_or(false)
    }

    fn set_visible(&mut self, visible: bool) {
        self.raw.set_visible(visible);
    }

    fn visible(&self) -> bool {
        self.raw.is_visible().unwrap_or(false)
    }

    fn set_transparent(&mut self, transparent: bool) {
        self.transparent = transparent;
        self.raw.set_transparent(transparent);
    }

    fn transparent(&self) -> bool {
        self.transparent
    }

    fn set_resizable(&mut self, resizable: bool) {
        self.raw.set_resizable(resizable);
    }

    fn resizable(&self) -> bool {
        self.raw.is_resizable()
    }

    fn set_min_size(&mut self, width: u32, height: u32) {
        self.min_size = Some((width, height));
        self.raw
            .set_min_inner_size(Some(LogicalSize::new(width, height)));
    }

    fn min_size(&self) -> Option<(u32, u32)> {
        self.min_size
    }

    fn set_max_size(&mut self, width: u32, height: u32) {
        self.max_size = Some((width, height));
        self.raw
            .set_max_inner_size(Some(LogicalSize::new(width, height)));
    }

    fn max_size(&self) -> Option<(u32, u32)> {
        self.max_size
    }

    fn request_redraw(&mut self) {
        self.raw.request_redraw();
    }
}
