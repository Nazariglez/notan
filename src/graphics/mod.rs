use std::rc::Rc;

use glow::*;
use lyon::lyon_tessellation as tess;
use lyon::lyon_tessellation::basic_shapes::{
    fill_rounded_rectangle, stroke_rectangle, stroke_rounded_rectangle, stroke_triangle,
    BorderRadii,
};
use lyon::lyon_tessellation::{BuffersBuilder, VertexBuffers};
use rayon::prelude::*;
use tess::basic_shapes::stroke_circle;
use wasm_bindgen::JsCast;
use web_sys;

use batchers::{ColorBatcher, SpriteBatcher};
use color::Color;
use shader::Shader;
use transform::Transform2d;

use crate::graphics::batchers::TextBatcher;
use crate::math::*;
use crate::res::*;
use crate::{log, math};
use lyon::lyon_algorithms::fit::FitStyle::Horizontal;

pub mod batchers;
mod blend;
pub mod color;
pub mod shader;
pub mod surface;
pub mod transform;

use crate::graphics::surface::Surface;
pub use blend::*;
use nalgebra_glm::proj;

/*TODO FILTERS: (or post processing effects...)
    draw.filter(filters: &[Filter], cb: |ctx|{
        ctx.drawImage(&img, 0.0, 0.0);
    });
    This is going to render:
        - Callback to a render_target
        - Render this render_target with each filter in order
        - Render to screen once all the filters are done

   Filter {
    blend: BlendMode,
    shader: Shader,
    draw: FnMut(shader) {}
   }

   PostProcess::new(app, 300.0, 300.0, &[Filters])
   PostProcess.draw(app, state);
   draw.image(postprocess.image(), 100.0, 100.0);

   PostProcess.add(blendMode, shader, |app, state| {
    apply uniforms, etc...
   });
*/

#[derive(Debug, Eq, PartialEq)]
enum PaintMode {
    Color,
    Image,
    Text,
    Empty,
}

//TODO draw_image with crop, scale, etc... draw_image_ext

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

pub struct RenderTarget {
    fbo: glow::WebFramebufferKey,
    width: i32,
    height: i32,
}

pub struct DrawData {
    alpha: f32,
    color: Color,
    shader: Option<shader::Shader>,
    transform: Transform2d,
    width: i32,
    height: i32,
    flipped: bool,
    projection: glm::Mat3,
}

