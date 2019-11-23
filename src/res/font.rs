use std::rc::Rc;
use std::cell::RefCell;
use super::{ResourceConstructor, Resource};
use crate::app::App;
use super::Texture;
use glyph_brush::{GlyphBrushBuilder, GlyphBrush, FontId, Section, GlyphVertex, BrushAction, BrushError};
use glyph_brush::rusttype::Scale;
use crate::graphics::GlContext;

struct InnerFont {
    id: FontId,
    loaded: bool,
}

#[derive(Clone)]
pub struct Font {
    inner: Rc<RefCell<InnerFont>>
}

impl Font {
    pub(crate) fn id(&self) -> FontId {
        self.inner.borrow().id
    }
}

impl Default for Font {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerFont {
                id: FontId(0),
                loaded: true,
            }))
        }
    }
}

impl ResourceConstructor for Font {
    fn new(file: &str) -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerFont {
                id: FontId(0),
                loaded: false
            }))
        }
    }
}

impl Resource for Font {
    fn parse(&mut self, app: &mut App, data: Vec<u8>) -> Result<(), String> {
        let id = app.graphics.font_manager.add(data);
        *self.inner.borrow_mut() = InnerFont {
            id,
            loaded: true,
        };
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.inner.borrow().loaded
    }
}

#[derive(Clone)]
pub(crate) struct FontQuad {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32,
}

pub(crate) struct FontManager<'a> {
    cache: GlyphBrush<'a, FontQuad>,
    texture: Texture,
}

impl<'a> FontManager<'a> {
    const DEFAULT:&'a [u8] = include_bytes!("../../assets/ubuntu/Ubuntu-B.ttf");

    pub fn new() -> Self {
        let cache = GlyphBrushBuilder::using_font_bytes(FontManager::DEFAULT).build();
        let texture = Texture::new("");
        Self {
            cache,
            texture
        }
    }

    fn add(&mut self, data: Vec<u8>) -> FontId {
        self.cache.add_font_bytes(data)
    }

    //https://github.com/17cupsofcoffee/tetra/blob/master/src/graphics/text.rs#L178
    pub fn try_update(&mut self, gl: &GlContext, id: FontId, text: &str, size: f32) {
        self.cache.queue(create_section(id, text, size));

        let action = loop {
            let try_action = self.cache.process_queued(
                |rect, data| {},
                |v| glyph_to_quad(&v),
            );

            match try_action {
                Ok(a) => break a,
                Err(BrushError::TextureTooSmall { suggested, .. }) => {
                    let (width, height) = suggested;
                    self.texture = Texture::from_size(gl, width as _, height as _).unwrap();
                    self.cache.resize_texture(width, height);
                }
            }
        };

        if let BrushAction::Draw(quad) = action {
            //https://github.com/17cupsofcoffee/tetra/blob/master/src/graphics/text.rs#L197
            //TODO update quad
        }
    }
}

fn create_section(id: FontId, text: &str, size: f32) -> Section {
    Section {
        text: text,
        scale: Scale::uniform(size),
        font_id: id,
        ..Section::default()
    }
}

fn glyph_to_quad(v: &GlyphVertex) -> FontQuad {
    FontQuad {
        x1: v.pixel_coords.min.x as f32,
        y1: v.pixel_coords.min.y as f32,
        x2: v.pixel_coords.max.x as f32,
        y2: v.pixel_coords.max.y as f32,
        u1: v.tex_coords.min.x as f32,
        v1: v.tex_coords.min.y as f32,
        u2: v.tex_coords.max.x as f32,
        v2: v.tex_coords.max.y as f32,
    }
}