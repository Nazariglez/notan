use crate::batchers::{ColorBatcher, SpriteBatcher, TextBatcher};
use crate::font::Font;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::{GlContext, GlowValue, Surface};
use glow::HasContext;

use glyph_brush::BrushAction::Draw;
use lyon::lyon_tessellation as tess;
use nae_core::graphics::{
    lyon_vbuff_to_vertex, BaseContext2d, BaseShader, BaseSurface, BlendFactor, BlendMode, Color,
    Geometry, LyonVertex, Transform2d, Vertex,
};
use nae_core::math::*;
use nae_core::resources::{BaseFont, BaseTexture, HorizontalAlign, VerticalAlign};
use std::rc::Rc;
use tess::basic_shapes::stroke_triangle;
use tess::basic_shapes::{
    fill_rounded_rectangle, stroke_circle, stroke_rectangle, stroke_rounded_rectangle, BorderRadii,
};
use tess::{BuffersBuilder, VertexBuffers};

#[cfg(not(target_arch = "wasm32"))]
use glutin::{PossiblyCurrent, WindowedContext};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
type Device = web_sys::HtmlCanvasElement;

#[cfg(not(target_arch = "wasm32"))]
type Device = WindowedContext<PossiblyCurrent>;

pub struct Context2d {
    pub(crate) gl: GlContext,
    pub(crate) text_batcher: TextBatcher,
    pub(crate) sprite_batcher: SpriteBatcher,
    pub(crate) color_batcher: ColorBatcher,
    is_drawing: bool,
    data: DrawData,
    paint_mode: PaintMode,
    stencil: bool,
    blend_mode: BlendMode,
    is_drawing_surface: bool,
    width: i32,
    height: i32,
}

impl Context2d {
    fn flush_text(&mut self) {
        self.text_batcher.flush(&self.gl, &self.data);
    }

    fn flush_color(&mut self) {
        self.color_batcher.flush(&self.gl, &self.data);
    }

    fn flush_sprite(&mut self) {
        self.sprite_batcher.flush(&self.gl, &self.data);
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

        self.color_batcher.draw(&self.gl, &self.data, vertex, color)
    }
}

impl BaseContext2d for Context2d {
    type Device = Device;
    type Shader = Shader;
    type Surface = Surface;
    type Texture = Texture;
    type Font = Font;

    fn new(device: &Self::Device) -> Result<Self, String> {
        create_context_2d(device)
    }

    fn set_shader(&mut self, shader: Option<&Shader>) {
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

    fn set_alpha(&mut self, alpha: f32) {
        self.data.alpha = alpha;
    }

    fn set_blend(&mut self, mode: BlendMode) {
        if mode != self.blend_mode {
            self.flush();
            self.blend_mode = mode;

            unsafe {
                self.gl.blend_func(
                    self.blend_mode.source().glow_value(),
                    self.blend_mode.destination().glow_value(),
                );
            }
        }
    }

    fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;

        if !self.is_drawing {
            self.data.set_size(width, height, false);
        }
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn set_color(&mut self, color: Color) {
        self.data.color = color;
    }

    fn transform(&mut self) -> &mut Transform2d {
        &mut self.data.transform
    }

    fn begin_to_surface(&mut self, surface: Option<&Surface>) {
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

            let v_width = (ww as f32 * self.data.dpi) as _;
            let v_height = (hh as f32 * self.data.dpi) as _;
            self.gl.viewport(0, 0, v_width, v_height);
        }

        self.color_batcher.reset();
        self.text_batcher.reset();
        self.sprite_batcher.reset();
    }

    fn begin(&mut self) {
        self.begin_to_surface(None);
    }

