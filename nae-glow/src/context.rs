use crate::batchers::TextBatcher;
use crate::font::Font;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::{GlContext, Surface};
use nae_core::graphics::{BaseContext2d, BlendMode, Color, Geometry, Transform2d, Vertex};
use nae_core::math::*;
use nae_core::resources::{BaseFont, HorizontalAlign, VerticalAlign};

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

pub struct Context2d {
    pub(crate) gl: GlContext,
    pub(crate) text_batcher: TextBatcher,
}

impl BaseContext2d for Context2d {
    type Shader = Shader;
    type Surface = Surface;
    type Texture = Texture;
    type Font = Font;

    fn set_shader(&mut self, shader: Option<&Self::Shader>) {
        unimplemented!()
    }

    fn update_custom_shader(&mut self, shader: Option<&Self::Shader>) {
        unimplemented!()
    }

    fn set_alpha(&mut self, alpha: f32) {
        unimplemented!()
    }

    fn set_blend(&mut self, mode: BlendMode) {
        unimplemented!()
    }

    fn set_size(&mut self, width: i32, height: i32) {
        unimplemented!()
    }

    fn width(&self) -> i32 {
        unimplemented!()
    }

    fn height(&self) -> i32 {
        unimplemented!()
    }

    fn set_color(&mut self, color: Color) {
        unimplemented!()
    }

    fn transform(&mut self) -> &mut Transform2d {
        unimplemented!()
    }

    fn begin_to_surface(&mut self, surface: Option<&Self::Surface>) {
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
