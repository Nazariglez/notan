use notan_core::window::{CursorIcon, NotanApp, NotanWindow, WindowId};
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};

pub struct Window {
    pub(crate) id: WindowId,
    pub(crate) size: (u32, u32),
    pub(crate) position: (i32, i32),
    pub(crate) title: String,
    pub(crate) cursor: CursorIcon,
    pub(crate) resizable: bool,
    pub(crate) min_size: Option<(u32, u32)>,
    pub(crate) max_size: Option<(u32, u32)>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            id: 0u64.into(),
            size: (0, 0),
            position: (0, 0),
            title: "Window".to_string(),
            cursor: CursorIcon::Default,
            resizable: false,
            min_size: None,
            max_size: None,
        }
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        unreachable!("Empty platform should not try to get RawWindowHandle");
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unreachable!("Empty platform should not try to get HasRawDisplayHandle");
    }
}

impl NotanWindow for Window {
    fn id(&self) -> WindowId {
        self.id
    }

    fn physical_size(&self) -> (u32, u32) {
        self.size
    }

    fn size(&self) -> (u32, u32) {
        self.size
    }

    fn width(&self) -> u32 {
        self.size.0
    }

    fn height(&self) -> u32 {
        self.size.1
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.size = (width, height);
    }

    fn scale(&self) -> f64 {
        1.0
    }

    fn position(&self) -> Result<(i32, i32), String> {
        Ok(self.position)
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    fn fullscreen(&self) -> bool {
        false
    }

    fn set_fullscreen(&mut self, _fullscreen: bool) {
        // no-op
    }

    fn request_focus(&mut self) {
        // no-op
    }

    fn has_focus(&self) -> bool {
        true
    }

    fn set_cursor_icon(&mut self, cursor: CursorIcon) {
        self.cursor = cursor;
    }

    fn cursor(&self) -> CursorIcon {
        self.cursor
    }

    fn set_maximized(&mut self, _maximized: bool) {
        // no-op
    }

    fn maximized(&self) -> bool {
        false
    }

    fn set_minimized(&mut self, _minimized: bool) {
        // no-op
    }

    fn minimized(&self) -> bool {
        false
    }

    fn set_visible(&mut self, _visible: bool) {
        // no-op
    }

    fn visible(&self) -> bool {
        false
    }

    fn set_transparent(&mut self, _transparent: bool) {
        // no-op
    }

    fn transparent(&self) -> bool {
        false
    }

    fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
    }

    fn resizable(&self) -> bool {
        self.resizable
    }

    fn set_min_size(&mut self, width: u32, height: u32) {
        self.min_size = Some((width, height));
    }

    fn min_size(&self) -> Option<(u32, u32)> {
        self.min_size
    }

    fn set_max_size(&mut self, width: u32, height: u32) {
        self.max_size = Some((width, height));
    }

    fn max_size(&self) -> Option<(u32, u32)> {
        self.max_size
    }

    fn request_redraw(&mut self) {}
}