impl DrawData {
    pub fn new(width: i32, height: i32) -> Self {
        let projection = get_projection(width, height, false);
        Self {
            width,
            height,
            alpha: 1.0,
            shader: None,
            transform: Transform2d::new(),
            color: Color::WHITE,
            projection,
            flipped: false,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_size(&mut self, width: i32, height: i32, flipped: bool) {
        if width != self.width || height != self.height || flipped != self.flipped {
            self.upadte_projection(width, height, flipped);
        }
    }

    pub fn set_shader(&mut self, shader: Option<&Shader>) {
        self.shader = shader.map(|v| v.clone());
    }

    fn upadte_projection(&mut self, width: i32, height: i32, flipped: bool) {
        self.width = width;
        self.height = height;
        self.flipped = flipped;
        self.projection = get_projection(self.width, self.height, flipped);
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
    }
}

fn get_projection(width: i32, height: i32, flipped: bool) -> glm::Mat3 {
    let ww = width as f32;
    let hh = height as f32;
    let bottom = if flipped { 0.0 } else { hh };
    let top = if flipped { hh } else { 0.0 };
    glm::translate2d(
        &glm::mat4_to_mat3(&glm::ortho(0.0, ww, bottom, top, -1.0, 1.0)),
        &glm::vec2(-ww * 0.5, -hh * 0.5),
    )
}

pub struct Context2d {
    pub(crate) gl: GlContext,
    driver: Driver,
    color_batcher: batchers::ColorBatcher,
    sprite_batcher: batchers::SpriteBatcher,
    text_batcher: batchers::TextBatcher,
    is_drawing: bool,
    render_target: Option<RenderTarget>,
    data: DrawData,
    paint_mode: PaintMode,
    stencil: bool,
    blend_mode: BlendMode,
    is_drawing_surface: bool,
    width: i32,
    height: i32,
}

impl Context2d {
    pub fn new(win: &web_sys::HtmlCanvasElement) -> Result<Context2d, String> {
        let width = win.width() as i32;
        let height = win.height() as i32;
        let (gl, driver) = create_gl_context(win)?;

        let data = DrawData::new(width, height);
        let color_batcher = ColorBatcher::new(&gl, &data)?;
        let sprite_batcher = SpriteBatcher::new(&gl, &data)?;
        let text_batcher = TextBatcher::new(&gl, &data)?;
        let blend_mode = BlendMode::NORMAL;

        //2d
        unsafe {
            gl.disable(glow::DEPTH_TEST);
            gl.enable(glow::BLEND);
            gl.blend_func(blend_mode.source().into(), blend_mode.destination().into());
        }

        Ok(Context2d {
            data,
            gl,
            driver,
            color_batcher,
            sprite_batcher,
            text_batcher,
            blend_mode,
            is_drawing: false,
            is_drawing_surface: false,
            render_target: None,
            paint_mode: PaintMode::Empty,
            stencil: false,
            width,
            height,
        })
    }

    pub(crate) fn add_font(&mut self, data: Vec<u8>) -> usize {
        self.text_batcher.manager.add(data)
    }

    pub(crate) fn text_size(
        &mut self,
        font: &Font,
        text: &str,
        size: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        max_width: Option<f32>,
    ) -> (f32, f32) {
        let size = self
            .text_batcher
            .manager
            .text_size(font, text, size, h_align, v_align, max_width);
        (size.0 as _, size.1 as _)
    }

    pub fn set_shader(&mut self, shader: Option<&Shader>) {
        if shader.is_none() && self.data.shader.is_none() {
            return;
        }

        if shader.is_some() && self.data.shader.is_none()
            || shader.is_none() && self.data.shader.is_some()
        {
            self.update_custom_shader(shader);
            return;
        }

        if let (Some(s1), Some(s2)) = (shader, &self.data.shader) {
            if s1.is_equal(s2) {
                self.update_custom_shader(shader);
            }
        }
    }

    fn update_custom_shader(&mut self, shader: Option<&Shader>) {
        self.flush();
        self.data.set_shader(shader);
        if let Some(s) = shader {
            s.use_me();
        }
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.data.set_alpha(alpha);
    }

    pub fn set_blend(&mut self, mode: BlendMode) {
        if mode != self.blend_mode {
            self.flush();
            self.blend_mode = mode;

            unsafe {
                self.gl.blend_func(
                    self.blend_mode.source().into(),
                    self.blend_mode.destination().into(),
                );
            }
        }
    }

    //    pub fn set_width(&mut self, width: i32) {
    //        self.data.set_width(width);
    //    }
    //
    //    pub fn set_height(&mut self, height: i32) {
    //        self.data.set_height(height);
    //    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;

        if !self.is_drawing {
            self.data.set_size(width, height, false);
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn set_color(&mut self, color: Color) {
        self.data.set_color(color);
    }

    pub fn transform(&mut self) -> &mut Transform2d {
        &mut self.data.transform
    }

    pub fn begin_to_surface(&mut self, surface: Option<&Surface>) {
        if self.is_drawing {
            return;
        }
        self.is_drawing = true;

        let (fbo, ww, hh) = if let Some(rt) = surface {
            self.is_drawing_surface = true;
            (Some(rt.fbo), rt.width() as _, rt.height() as _)
        } else {
            (None, self.width(), self.height())
        };

        self.data.set_size(ww, hh, self.is_drawing_surface);
        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, fbo);

            if fbo.is_some() {
                self.gl.draw_buffer(glow::COLOR_ATTACHMENT0);
            }

            self.gl.viewport(0, 0, ww, hh);
        }

        self.color_batcher.reset();
        self.text_batcher.reset();
        self.sprite_batcher.reset();
    }

    pub fn end(&mut self) {
        if !self.is_drawing {
            return;
        }
        self.is_drawing = false;
        self.clear_mask(); //this is already doing flush

        self.data
            .set_size(self.width, self.height, self.is_drawing_surface);
        self.is_drawing_surface = false;

        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            self.gl.bind_texture(glow::TEXTURE_2D, None);
            self.gl.viewport(0, 0, self.width(), self.height());
        }
    }

