use crate::window::WebWindowBackend;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, Window};

pub fn fullscreen_callack(window: &WebWindowBackend) -> Rc<RefCell<dyn Fn()>> {
    unimplemented!()
}

pub fn request_animation_frame(win: &Window, f: &Closure<dyn FnMut()>) -> i32 {
    win.request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

pub fn get_or_create_canvas(doc: &Document, canvas_id: &str) -> Result<HtmlCanvasElement, String> {
    let canvas = match doc.get_element_by_id(canvas_id) {
        Some(c) => c,
        None => {
            let c = doc
                .create_element("canvas")
                .map_err(|e| format!("{:?}", e))?;

            let body = doc
                .body()
                .ok_or("body doesn't exists on document.".to_string())?;
            body.append_child(&c).map_err(|e| format!("{:?}", e))?;

            c.set_id(canvas_id);
            c
        }
    };
    canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|e| format!("{:?}", e))
}
