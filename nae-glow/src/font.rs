use nae_core::resources::{BaseFont, VerticalAlign, HorizontalAlign, Resource, ResourceConstructor};
use nae_core::BaseApp;
use crate::context::Context2d;
use std::rc::Rc;
use std::cell::RefCell;
use super::texture::Texture;
use glow::HasContext;
use glyph_brush::rusttype::Scale;
use glyph_brush::{
    BrushAction, BrushError, FontId, GlyphBrush, GlyphBrushBuilder, GlyphCruncher, GlyphVertex,
    Section,
};

#[derive(Clone)]
pub struct Font {
    inner: Rc<RefCell<InnerFont>>
}

impl BaseFont for Font {
    fn text_size<T: BaseApp<Graphics = Self::Context2d>, F: BaseFont>(app: &mut T, font: &F, text: &str, size: f32) -> (f32, f32) {
        <Font as BaseFont>::text_size_ext(
            app,
            font,
            text,
            size,
            HorizontalAlign::Left,
            VerticalAlign::Top,
            None,
        )
    }

    fn text_size_ext<T: BaseApp<Graphics = Self::Context2d>, F: BaseFont>(app: &mut T, font: &F, text: &str, size: f32, h_align: HorizontalAlign, v_align: VerticalAlign, max_width: Option<f32>) -> (f32, f32) {
        text_size(app.graphics(), font, text, size, h_align, v_align, max_width)
    }
}

impl Resource for Font {
    type Context2d = Context2d;

    fn parse<T: BaseApp<Graphics = Self::Context2d>>(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String> {
        let id = add_font(app.graphics(), data);
        *self.inner.borrow_mut() = InnerFont {
            id: FontId(id),
            loaded: true,
        };
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.inner.borrow().loaded
    }
}

impl ResourceConstructor for Font {
    fn new(file: &str) -> Self {
        unimplemented!()
    }
}

struct InnerFont {
    id: FontId,
    loaded: bool,
}

fn text_size<F: BaseFont>(ctx: &mut Context2d, font: &F, text: &str, size: f32, h_align: HorizontalAlign, v_align: VerticalAlign, max_width: Option<f32>) -> (f32, f32) {
    unimplemented!()
//    let size = ctx
//        .text_batcher
//        .manager
//        .text_size(font, text, size, h_align, v_align, max_width);
//    (size.0 as _, size.1 as _)
}

fn add_font(ctx: &mut Context2d, data: Vec<u8>) -> usize {
    unimplemented!()
}