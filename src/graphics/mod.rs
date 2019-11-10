use self::shaders::ColorBatcher;
use crate::graphics::shaders::{PatternBatcher, SpriteBatcher};
use crate::math::*;
use crate::res::*;
use color::Color;
use glow::*;
use lyon::lyon_tessellation as tess;
use rayon::prelude::*;
use std::rc::Rc;
use tess::basic_shapes::stroke_circle;
use wasm_bindgen::JsCast;
use web_sys;

pub mod color;
pub mod shader;
pub mod shaders;
pub mod transform;

use transform::Transform2d;

#[derive(Debug, Eq, PartialEq)]
enum PaintMode {
    Color,
    Image,
    Pattern,
    Text,
    Empty,
}

//TODO draw_image with crop, scale, etc... draw_image_ext

/*
TODO API:
    let draw = app.draw();
    draw.transform()
        .translate(100.0, 100.0)
        .scale(2.0, 2.0)
        .rotate_deg(45.0);
    draw.circle(0.0, 0.0, 50.0);
    draw.transform()
        .pop()
        .pop()
        .pop();
    - - - - - - - - - - Same As: - - - - - - - - - - - - -
    let draw = app.draw();
    draw.obj()
        .circle(100.0, 100.0, 50.0)
        .scale(2.0, 2.0)
        .rotate_dev(45.0);
        //.matrix(push your own matrix)L
*/

//TODO glsl to spv https://crates.io/crates/shaderc -> https://crates.io/crates/spirv_cross spv->glsl->etc...

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

use crate::math;
use lyon::lyon_tessellation::basic_shapes::{
    fill_rounded_rectangle, stroke_rectangle, stroke_rounded_rectangle, stroke_triangle,
    BorderRadii,
};
use lyon::lyon_tessellation::{BuffersBuilder, VertexBuffers};

pub struct DrawData {
    alpha: f32,
    color: Color,
    shader: Option<shader::Shader>,
    transform: Transform2d,
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
            transform: Transform2d::new(),
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

    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
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

pub struct Context2d {
    gl: GlContext,
    driver: Driver,
    color_batcher: shaders::ColorBatcher,
    sprite_batcher: shaders::SpriteBatcher,
    pattern_batcher: shaders::PatternBatcher,
    is_drawing: bool,
    render_target: Option<RenderTarget>,
    data: DrawData,
    paint_mode: PaintMode,
}

impl Context2d {
    pub fn new(win: &web_sys::HtmlCanvasElement) -> Result<Context2d, String> {
        let width = win.width() as i32;
        let height = win.height() as i32;
        let (gl, driver) = create_gl_context(win)?;

        let data = DrawData::new(width, height);
        let color_batcher = ColorBatcher::new(&gl, &data)?;
        let sprite_batcher = SpriteBatcher::new(&gl, &data)?;
        let pattern_batcher = PatternBatcher::new(&gl, &data)?;

        //2d
        unsafe {
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        }

        Ok(Context2d {
            data,
            gl,
            driver,
            color_batcher,
            sprite_batcher,
            pattern_batcher,
            is_drawing: false,
            render_target: None,
            paint_mode: PaintMode::Empty,
        })
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.data.set_alpha(alpha);
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

    pub fn transform(&mut self) -> &mut Transform2d {
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
        self.flush_sprite();
        self.flush_pattern();
    }

    fn flush_color(&mut self) {
        self.color_batcher.flush(&self.gl, &self.data);
    }

    fn flush_sprite(&mut self) {
        self.sprite_batcher.flush(&self.gl, &self.data);
    }

    fn flush_pattern(&mut self) {
        self.pattern_batcher.flush(&self.gl, &self.data);
    }

    fn draw_color(&mut self, vertex: &[f32], color: Option<&[Color]>) {
        self.set_paint_mode(PaintMode::Color);
        let color_vertex = match color {
            Some(c) => c.iter().map(|c| c.to_rgba()).fold(vec![], |mut acc, v| {
                acc.append(&mut vec![v.0, v.1, v.2, v.3]);
                acc
            }),
            _ => vec![],
        };

        let color = if color.is_some() {
            Some(color_vertex.as_slice())
        } else {
            None
        };

        self.color_batcher.draw(&self.gl, &self.data, vertex, color);
    }

    pub fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.draw_color(&[x1, y1, x2, y2, x3, y3], None);
    }

    pub fn stroke_triangle(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        line_width: f32,
    ) {
        let mut output: VertexBuffers<(f32, f32), u16> = VertexBuffers::new();
        let mut opts = tess::StrokeOptions::tolerance(0.01);
        opts = opts.with_line_width(line_width);
        stroke_triangle(
            tess::math::point(x1, y1),
            tess::math::point(x2, y2),
            tess::math::point(x3, y3),
            &mut opts,
            &mut BuffersBuilder::new(&mut output, LyonVertex),
        )
        .unwrap();

        self.draw_color(&lyon_vbuff_to_vertex(output), None);
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let x2 = x + width;
        let y2 = y + height;
        let vertices = [x, y, x2, y, x, y2, x, y2, x2, y, x2, y2];

        self.draw_color(&vertices, None);
    }