    pub fn begin(&mut self) {
        self.begin_to_surface(None);
    }

    pub fn clear(&mut self, color: Color) {
        let [r, g, b, a] = color.to_rgba();
        unsafe {
            self.gl.clear_color(r, g, b, a);
            let mut flags = glow::COLOR_BUFFER_BIT;
            if self.stencil {
                flags |= glow::STENCIL_BUFFER_BIT;
            }

            self.gl.clear(flags);
        }
    }

    pub fn begin_mask(&mut self) {
        self.flush();
        unsafe {
            self.stencil = true;
            self.gl.enable(glow::STENCIL_TEST);
            self.gl.stencil_op(glow::KEEP, glow::KEEP, glow::REPLACE);
            self.gl.stencil_func(glow::ALWAYS, 1, 0xff);
            self.gl.stencil_mask(0xff);
            self.gl.depth_mask(false);
            self.gl.color_mask(false, false, false, false);
        }
    }

    pub fn end_mask(&mut self) {
        self.flush();
        unsafe {
            self.gl.stencil_func(glow::EQUAL, 1, 0xff);
            self.gl.stencil_mask(0x00);
            self.gl.depth_mask(true);
            self.gl.color_mask(true, true, true, true);
        }
    }

    pub fn clear_mask(&mut self) {
        self.flush();
        unsafe {
            self.gl.flush();
            self.stencil = false;
            self.gl.disable(glow::STENCIL_TEST);
        }
    }

    pub fn flush(&mut self) {
        self.flush_color();
        self.flush_sprite();
        self.flush_text();
    }

    fn flush_text(&mut self) {
        self.text_batcher.flush(&self.gl, &self.data);
    }

    fn flush_color(&mut self) {
        self.color_batcher.flush(&self.gl, &self.data);
    }

    fn flush_sprite(&mut self) {
        self.sprite_batcher.flush(&self.gl, &self.data);
    }

    pub fn set_font(&mut self, font: &Font) {
        self.text_batcher.set_font(font);
    }

    pub fn font(&self) -> &Font {
        &self.text_batcher.font
    }

    pub fn text(&mut self, text: &str, x: f32, y: f32, size: f32) {
        self.text_ext(
            text,
            x,
            y,
            size,
            HorizontalAlign::Left,
            VerticalAlign::Top,
            None,
        );
    }

    pub fn text_ext(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        max_width: Option<f32>,
    ) {
        self.set_paint_mode(PaintMode::Text);
        self.text_batcher.draw_text(
            &self.gl, &self.data, text, x, y, size, h_align, v_align, max_width,
        );
    }

