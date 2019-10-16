use glow::*;

//add #[cfg(target_arch = "wasm32")]
use crate::graphics;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

pub struct Window {
    canvas: web_sys::HtmlCanvasElement,
    //    ctx: graphics::Context,
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

        //        let ctx = graphics::Context::new(&canvas).unwrap(); //TODO manage error

        Window {
            canvas: canvas,
            //            ctx: ctx, //gl: glow::Context::from_webgl2_context(gl)
        }
    }

    pub fn window(&self) -> &web_sys::HtmlCanvasElement {
        return &self.canvas;
    }

    //window_rect()? top_right, top_left, center, etc...
//    pub fn top_right(&self) -> (f32, f32) {
//        (self.canvas.width() as f32 * -0.5, self.canvas.height() as f32 * -0.5)
//    }
}
