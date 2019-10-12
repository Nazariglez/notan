use glow::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

pub mod shaders;
pub mod renderer;

pub type GlContext = Rc<glow::Context>;
enum Driver {
    WebGL,
    WebGL2,
    OpenGL,
    OpenGLES,
    Metal,
    Dx11,
    Dx12,
    Vulkan,
}

pub struct Context {
    gl: GlContext,
    driver: Driver,
}

impl Context {
    pub fn new(win: &web_sys::HtmlCanvasElement) -> Result<Context, String> {
        let (ctx, driver) = create_gl_context(win)?;

        unsafe {
            ctx.clear_color(0.1, 0.2, 0.3, 1.0);
            ctx.clear(glow::COLOR_BUFFER_BIT);
        }

        Ok(Context {
            gl: ctx,
            driver: driver,
        })
    }
}

fn create_gl_context(win: &web_sys::HtmlCanvasElement) -> Result<(GlContext, Driver), String> {
    if let Ok(ctx) = create_webgl2_context(win) {
        return Ok((ctx, Driver::WebGL2));
    }

    let ctx = create_webgl_context(win)?;
    Ok((ctx, Driver::WebGL))
}

fn create_webgl_context(win: &web_sys::HtmlCanvasElement) -> Result<GlContext, String> {
    //TODO manage errors
    let gl = win
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGlRenderingContext>()
        .unwrap();

    let ctx = Rc::new(glow::Context::from_webgl1_context(gl));
    Ok(ctx)
}

fn create_webgl2_context(win: &web_sys::HtmlCanvasElement) -> Result<GlContext, String> {
    //TODO manage errors
    let gl = win
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();

    let ctx = Rc::new(glow::Context::from_webgl2_context(gl));
    Ok(ctx)
}
