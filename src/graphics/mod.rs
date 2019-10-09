use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

pub struct Context {
    gl: glow::Context
}

impl Context {
    pub fn new(win: &web_sys::HtmlCanvasElement) -> Context {
        let gl = win.get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();

        gl.clear_color(0.1, 0.2, 0.3, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);

        Context {
            gl: glow::Context::from_webgl2_context(gl)
        }
    }
}