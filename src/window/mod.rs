use glow::*;

//add #[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use crate::graphics;

pub struct Window {
    canvas: web_sys::HtmlCanvasElement,
    ctx: graphics::Context,
    //gl: glow::Context
}

impl Window {
    pub fn new() -> Window {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let ctx = graphics::Context::new(&canvas);

        Window {
            canvas: canvas,
            ctx: ctx
            //gl: glow::Context::from_webgl2_context(gl)
        }
    }
}
