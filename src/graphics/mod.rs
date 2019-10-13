use self::shaders::ColorBatcher;
use color::Color;
use glow::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use crate::graphics::shaders::Shader;

pub mod color;
pub mod shaders;

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

pub struct RenderTarget {
    fbo: glow::WebFramebufferKey,
    width: i32,
    height: i32,
}

pub struct Transform {

}

impl Transform {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Context {
    gl: GlContext,
    driver: Driver,
    color_batcher: shaders::ColorBatcher,
    is_drawing: bool,
    color: Color,
    alpha: f32,
    render_target: Option<RenderTarget>,
    shader: Option<Shader>,
    transform: Transform,
    width: i32,
    height: i32
}

impl Context {
    pub fn new(win: &web_sys::HtmlCanvasElement) -> Result<Context, String> {
        let width = win.width() as i32;
        let height = win.height() as i32;
        let (gl, driver) = create_gl_context(win)?;
        let color_batcher = ColorBatcher::new(&gl, width, height)?;

        Ok(Context {
            gl,
            driver,
            color_batcher,
            is_drawing: false,
            color: Color::White,
            alpha: 1.0,
            render_target: None,
            shader: None,
            transform: Transform::new(),
            width,
            height
        })
    }

    pub fn begin(&mut self, clear_color: Option<color::Color>) {
        if self.is_drawing {
            return;
        }
        self.is_drawing = true;

        unsafe {
            let (fbo, ww, hh) = if let Some(rt) = &self.render_target {
                (Some(rt.fbo), rt.width, rt.height)
            } else {
                (None, self.width, self.height)
            };

            self.gl.bind_framebuffer(glow::FRAMEBUFFER, fbo);
            self.gl.viewport(0, 0, ww, hh);

            if let Some(c) = clear_color {
                let (r, g, b, a) = c.to_rgba();
                self.gl.clear_color(r, g, b, a);
                self.gl.clear(glow::COLOR_BUFFER_BIT);
            }
        }
    }

    pub fn end(&mut self) {
        if !self.is_drawing {
            return;
        }
        self.is_drawing = false;
        self.flush();
    }

    pub fn flush(&mut self) {}
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