    fn end(&mut self) {
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

    fn clear(&mut self, color: Color) {
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

    fn begin_mask(&mut self) {
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

    fn end_mask(&mut self) {
        self.flush();
        unsafe {
            self.gl.stencil_func(glow::EQUAL, 1, 0xff);
            self.gl.stencil_mask(0x00);
            self.gl.depth_mask(true);
            self.gl.color_mask(true, true, true, true);
        }
    }

    fn clear_mask(&mut self) {
        self.flush();
        unsafe {
            self.gl.flush();
            self.stencil = false;
            self.gl.disable(glow::STENCIL_TEST);
        }
    }

    fn flush(&mut self) {
        self.flush_color();
        self.flush_sprite();
        self.flush_text();
    }

    fn set_font(&mut self, font: &Self::Font) {
        self.text_batcher.set_font(font);
    }

    fn font(&self) -> &Self::Font {
        &self.text_batcher.font
    }

    fn text(&mut self, text: &str, x: f32, y: f32, size: f32) {
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

    fn text_ext(
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

    fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.draw_color(&[x1, y1, x2, y2, x3, y3], None);
    }

    fn stroke_triangle(
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

        self.draw_color(&lyon_vbuff_to_vertex(output), None)
    }

    fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let x2 = x + width;
        let y2 = y + height;
        let vertices = [x, y, x2, y, x, y2, x, y2, x2, y, x2, y2];

        self.draw_color(&vertices, None);
    }

    fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32, line_width: f32) {
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

    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, strength: f32) {
        let (mut xx, mut yy) = if eq_float(y1, y2) {
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

    fn circle(&mut self, x: f32, y: f32, radius: f32) {
        self.draw_color(&get_circle_vertices(x, y, radius, None), None);
    }

    fn rounded_rect(&mut self, x: f32, y: f32, width: f32, height: f32, radius: f32) {
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

    fn stroke_rounded_rect(
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

    fn stroke_circle(&mut self, x: f32, y: f32, radius: f32, line_width: f32) {
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

    fn geometry(&mut self, geometry: &mut Geometry) {
        geometry.build();
        if let Some((v, vc)) = &geometry.vertices {
            self.draw_color(v, Some(vc));
        }
    }

    fn image(&mut self, img: &Self::Texture, x: f32, y: f32) {
        self.image_ext(img, x, y, img.width(), img.height(), 0.0, 0.0, 0.0, 0.0);
    }

    fn image_crop(
        &mut self,
        img: &Self::Texture,
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

    fn image_ext(
        &mut self,
        img: &Self::Texture,
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

    fn pattern(
        &mut self,
        img: &Self::Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        self.pattern_ext(img, x, y, width, height, offset_x, offset_y, 1.0, 1.0);
    }

    fn pattern_ext(
        &mut self,
        img: &Self::Texture,
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

    fn vertex(&mut self, vertices: &[Vertex]) {
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

    fn image_9slice(&mut self, img: &Self::Texture, x: f32, y: f32, width: f32, height: f32) {
        let ww = img.width() / 3.0;
        let hh = img.height() / 3.0;
        self.image_9slice_ext(img, x, y, width, height, ww, ww, hh, hh);
    }

    fn image_9slice_ext(
        &mut self,
        img: &Self::Texture,
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

pub(crate) struct DrawData {
    pub alpha: f32,
    pub color: Color,
    pub shader: Option<Shader>,
    pub transform: Transform2d,
    pub width: i32,
    pub height: i32,
    pub flipped: bool,
    pub projection: Mat3,
    pub dpi: f32,
}

impl DrawData {
    pub fn new(width: i32, height: i32, dpi: f32) -> Self {
        let projection = projection_2d(width, height, false, dpi);
        let transform = Transform2d::new();
        Self {
            width,
            height,
            alpha: 1.0,
            shader: None,
            transform,
            color: Color::WHITE,
            projection,
            flipped: false,
            dpi,
        }
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
        self.projection = projection_2d(self.width, self.height, flipped, self.dpi);
    }
}

#[derive(Debug, Eq, PartialEq)]
enum PaintMode {
    Color,
    Image,
    Text,
    Empty,
}

#[cfg(target_arch = "wasm32")]
fn create_gl_context(win: &web_sys::HtmlCanvasElement) -> Result<(GlContext, String), String> {
    if let Ok(ctx) = create_webgl2_context(win) {
        return Ok((ctx, String::from("webgl2")));
    }

    let ctx = create_webgl_context(win)?;
    Ok((ctx, String::from("webgl")))
}

#[cfg(not(target_arch = "wasm32"))]
fn create_gl_context(device: &()) -> Result<(GlContext, String), String> {
    unimplemented!()
}

#[cfg(target_arch = "wasm32")]
fn webgl_options() -> web_sys::WebGlContextAttributes {
    let mut opts = web_sys::WebGlContextAttributes::new();
    opts.stencil(true);
    opts.premultiplied_alpha(false);
    opts.alpha(false);
    opts.antialias(true);
    opts
}

#[cfg(target_arch = "wasm32")]
fn create_webgl_context(win: &web_sys::HtmlCanvasElement) -> Result<GlContext, String> {
    //TODO manage errors
    let gl = win
        .get_context_with_context_options("webgl", webgl_options().as_ref())
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGlRenderingContext>()
        .unwrap();

    let ctx = Rc::new(glow::Context::from_webgl1_context(gl));
    Ok(ctx)
}

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
fn create_context_2d(win: &web_sys::HtmlCanvasElement) -> Result<Context2d, String> {
    let width = win.width() as _;
    let height = win.height() as _;
    let (gl, driver) = create_gl_context(win)?;
    let data = DrawData::new(width, height, 1.0);
    let blend_mode = BlendMode::NORMAL;

    let text_batcher = TextBatcher::new(&gl)?;
    let sprite_batcher = SpriteBatcher::new(&gl)?;
    let color_batcher = ColorBatcher::new(&gl)?;

    initialize_gl_2d(&gl, blend_mode);

    Ok(Context2d {
        data,
        gl,
        text_batcher,
        sprite_batcher,
        color_batcher,
        blend_mode,
        is_drawing: false,
        is_drawing_surface: false,
        paint_mode: PaintMode::Empty,
        stencil: false,
        width,
        height,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn create_context_2d(win_ctx: &WindowedContext<PossiblyCurrent>) -> Result<Context2d, String> {
    let win: &glutin::Window = win_ctx.window();
    let (width, height) = if let Some(size) = win.get_inner_size() {
        (size.width as _, size.height as _)
    } else {
        (800, 600)
    };

    let ctx = glow::Context::from_loader_function(|s| win_ctx.get_proc_address(s) as *const _);

    let gl = Rc::new(ctx);
    let blend_mode = BlendMode::NORMAL;

    let text_batcher = TextBatcher::new(&gl)?;
    let sprite_batcher = SpriteBatcher::new(&gl)?;
    let color_batcher = ColorBatcher::new(&gl)?;

    let dpi = win_ctx.window().get_hidpi_factor() as f32;
    let data = DrawData::new(width, height, dpi);

    initialize_gl_2d(&gl, blend_mode);

    Ok(Context2d {
        data,
        gl,
        text_batcher,
        sprite_batcher,
        color_batcher,
        blend_mode,
        is_drawing: false,
        is_drawing_surface: false,
        paint_mode: PaintMode::Empty,
        stencil: false,
        width,
        height,
    })
}

fn initialize_gl_2d(gl: &GlContext, blend_mode: BlendMode) {
    unsafe {
        gl.disable(glow::DEPTH_TEST);
        gl.enable(glow::BLEND);
        gl.blend_func(
            blend_mode.source().glow_value(),
            blend_mode.destination().glow_value(),
        );
    }
}

impl GlowValue for BlendFactor {
    fn glow_value(&self) -> u32 {
        use BlendFactor::*;
        match self {
            One => glow::ONE,
            Zero => glow::ZERO,
            SourceColor => glow::SRC_COLOR,
            InverseSourceColor => glow::ONE_MINUS_SRC_COLOR,
            DestinationColor => glow::DST_COLOR,
            InverseDestinationColor => glow::ONE_MINUS_DST_COLOR,
            SourceAlpha => glow::SRC_ALPHA,
            InverseSourceAlpha => glow::ONE_MINUS_SRC_ALPHA,
            DestinationAlpha => glow::DST_ALPHA,
            InverseDestinationAlpha => glow::ONE_MINUS_DST_ALPHA,
        }
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
