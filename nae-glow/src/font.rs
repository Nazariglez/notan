use super::texture::Texture;
use crate::context::Context2d;
use crate::texture::{max_texture_size, texture_from_gl_context};
use crate::GlContext;
use glow::HasContext;
use glyph_brush::rusttype::Scale;
use glyph_brush::{
    BrushAction, BrushError, FontId, GlyphBrush, GlyphBrushBuilder, GlyphCruncher, GlyphVertex,
    Section,
};
use nae_core::resources::{
    BaseFont, BaseTexture, HorizontalAlign, Resource, ResourceConstructor, TextureFilter,
    TextureFormat, VerticalAlign,
};
use nae_core::BaseSystem;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Font {
    inner: Rc<RefCell<InnerFont>>,
}

impl Font {
    pub(crate) fn id(&self) -> FontId {
        self.inner.borrow().id
    }
}

impl BaseFont for Font {
    type Kind = Self;

    fn text_size<T: BaseSystem<Context2d = Self::Context2d>>(
        app: &mut T,
        font: &Font,
        text: &str,
        size: f32,
    ) -> (f32, f32) {
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

    fn text_size_ext<T: BaseSystem<Context2d = Self::Context2d>>(
        app: &mut T,
        font: &Font,
        text: &str,
        size: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        max_width: Option<f32>,
    ) -> (f32, f32) {
        text_size(app.ctx2(), font, text, size, h_align, v_align, max_width)
    }
}

impl Default for Font {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerFont {
                id: FontId(0),
                loaded: true,
            })),
        }
    }
}

impl Resource for Font {
    type Context2d = Context2d;

    fn parse<T: BaseSystem<Context2d = Self::Context2d>>(
        &mut self,
        app: &mut T,
        data: Vec<u8>,
    ) -> Result<(), String> {
        let id = add_font(app.ctx2(), data);
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
        Self {
            inner: Rc::new(RefCell::new(InnerFont {
                id: FontId(0),
                loaded: false,
            })),
        }
    }
}

struct InnerFont {
    id: FontId,
    loaded: bool,
}

fn text_size(
    ctx: &mut Context2d,
    font: &Font,
    text: &str,
    size: f32,
    h_align: HorizontalAlign,
    v_align: VerticalAlign,
    max_width: Option<f32>,
) -> (f32, f32) {
    let size = ctx
        .text_batcher
        .manager
        .text_size(font, text, size, h_align, v_align, max_width);
    (size.0 as _, size.1 as _)
}

fn add_font(ctx: &mut Context2d, data: Vec<u8>) -> usize {
    ctx.text_batcher.manager.add(data)
}

#[derive(Debug, Clone)]
pub(crate) struct FontTextureData {
    pub x: f32,
    pub y: f32,
    pub source_x: f32,
    pub source_y: f32,
    pub source_width: f32,
    pub source_height: f32,
    pub color: [f32; 4],
}

pub(crate) struct FontManager<'a> {
    cache: GlyphBrush<'a, FontTextureData>,
    max_texture_size: i32,
    pub texture: Texture,
    pub data: Vec<FontTextureData>,
    pub width: u32,
    pub height: u32,
}

impl<'a> FontManager<'a> {
    const DEFAULT_DATA: &'a [u8] = include_bytes!("../resources/ubuntu/Ubuntu-B.ttf");

    pub fn new(gl: &GlContext) -> Result<Self, String> {
        let cache = GlyphBrushBuilder::using_font_bytes(FontManager::DEFAULT_DATA).build();
        let (width, height) = cache.texture_dimensions();
        let texture = texture_from_gl_context(
            gl,
            width as _,
            height as _,
            TextureFormat::Rgba,
            TextureFormat::Rgba,
            TextureFilter::Nearest,
            TextureFilter::Nearest,
        )?;
        Ok(Self {
            cache,
            texture,
            data: vec![],
            max_texture_size: max_texture_size(gl),
            width: width,
            height: height,
        })
    }

    pub fn add(&mut self, data: Vec<u8>) -> usize {
        self.cache.add_font_bytes(data).0
    }

    pub fn texture_dimensions(&self) -> (u32, u32) {
        self.cache.texture_dimensions()
    }

    pub fn text_size(
        &mut self,
        font: &Font,
        text: &str,
        size: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        max_width: Option<f32>,
    ) -> (f32, f32) {
        let section = Section {
            text,
            scale: Scale::uniform(size),
            font_id: font.id(),
            bounds: (max_width.unwrap_or(std::f32::INFINITY), std::f32::INFINITY),
            layout: glyph_brush::Layout::default()
                .h_align(h_align.glyph_value())
                .v_align(v_align.glyph_value()),
            ..Section::default()
        };

        if let Some(bounds) = self.cache.glyph_bounds(section) {
            return (bounds.width(), bounds.height());
        }

        (0.0, 0.0)
    }

