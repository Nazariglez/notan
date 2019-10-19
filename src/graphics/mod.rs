use self::shaders::ColorBatcher;
use crate::graphics::shaders::Shader;
use color::Color;
use glow::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use lyon::lyon_tessellation as tess;
use tess::basic_shapes::{fill_circle, stroke_circle};
use rayon::prelude::*;

pub mod color;
pub mod shaders;

pub type GlContext = Rc<glow::Context>;
enum Driver {
    WebGL,
    WebGL2,
    //    OpenGL,
    //    OpenGLES,
    //    Metal,
    //    Dx11,
    //    Dx12,
    //    Vulkan,
}

//TODO check this nannout beatiful API https://github.com/nannou-org/nannou/blob/master/examples/simple_draw.rs

pub struct RenderTarget {
    fbo: glow::WebFramebufferKey,
    width: i32,
    height: i32,
}

use crate::{glm, log};
use nalgebra_glm::mat4_to_mat3;
use lyon::lyon_tessellation::debugger::DebuggerMsg::Point;
use lyon::lyon_tessellation::{VertexBuffers, BuffersBuilder, StrokeOptions};

//TODO use generic to be able to use with Mat2, Mat3, Mat4
pub struct Transform(Vec<glm::Mat3>);

impl Transform {
    pub fn new() -> Self {
        Self(vec![glm::Mat3::identity()])
    }

    pub fn push(&mut self, matrix: glm::Mat3) {
        self.0.push(self.matrix() * matrix);
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
    projection: glm::Mat3,
}

impl DrawData {
    pub fn new(width: i32, height: i32) -> Self {
        let projection = get_projection(width, height);
        Self {
            width,
            height,
            alpha: 1.0,
            shader: None,
            transform: Transform::new(),
            color: Color::White,
            projection,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.projection = get_projection(self.width, self.height);
    }
}

fn get_projection(width: i32, height: i32) -> glm::Mat3 {
    //    mat4_to_mat3(&glm::ortho(
    //        0.0,
    //        width as f32,
    //        0.0,
    //        height as f32 * -1.0,
    //        -1.0,
    //        1.0)
    //    )
    let w = width as f32;
    let h = height as f32;
    glm::mat3(2.0 / w, 0.0, -1.0, 0.0, -2.0 / h, 1.0, 0.0, 0.0, 1.0)
}

pub struct Context {
    gl: GlContext,
    driver: Driver,
    color_batcher: shaders::ColorBatcher,
    is_drawing: bool,
    render_target: Option<RenderTarget>,
    data: DrawData,
}

impl Context {
    pub fn new(win: &web_sys::HtmlCanvasElement) -> Result<Context, String> {
        let width = win.width() as i32;
        let height = win.height() as i32;
        let (gl, driver) = create_gl_context(win)?;

        let data = DrawData::new(width, height);
        let color_batcher = ColorBatcher::new(&gl, &data)?;

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

    //    pub fn set_width(&mut self, width: i32) {
    //        self.data.set_width(width);
    //    }
    //
    //    pub fn set_height(&mut self, height: i32) {
    //        self.data.set_height(height);
    //    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.data.set_size(width, height);
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

    pub fn transform(&mut self) -> &mut Transform {
        &mut self.data.transform
    }

    pub fn begin(&mut self) {
        if self.is_drawing {
            return;
        }
        self.is_drawing = true;

        let (fbo, ww, hh) = if let Some(rt) = &self.render_target {
            (Some(rt.fbo), rt.width, rt.height)
        } else {
            (None, self.width(), self.height())
        };

        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, fbo);
            self.gl.viewport(0, 0, ww, hh);
        }

        self.color_batcher.begin();
    }

    pub fn clear(&mut self, color: Color) {
        let (r, g, b, a) = color.to_rgba();
        unsafe {
            self.gl.clear_color(r, g, b, a);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
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
        self.color_batcher.flush(&self.gl, &self.data);
    }

    fn draw_color(&mut self, vertex: &[f32], color: Option<&[Color]>) {
        let color_vertex= match color {
            Some(c) => {
                c.iter().map(|c| c.to_rgba())
                    .fold(vec![], |mut acc, v| {
                        acc.append(&mut vec![v.0, v.1, v.2, v.3]);
                        acc
                    })
            }
            _ => vec![]
        };

        self.color_batcher.draw(&self.gl, &self.data, vertex, if color.is_some() {
            Some(&color_vertex)
        } else {
            None
        });
    }

    pub fn draw_triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.draw_color(&[x1, y1, x2, y2, x3, y3], None);
    }

    pub fn draw_triangle_with_color_vertices(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3:f32, colors: &[Color; 3]) {
        self.draw_color(&[x1, y1, x2, y2, x3, y3], Some(colors));
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let x2 = x + width;
        let y2 = y + height;
        let vertices = [x, y, x2, y, x, y2, x, y2, x2, y, x2, y2];

        self.draw_color(&vertices, None);
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, strength: f32) {
        let (mut xx, mut yy) = if y1 == y2 {
            (0.0, -1.0)
        } else {
            (1.0, -(x2 - x1) / (y2 - y1))
        };

        let len = (xx * xx + yy * yy).sqrt();
        if len != 0.0 {
            //TODO use epsilon to check this floats?
            let mul = strength / len;
            xx *= mul;
            yy *= mul;
        }

        let px1 = x1 + 0.5 * xx;
        let py1 = y1 + 0.5 * yy;
        let px2 = x2 + 0.5 * xx;
        let py2 = y2 + 0.5 * yy;
        let px3 = px1 - xx;
        let py3 = py1 - yy;
        let px4 = px2 - xx;
        let py4 = py2 - yy;

        self.draw_color(&[px1, py1, px2, py2, px3, py3, px3, py3, px2, py2, px4, py4], None);
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32) {
        self.draw_color(&get_circle_vertices(x, y, radius, None), None);
        //https://docs.rs/lyon_tessellation/0.14.1/lyon_tessellation/geometry_builder/index.html
        //https://docs.rs/lyon_tessellation/0.14.1/lyon_tessellation/struct.FillTessellator.html#examples
    }

