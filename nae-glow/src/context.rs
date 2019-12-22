use crate::batchers::{ColorBatcher, SpriteBatcher, TextBatcher};
use crate::font::Font;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::{GlContext, GlowValue, Surface};
use glow::HasContext;
use glyph_brush::BrushAction::Draw;
use nae_core::graphics::{
    BaseContext2d, BaseShader, BlendFactor, BlendMode, Color, Geometry, Transform2d, Vertex,
};
use nae_core::math::*;
use nae_core::resources::{BaseFont, HorizontalAlign, VerticalAlign};
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

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

#[cfg(target_arch = "wasm32")]
type Device = web_sys::HtmlCanvasElement;

#[cfg(not(target_arch = "wasm32"))]
type Device = ();

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
        unimplemented!()
    }

    fn begin(&mut self) {
        unimplemented!()
    }

    fn end(&mut self) {
        unimplemented!()
    }

    fn clear(&mut self, color: Color) {
        unimplemented!()
    }

    fn begin_mask(&mut self) {
        unimplemented!()
    }

    fn end_mask(&mut self) {
        unimplemented!()
    }

    fn clear_mask(&mut self) {
        unimplemented!()
    }

    fn flush(&mut self) {
        unimplemented!()
    }

    fn set_font(&mut self, font: &Self::Font) {
        unimplemented!()
    }

    fn font(&self) -> &Self::Font {
        unimplemented!()
    }

    fn text(&mut self, text: &str, x: f32, y: f32, size: f32) {
        unimplemented!()
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
        unimplemented!()
    }

    fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        unimplemented!()
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
        unimplemented!()
    }

    fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        unimplemented!()
    }

    fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32, line_width: f32) {
        unimplemented!()
    }

    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, strength: f32) {
        unimplemented!()
    }

    fn circle(&mut self, x: f32, y: f32, radius: f32) {
        unimplemented!()
    }

    fn rounded_rect(&mut self, x: f32, y: f32, width: f32, height: f32, radius: f32) {
        unimplemented!()
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
        unimplemented!()
    }

    fn stroke_circle(&mut self, x: f32, y: f32, radius: f32, line_width: f32) {
        unimplemented!()
    }

    fn geometry(&mut self, geometry: &mut Geometry) {
        unimplemented!()
    }

    fn image(&mut self, img: &Self::Texture, x: f32, y: f32) {
        unimplemented!()
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
        unimplemented!()
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
        unimplemented!()
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
        unimplemented!()
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
        unimplemented!()
    }

    fn vertex(&mut self, vertices: &[Vertex]) {
        unimplemented!()
    }

    fn image_9slice(&mut self, img: &Self::Texture, x: f32, y: f32, width: f32, height: f32) {
        unimplemented!()
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
        unimplemented!()
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
}

impl DrawData {
    pub fn new(width: i32, height: i32) -> Self {
        let projection = projection_2d(width, height, false);
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
        self.projection = projection_2d(self.width, self.height, flipped);
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
    let data = DrawData::new(width, height);
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
fn create_context_2d(win: &()) -> Result<Context2d, String> {
    unimplemented!()
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
