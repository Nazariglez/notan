use crate::utils::get_or_create_canvas;
use notan_app::WindowBackend;
use web_sys::{Document, HtmlCanvasElement, Window};

pub struct WebWindowBackend {
    pub canvas: HtmlCanvasElement,
    pub window: Window,
    pub document: Document,
}

impl WebWindowBackend {
    pub fn new() -> Result<Self, String> {
        let window = web_sys::window().ok_or(String::from("Can't access window dom object."))?;
        let document = window
            .document()
            .ok_or("Can't access document dom object ")?;
        let canvas = get_or_create_canvas(&document, "notan_canvas")?;
        Ok(Self {
            window,
            document,
            canvas,
        })
    }
}

impl WindowBackend for WebWindowBackend {
    fn set_size(&mut self, width: i32, height: i32) {
        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
    }

    fn size(&self) -> (i32, i32) {
        (self.canvas.client_width(), self.canvas.client_height())
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        unimplemented!()
    }

    fn is_fullscreen(&self) -> bool {
        self.document.fullscreen()
    }
}

unsafe impl Send for WebWindowBackend {}
unsafe impl Sync for WebWindowBackend {}