    pub fn draw_rounded_rect(&mut self, x: f32, y:f32, width: f32, height: f32, radius: f32) {

    }

    pub fn stroke_triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, line_width: f32) {

    }

    pub fn stroke_rect(&mut self, x: f32, y:f32, width: f32, height: f32, line_width: f32) {

    }

    pub fn stroke_rounded_rect(&mut self, x: f32, y:f32, width: f32, height: f32, radius: f32) {

    }

    pub fn stroke_circle(&mut self, x: f32, y:f32, radius: f32, line_width: f32) {
        let mut output:VertexBuffers<(f32, f32), usize> = VertexBuffers::new();
        let mut opts = tess::StrokeOptions::tolerance(0.01);
        opts = opts.with_line_width(line_width);
        log(&format!("line options: {:?}", opts));
        stroke_circle(
            tess::math::point(x, y),
            radius,
            &mut opts,
            &mut BuffersBuilder::new(
                &mut output,
                WithColor,
            )
        );
//        fill_circle(
//            tess::math::point(x, y),
//            radius,
//            &tess::FillOptions::tolerance(0.05),
//            &mut BuffersBuilder::new(
//                &mut output,
//                WithColor([1.0, 1.0, 0.0, 1.0])
//            )
//        );

        let mut vertices = vec![];
//        output.vertices.iter().for_each(|v| {
//            vertices.push(v.position[0]);
//            vertices.push(v.position[1]);
//        });
        // https://stackoverflow.com/questions/28075739/drawelements-vs-drawarrays-in-webgl
        output.indices.iter().for_each(|mut i| {
            vertices.push(output.vertices[*i].0);
            vertices.push(output.vertices[*i].1);
        });

        self.draw_color(&vertices, None);

        log(&format!("output: {:?} {} {}", output, output.vertices.len(), output.indices.len()));
    }

    pub fn draw_geometry(&mut self, geometry: &mut Geometry) {

    }

    pub fn draw_svg(&mut self, svg: &mut Svg) {

    }

    pub fn draw_vertex(&mut self, vertices: &[Vertex]) {
        let (vert, color_vert) = vertices.iter()
            .fold((vec![], vec![]), |(mut v_acc, mut vc_acc), v| {
                v_acc.push(v.pos.0);
                v_acc.push(v.pos.1);
                vc_acc.push(v.color);
                (v_acc, vc_acc)
            });

        log(&format!("vert: {:?} color:{:?}", vert, color_vert));
        self.draw_color(&vert, Some(&color_vert));
    }
}

pub struct Vertex {
    pos: (f32, f32),
    color: Color
}

impl Vertex {
    pub fn new(x:f32, y:f32, color: Color) -> Self {
        Self {
            pos: (x, y),
            color: color
        }
    }
}

pub struct Svg {}

pub struct Geometry {
    //https://github.com/nical/lyon/issues/462
    vertices: Option<Vec<f32>>,
}

// Our custom vertex.
//#[derive(Copy, Clone, Debug)]
//pub struct MyVertex {
//    position: [f32; 2],
////    color: [f32; 4],
//}

// The vertex constructor. This is the object that will be used to create the custom
// verticex from the information provided by the tessellators.
struct WithColor;

impl tess::VertexConstructor<tess::StrokeVertex, (f32, f32)> for WithColor {
    fn new_vertex(&mut self, vertex: tess::StrokeVertex) -> (f32, f32) {
        // FillVertex also provides normals but we don't need it here.
        (vertex.position.x, vertex.position.y)
//        MyVertex {
//            position: [
//                vertex.position.x,
//                vertex.position.y,
//            ],
////            color: self.0,
//        }
    }
}

fn get_circle_vertices(x: f32, y: f32, radius: f32, segments: Option<i32>) -> Vec<f32> {
    let segments = if let Some(s) = segments {
        s
    } else {
        (10.0 * radius.sqrt()).floor() as i32
    };
    let theta = 2.0 * std::f32::consts::PI / segments as f32;
    let cos = theta.cos();
    let sin = theta.sin();
    let mut xx = radius;
    let mut yy = 0.0;

    let mut vertices = vec![];
    for i in (0..segments) {
        let x1 = xx + x;
        let y1 = yy + y;
        let last_x = xx;
        xx = cos * xx - sin * yy;
        yy = cos * yy + sin * last_x;
        vertices.push(x1);
        vertices.push(y1);
        vertices.push(xx + x);
        vertices.push(yy + y);
        vertices.push(x);
        vertices.push(y);
        //        vertices.append(&mut vec![x1, y1, xx+x, yy+y, x, y]);
    }

    vertices
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
