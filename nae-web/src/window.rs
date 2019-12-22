use nae_core::window::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub struct Window {
    pub(crate) canvas: HtmlCanvasElement,
    title: String,
    width: i32,
    height: i32,
    fullscreen: bool,
}

impl Window {
    pub(crate) fn new(title: &str, width: i32, height: i32) -> Result<Self, String> {
        let win = web_sys::window().ok_or(String::from("Can't access window dom object."))?;

        let canvas = win
            .document()
            .ok_or("Can't access document dom object ")?
            .get_element_by_id("nae_canvas")
            .ok_or("Can't get the element HtmlCanvasElement#nae_canvas")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|e| e.to_string())?;

        Ok(Self {
            title: title.to_string(),
            canvas,
            width,
            height,
            fullscreen: false,
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
}