    fn draw_color(&mut self, vertex: &[f32], color: Option<&[Color]>) {
        self.set_paint_mode(PaintMode::Color);
        let color_vertex = match color {
            Some(c) => c.iter().fold(vec![], |mut acc, v| {
                acc.extend_from_slice(&v.to_rgba());
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

    pub fn image(&mut self, img: &Texture, x: f32, y: f32) {
        self.image_ext(img, x, y, img.width(), img.height(), 0.0, 0.0, 0.0, 0.0);
    }

    pub fn image_crop(
        &mut self,
        img: &Texture,
        x: f32,
        y: f32,
        source_x: f32,
        source_y: f32,
        source_width: f32,
        source_height: f32,
    ) {
        self.image_ext(
            img,
            x,
            y,
            source_width,
            source_height,
            source_x,
            source_y,
            source_width,
            source_height,
        );
    }

    pub fn image_ext(
        &mut self,
        img: &Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        source_x: f32,
        source_y: f32,
        source_width: f32,
        source_height: f32,
    ) {
        self.set_paint_mode(PaintMode::Image);
        self.sprite_batcher.draw_image(
            &self.gl,
            &self.data,
            x,
            y,
            width,
            height,
            img,
            source_x,
            source_y,
            source_width,
            source_height,
            None,
        );
    }

    //TODO allow to change the tex_matrix for images and patterns?

    pub fn pattern(
        &mut self,
        img: &Texture,
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
                self.flush_text();
            }
            PaintMode::Image => {
                self.flush_color();
                self.flush_text();
            }
            PaintMode::Text => {
                self.flush_color();
                self.flush_sprite();
            }
            _ => {}
        }
    }

    pub fn pattern_ext(
        &mut self,
        img: &Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        offset_x: f32,
        offset_y: f32,
        scale_x: f32,
        scale_y: f32,
    ) {
        self.set_paint_mode(PaintMode::Image);
        self.sprite_batcher.draw_pattern(
            &self.gl, &self.data, x, y, img, width, height, offset_x, offset_y, scale_x, scale_y,
            None,
        );
    }

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

    pub fn image_9slice(&mut self, img: &Texture, x: f32, y: f32, width: f32, height: f32) {
        let ww = img.width() / 3.0;
        let hh = img.height() / 3.0;
        self.image_9slice_ext(img, x, y, width, height, ww, ww, hh, hh);
    }

    pub fn image_9slice_ext(
        &mut self,
        img: &Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
    ) {
        let center_sw = img.width() - (left + right);
        let center_sh = img.height() - (top + bottom);
        let center_w = width - (left + right);
        let center_h = height - (top + bottom);

        self.image_crop(img, x, y, 0.0, 0.0, left, top);
        self.image_ext(img, x + left, y, center_w, top, left, 0.0, center_sw, top);
        self.image_crop(
            img,
            x + left + center_w,
            y,
            left + center_sw,
            0.0,
            right,
            top,
        );

        self.image_ext(img, x, y + top, left, center_h, 0.0, top, left, center_sh);
        self.image_ext(
            img,
            x + left,
            y + top,
            center_w,
            center_h,
            left,
            top,
            center_sw,
            center_sh,
        );
        self.image_ext(
            img,
            x + left + center_w,
            y + top,
            right,
            center_h,
            left + center_sw,
            top,
            right,
            center_sh,
        );

        self.image_crop(
            img,
            x,
            y + top + center_h,
            0.0,
            top + center_sh,
            left,
            bottom,
        );
        self.image_ext(
            img,
            x + left,
            y + top + center_h,
            center_w,
            bottom,
            left,
            top + center_sh,
            center_sw,
            bottom,
        );
        self.image_crop(
            img,
            x + left + center_w,
            y + top + center_h,
            left + center_sw,
            top + center_sh,
            right,
            bottom,
        );
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

fn webgl_options() -> web_sys::WebGlContextAttributes {
    let mut opts = web_sys::WebGlContextAttributes::new();
    opts.stencil(true);
    opts.premultiplied_alpha(false);
    opts.alpha(false);
    opts.antialias(true);
    opts
}

fn create_webgl_context(win: &web_sys::HtmlCanvasElement) -> Result<GlContext, String> {
    //TODO manage errors
    let gl = win
        .get_context_with_context_options("webgl2", webgl_options().as_ref())
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
        .get_context_with_context_options("webgl2", webgl_options().as_ref())
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();

    let ctx = Rc::new(glow::Context::from_webgl2_context(gl));
    Ok(ctx)
}

pub(crate) fn create_gl_tex(
    gl: &GlContext,
    width: i32,
    height: i32,
    data: &[u8],
) -> Result<glow::WebTextureKey, String> {
    unsafe {
        let tex = gl.create_texture()?;
        gl.bind_texture(glow::TEXTURE_2D, Some(tex));

        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::NEAREST as i32,
        );

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width,
            height,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(data),
        );

        //TODO mipmaps? gl.generate_mipmap(glow::TEXTURE_2D);
        gl.bind_texture(glow::TEXTURE_2D, None);
        Ok(tex)
    }
}

pub(crate) fn create_gl_tex_ext(
    gl: &GlContext,
    width: i32,
    height: i32,
    data: &[u8],
    internal: i32,
    format: i32,
    min_filter: i32,
    mag_filter: i32,
    bytes_per_pixel: usize,
) -> Result<glow::WebTextureKey, String> {
    unsafe {
        let tex = gl.create_texture()?;
        if bytes_per_pixel == 1 {
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
        }

        gl.bind_texture(glow::TEXTURE_2D, Some(tex));

        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, mag_filter);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, min_filter);

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            internal,
            width,
            height,
            0,
            format as _,
            glow::UNSIGNED_BYTE,
            Some(data),
        );

        //TODO mipmaps? gl.generate_mipmap(glow::TEXTURE_2D);
        gl.bind_texture(glow::TEXTURE_2D, None);
        Ok(tex)
    }
}
