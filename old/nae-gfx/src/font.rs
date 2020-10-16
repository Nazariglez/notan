use crate::texture::{max_texture_size, texture_from_gl_context, Texture, TextureOptions};
use crate::{Draw, GlContext, Graphics};
use glow::HasContext;
use glyph_brush::rusttype::Scale;
use glyph_brush::{
    BrushAction, BrushError, FontId, GlyphBrush, GlyphBrushBuilder, GlyphCruncher, GlyphVertex,
    Section,
};
use nae_core::{
    BaseApp, BaseSystem, HorizontalAlign, Resource, TextureFilter, TextureFormat, VerticalAlign,
};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Font {
    inner: Rc<RefCell<InnerFont>>,
}

impl PartialEq for Font {
    fn eq(&self, other: &Self) -> bool {
        self.raw() == other.raw()
    }
}

impl Font {
    pub(crate) fn raw(&self) -> Option<FontId> {
        self.inner.borrow().id
    }

    /// Create a new font from bytes
    pub fn from_bytes<T, S>(app: &mut T, data: &[u8]) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics, Draw = Draw>,
    {
        let mut font = Font {
            inner: Rc::new(RefCell::new(InnerFont { id: None })),
        };

        font.set_data(app, data.to_vec());
        Ok(font)
    }

    /// Returns the width and height for the text passed with the current size
    pub fn size(
        &self,
        draw: &mut Draw,
        text: &str,
        size: f32,
        max_width: Option<f32>,
    ) -> (f32, f32) {
        let h_align = draw.text_horizontal_align;
        let v_align = draw.text_vertical_align;
        draw.text_batcher
            .text_size(self, text, size, h_align, v_align, max_width)
    }

    /// Returns if the resource is already loaded
    pub fn is_loaded(&self) -> bool {
        self.inner.borrow().id.is_some()
    }
}

impl<T, S> Resource<T> for Font
where
    T: BaseApp<System = S>,
    S: BaseSystem<Draw = Draw>,
{
    fn prepare(app: &mut T, _file: &str) -> Result<Self, String> {
        Ok(Font {
            inner: Rc::new(RefCell::new(InnerFont { id: None })),
        })
    }

    fn set_data(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String> {
        let id = add_font(app.system().draw(), data);
        *self.inner.borrow_mut() = InnerFont {
            id: Some(FontId(id)),
        };

        Ok(())
    }
}

fn add_font(draw: &mut Draw, data: Vec<u8>) -> usize {
    draw.text_batcher.add_font(data)
}

struct InnerFont {
    id: Option<FontId>,
}

#[inline]
fn h_align_to_glyph(align: HorizontalAlign) -> glyph_brush::HorizontalAlign {
    match align {
        HorizontalAlign::Left => glyph_brush::HorizontalAlign::Left,
        HorizontalAlign::Center => glyph_brush::HorizontalAlign::Center,
        HorizontalAlign::Right => glyph_brush::HorizontalAlign::Right,
    }
}

#[inline]
fn v_align_to_glyph(align: VerticalAlign) -> glyph_brush::VerticalAlign {
    match align {
        VerticalAlign::Top => glyph_brush::VerticalAlign::Top,
        VerticalAlign::Center => glyph_brush::VerticalAlign::Center,
        VerticalAlign::Bottom => glyph_brush::VerticalAlign::Bottom,
    }
}

#[inline]
fn max_suggest_size(
    max_texture_dimensions: u32,
    suggested: (u32, u32),
    dimensions: (u32, u32),
) -> (u32, u32) {
    let suggested_bigger =
        (suggested.0 > max_texture_dimensions || suggested.1 > max_texture_dimensions);
    let dimensions_smaller =
        (dimensions.0 < max_texture_dimensions || dimensions.1 < max_texture_dimensions);
    if suggested_bigger && dimensions_smaller {
        (max_texture_dimensions, max_texture_dimensions)
    } else {
        suggested
    }
}

// Manager
pub(crate) struct FontManager<'a> {
    cache: GlyphBrush<'a, FontTextureData>,
    max_texture_size: i32,
    pub texture: Texture,
    pub data: Vec<FontTextureData>,
    pub width: u32,
    pub height: u32,
}

impl<'a> FontManager<'a> {
    pub fn new(gl: &GlContext) -> Result<Self, String> {
        let cache = GlyphBrushBuilder::without_fonts().build();
        let (width, height) = cache.texture_dimensions();
        let texture = texture_from_gl_context(
            gl,
            width as _,
            height as _,
            &TextureOptions {
                format: TextureFormat::Rgba,
                internal_format: TextureFormat::Rgba,
                min_filter: TextureFilter::Nearest,
                mag_filter: TextureFilter::Nearest,
            },
        )?;

        let max_texture_size = max_texture_size(gl);

        Ok(Self {
            cache,
            texture,
            max_texture_size,
            width,
            height,
            data: vec![],
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
            font_id: font.raw().unwrap(),
            bounds: (max_width.unwrap_or(std::f32::INFINITY), std::f32::INFINITY),
            layout: glyph_brush::Layout::default()
                .h_align(h_align_to_glyph(h_align))
                .v_align(v_align_to_glyph(v_align)),
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
        z: f32,
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
            z,
            scale: Scale::uniform(size),
            font_id: font.raw().unwrap(),
            bounds: (max_width, std::f32::INFINITY),
            layout: glyph_brush::Layout::default()
                .h_align(h_align_to_glyph(h_align))
                .v_align(v_align_to_glyph(v_align)),
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
                        &TextureOptions {
                            format: TextureFormat::Red,
                            internal_format: TextureFormat::R8,
                            min_filter: TextureFilter::Linear,
                            mag_filter: TextureFilter::Linear,
                        },
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
        gl.bind_texture(glow::TEXTURE_2D, texture.raw());

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

#[derive(Debug, Clone)]
pub(crate) struct FontTextureData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub source_x: f32,
    pub source_y: f32,
    pub source_width: f32,
    pub source_height: f32,
    pub color: [f32; 4],
}

fn glyph_to_data(v: &GlyphVertex, width: f32, height: f32) -> FontTextureData {
    let sx = v.tex_coords.min.x * width;
    let sy = v.tex_coords.min.y * height;
    FontTextureData {
        x: v.pixel_coords.min.x as _,
        y: v.pixel_coords.min.y as _,
        z: v.z,
        source_x: sx,
        source_y: sy,
        source_width: v.tex_coords.max.x * width - sx,
        source_height: v.tex_coords.max.y * height - sy,
        color: v.color,
    }
}
