use self::shaders::ColorBatcher;
use crate::graphics::shaders::Shader;
use color::Color;
use glow::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

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

use crate::glm;

//TODO use generic to be able to use with Mat2, Mat3, Mat4
pub struct Transform(Vec<glm::Mat3>);

impl Transform {
    pub fn new() -> Self {
        Self(vec![glm::Mat3::identity()])
    }

    pub fn push(&mut self, matrix: glm::Mat3) {
        self.0.push(matrix);
    }

    pub fn pop(&mut self) {
        if self.0.len() <= 1 {
            return;
        }
        self.0.pop();
    }

    pub fn matrix(&self) -> &glm::Mat3 {
        &self.0[self.0.len() - 1]
    }
}

pub struct DrawData {
    alpha: f32,
    color: Color,
    shader: Option<Shader>,
    transform: Transform,
    width: i32,
    height: i32,
}

impl DrawData {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            alpha: 1.0,
            shader: None,
            transform: Transform::new(),
            color: Color::White,
        }
    }

    pub fn set_color(&mut self, color:Color) {
        self.color = color;
    }
}

pub struct Context {
    gl: GlContext,
    driver: Driver,
    color_batcher: shaders::ColorBatcher,
    is_drawing: bool,
    render_target: Option<RenderTarget>,
    data: Rc<DrawData>,
}

impl Context {
    pub fn new(win: &web_sys::HtmlCanvasElement) -> Result<Context, String> {
        let width = win.width() as i32;
        let height = win.height() as i32;
        let (gl, driver) = create_gl_context(win)?;

        let data = Rc::new(DrawData::new(width, height));
        let color_batcher = ColorBatcher::new(&gl, data.clone())?;

        //2d
        unsafe {
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        }

        Ok(Context {
            data,
            gl,
            driver,
            color_batcher,
            is_drawing: false,
            render_target: None,
        })
    }

    pub fn width(&self) -> i32 {
        self.data.width
    }

    pub fn height(&self) -> i32 {
        self.data.height
    }

    pub fn set_color(&mut self, color: Color) {
        self.data.set_color(color);
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
                (None, self.width(), self.height())
            };

            self.gl.bind_framebuffer(glow::FRAMEBUFFER, fbo);
            self.gl.viewport(0, 0, ww, hh);

            if let Some(c) = clear_color {
                let (r, g, b, a) = c.to_rgba();
                self.gl.clear_color(r, g, b, a);
                self.gl.clear(glow::COLOR_BUFFER_BIT);
            }
        }

        self.color_batcher.begin();
    }

    pub fn end(&mut self) {
        if !self.is_drawing {
            return;
        }
        self.is_drawing = false;
        self.flush();
    }

    pub fn flush(&mut self) {
        self.flush_color();
    }

    fn flush_color(&mut self) {
        self.color_batcher.flush(&self.gl);
    }

    fn draw_color(&mut self, vertex: &[f32]) {
        self.color_batcher.draw(&self.gl, vertex, None);
    }

    pub fn fill_triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.draw_color(&[x1, y1, x2, y2, x3, y3]);
    }

    pub fn fill_rect(&mut self, x:f32, y:f32, width:f32, height:f32) {
        let x2 = x + width;
        let y2 = y + height;
        let vertices = [
            x, y,
            x2, y,
            x, y2,
            x, y2,
            x2, y,
            x2, y2
        ];

        self.draw_color(&vertices);
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
