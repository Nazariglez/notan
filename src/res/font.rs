use super::Texture;
use super::{Resource, ResourceConstructor};
use crate::app::App;
use crate::graphics::GlContext;
use crate::log;
use crate::res::update_texture;
use glyph_brush::rusttype::Scale;
use glyph_brush::{
    BrushAction, BrushError, FontId, GlyphBrush, GlyphBrushBuilder, GlyphVertex, Section,
};
use std::cell::RefCell;
use std::rc::Rc;

struct InnerFont {
    id: FontId,
    loaded: bool,
}

#[derive(Clone)]
pub struct Font {
    inner: Rc<RefCell<InnerFont>>,
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
            })),
        }
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

impl Resource for Font {
    fn parse(&mut self, app: &mut App, data: Vec<u8>) -> Result<(), String> {
        let id = app.graphics.font_manager.add(data);
        *self.inner.borrow_mut() = InnerFont { id, loaded: true };
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.inner.borrow().loaded
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FontQuad {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub u1: f32,
    pub v1: f32,
    pub u2: f32,
    pub v2: f32,
}

pub(crate) struct FontManager<'a> {
    cache: GlyphBrush<'a, FontQuad>,
    pub(crate) texture: Texture,
    pub(crate) quads: Vec<FontQuad>,
}

impl<'a> FontManager<'a> {
    const DEFAULT: &'a [u8] = include_bytes!("../../assets/ubuntu/Ubuntu-B.ttf");

    pub fn new(gl: &GlContext) -> Result<Self, String> {
        let cache = GlyphBrushBuilder::using_font_bytes(FontManager::DEFAULT).build();
        let (width, height) = cache.texture_dimensions();
        let texture = Texture::from_size(gl, width as _, height as _)?;
        Ok(Self {
            cache,
            texture,
            quads: vec![],
        })
    }

    fn add(&mut self, data: Vec<u8>) -> FontId {
        self.cache.add_font_bytes(data)
    }

    //https://github.com/17cupsofcoffee/tetra/blob/master/src/graphics/text.rs#L178
    pub fn try_update(&mut self, gl: &GlContext, id: FontId, text: &str, size: f32) {
        self.cache.queue(create_section(id, text, size));

        let texture = &mut self.texture;
        let action = loop {
            let try_action = self.cache.process_queued(
                |rect, data| update_texture(gl, &texture, rect, data),
                |v| glyph_to_quad(&v),
            );

            match try_action {
                Ok(a) => break a,
                Err(BrushError::TextureTooSmall { suggested, .. }) => {
                    let (width, height) = suggested;
                    log(&format!("{} {}", width, height));
                    *texture = Texture::from_size(gl, width as _, height as _).unwrap();
                    self.cache.resize_texture(width, height);
                }
            }
        };

        if let BrushAction::Draw(quads) = action {
            //            log(&format!("draw... {} {:#?}", quads.len(), quads));
            //https://github.com/17cupsofcoffee/tetra/blob/master/src/graphics/text.rs#L197
            //TODO update quad
            self.quads = quads;
        }
    }
}

fn create_section(id: FontId, text: &str, size: f32) -> Section {
    Section {
        text: text,
        scale: Scale::uniform(size * 100.0),
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