    pub fn queue(
        &mut self,
        font: &Font,
        x: f32,
        y: f32,
        text: &str,
        size: f32,
        color: [f32; 4],
        max_width: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
    ) {
        let section = Section {
            text,
            screen_position: (x, y),
            scale: Scale::uniform(size),
            font_id: font.id(),
            bounds: (max_width, std::f32::INFINITY),
            layout: glyph_brush::Layout::default()
                .h_align(h_align.glyph_value())
                .v_align(v_align.glyph_value()),
            color,
            ..Section::default()
        };
        self.cache.queue(section);
    }

    pub fn process_queue(
        &mut self,
        gl: &GlContext,
        texture: &mut Texture,
    ) -> Option<Vec<FontTextureData>> {
        let action = loop {
            let try_action = self.cache.process_queued(
                |rect, data| update_texture(gl, texture, rect, data),
                |vert| glyph_to_data(&vert, texture.width(), texture.height()),
            );

            match try_action {
                Ok(action) => break action,
                Err(BrushError::TextureTooSmall { suggested }) => {
                    let (width, height) = max_suggest_size(
                        self.max_texture_size as u32,
                        suggested,
                        self.cache.texture_dimensions(),
                    );
                    *texture = texture_from_gl_context(
                        gl,
                        width as _,
                        height as _,
                        TextureFormat::R8,
                        TextureFormat::Red,
                        TextureFilter::Linear,
                        TextureFilter::Linear,
                    )
                    .unwrap();
                    self.cache.resize_texture(width, height);
                }
            }
        };

        if let BrushAction::Draw(data) = action {
            return Some(data);
        }

        None
    }
}

fn max_suggest_size(
    max_texture_dimensions: u32,
    suggested: (u32, u32),
    dimensions: (u32, u32),
) -> (u32, u32) {
    if (suggested.0 > max_texture_dimensions || suggested.1 > max_texture_dimensions)
        && (dimensions.0 < max_texture_dimensions || dimensions.1 < max_texture_dimensions)
    {
        (max_texture_dimensions, max_texture_dimensions)
    } else {
        suggested
    }
}

fn glyph_to_data(v: &GlyphVertex, width: f32, height: f32) -> FontTextureData {
    let sx = v.tex_coords.min.x * width;
    let sy = v.tex_coords.min.y * height;
    FontTextureData {
        x: v.pixel_coords.min.x as _,
        y: v.pixel_coords.min.y as _,
        source_x: sx,
        source_y: sy,
        source_width: v.tex_coords.max.x * width - sx,
        source_height: v.tex_coords.max.y * height - sy,
        color: v.color,
    }
}

fn update_texture(
    gl: &GlContext,
    texture: &Texture,
    rect: glyph_brush::rusttype::Rect<u32>,
    data: &[u8],
) {
    let xx = rect.min.x as i32;
    let yy = rect.min.y as i32;
    let ww = rect.width() as i32;
    let hh = rect.height() as i32;

    unsafe {
        gl.bind_texture(glow::TEXTURE_2D, texture.tex());

        gl.tex_sub_image_2d_u8_slice(
            glow::TEXTURE_2D,
            0,
            xx,
            yy,
            ww,
            hh,
            glow::RED,
            glow::UNSIGNED_BYTE,
            Some(data),
        );
    }
}

trait AlignToGlyph<T> {
    fn glyph_value(&self) -> T;
}

impl AlignToGlyph<glyph_brush::HorizontalAlign> for HorizontalAlign {
    fn glyph_value(&self) -> glyph_brush::HorizontalAlign {
        match self {
            HorizontalAlign::Left => glyph_brush::HorizontalAlign::Left,
            HorizontalAlign::Center => glyph_brush::HorizontalAlign::Center,
            HorizontalAlign::Right => glyph_brush::HorizontalAlign::Right,
        }
    }
}

impl AlignToGlyph<glyph_brush::VerticalAlign> for VerticalAlign {
    fn glyph_value(&self) -> glyph_brush::VerticalAlign {
        match self {
            VerticalAlign::Top => glyph_brush::VerticalAlign::Top,
            VerticalAlign::Center => glyph_brush::VerticalAlign::Center,
            VerticalAlign::Bottom => glyph_brush::VerticalAlign::Bottom,
        }
    }
}