    pub fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32, line_width: f32) {
        let mut output: VertexBuffers<(f32, f32), u16> = VertexBuffers::new();
        let mut opts = tess::StrokeOptions::tolerance(0.01);
        opts = opts.with_line_width(line_width);
        stroke_rectangle(
            &tess::math::rect(x, y, width, height),
            &mut opts,
            &mut BuffersBuilder::new(&mut output, LyonVertex),
        )
        .unwrap();

        self.draw_color(&lyon_vbuff_to_vertex(output), None);
    }

    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, strength: f32) {
        let (mut xx, mut yy) = if math::eq_float(y1, y2) {
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

        self.draw_color(
            &[px1, py1, px2, py2, px3, py3, px3, py3, px2, py2, px4, py4],
            None,
        );
    }

    pub fn circle(&mut self, x: f32, y: f32, radius: f32) {
        self.draw_color(&get_circle_vertices(x, y, radius, None), None);
    }

    pub fn rounded_rect(&mut self, x: f32, y: f32, width: f32, height: f32, radius: f32) {
        let mut output: VertexBuffers<(f32, f32), u16> = VertexBuffers::new();
        let opts = tess::FillOptions::tolerance(0.01);
        fill_rounded_rectangle(
            &tess::math::rect(x, y, width, height),
            &BorderRadii::new(radius, radius, radius, radius),
            &opts,
            &mut BuffersBuilder::new(&mut output, LyonVertex),
        )
        .unwrap();

        self.draw_color(&lyon_vbuff_to_vertex(output), None);
    }

    pub fn stroke_rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        radius: f32,
        line_width: f32,
    ) {
        let mut output: VertexBuffers<(f32, f32), u16> = VertexBuffers::new();
        let mut opts = tess::StrokeOptions::tolerance(0.01);
        opts = opts.with_line_width(line_width);
        stroke_rounded_rectangle(
            &tess::math::rect(x, y, width, height),
            &BorderRadii::new(radius, radius, radius, radius),
            &opts,
            &mut BuffersBuilder::new(&mut output, LyonVertex),
        )
        .unwrap();

        self.draw_color(&lyon_vbuff_to_vertex(output), None);
    }

    pub fn stroke_circle(&mut self, x: f32, y: f32, radius: f32, line_width: f32) {
        let mut output: VertexBuffers<(f32, f32), u16> = VertexBuffers::new();
        let mut opts = tess::StrokeOptions::tolerance(0.01);
        opts = opts.with_line_width(line_width);
        stroke_circle(
            tess::math::point(x, y),
            radius,
            &mut opts,
            &mut BuffersBuilder::new(&mut output, LyonVertex),
        )
        .unwrap();
        self.draw_color(&lyon_vbuff_to_vertex(output), None);
    }

    pub fn geometry(&mut self, geometry: &mut Geometry) {
        geometry.build();
        if let Some((v, vc)) = &geometry.vertices {
            self.draw_color(v, Some(vc));
        }
    }

    //    pub fn draw_svg(&mut self, svg: &mut Svg) {}

    pub fn image(&mut self, img: &mut Texture, x: f32, y: f32) {
        self.image_ext(img, x, y, 0.0, 0.0, 0.0, 0.0);
    }

    pub fn image_ext(
        &mut self,
        img: &mut Texture,
        x: f32,
        y: f32,
        sx: f32,
        sy: f32,
        sw: f32,
        sh: f32,
    ) {
        self.set_paint_mode(PaintMode::Image);
        self.sprite_batcher
            .draw(&self.gl, &self.data, x, y, img, sx, sy, sw, sh, None);
    }

    //TODO add a method to draw the image scaled without using the matrix?

    pub fn pattern(
        &mut self,
        img: &mut Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        self.pattern_ext(img, x, y, width, height, offset_x, offset_y, 1.0, 1.0);
    }

    fn set_paint_mode(&mut self, mode: PaintMode) {
        if mode == self.paint_mode {
            return;
        }
        self.paint_mode = mode;
        match &self.paint_mode {
            PaintMode::Color => {
                self.flush_sprite();
                self.flush_pattern();
            }
            PaintMode::Image => {
                self.flush_color();
                self.flush_pattern();
            }
            PaintMode::Pattern => {
                self.flush_color();
                self.flush_sprite();
            }
            _ => {}
        }
    }

    pub fn pattern_ext(
        &mut self,
        img: &mut Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        offset_x: f32,
        offset_y: f32,
        scale_x: f32,
        scale_y: f32,
    ) {
        self.set_paint_mode(PaintMode::Pattern);
        self.pattern_batcher.draw(
            &self.gl, &self.data, x, y, img, width, height, offset_x, offset_y, scale_x, scale_y,
            None,
        );
    }

    pub fn nine_slice(&mut self, _x: f32, _y: f32, _opts: String) {}

    pub fn vertex(&mut self, vertices: &[Vertex]) {
        let (vert, color_vert) =
            vertices
                .iter()
                .fold((vec![], vec![]), |(mut v_acc, mut vc_acc), v| {
                    v_acc.push(v.pos.0);
                    v_acc.push(v.pos.1);
                    vc_acc.push(v.color);
                    (v_acc, vc_acc)
                });

        self.draw_color(&vert, Some(&color_vert));
    }
}

pub struct Vertex {
    pos: (f32, f32),
    color: Color,
}

impl Vertex {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self { pos: (x, y), color }
    }
}

pub struct Svg {}

fn get_circle_vertices(x: f32, y: f32, radius: f32, segments: Option<i32>) -> Vec<f32> {
    let segments = if let Some(s) = segments {
        s
    } else {
        (10.0 * radius.sqrt()).floor() as i32
    };
    let theta = 2.0 * PI / segments as f32;
    let cos = theta.cos();
    let sin = theta.sin();
    let mut xx = radius;
    let mut yy = 0.0;

    let mut vertices = vec![];
    for _i in 0..segments {
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
