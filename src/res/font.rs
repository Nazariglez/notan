use super::Texture;
use super::{Resource, ResourceConstructor};
use crate::app::App;
use crate::graphics::color::Color;
use crate::graphics::GlContext;
use crate::log;
use crate::res::update_texture;
use glow::HasContext;
use glyph_brush::rusttype::Scale;
use glyph_brush::{
    BrushAction, BrushError, FontId, GlyphBrush, GlyphBrushBuilder, GlyphVertex, Section,
};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::__rt::core::cmp::max;

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
        let id = app.graphics.add_font(data);
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

fn max_texture_size(gl: &GlContext) -> i32 {
    unsafe { gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE) }
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
    const DEFAULT_DATA: &'a [u8] = include_bytes!("../../assets/ubuntu/Ubuntu-B.ttf");

    pub fn new(gl: &GlContext) -> Result<Self, String> {
        let cache = GlyphBrushBuilder::using_font_bytes(FontManager::DEFAULT_DATA).build();
        let (width, height) = cache.texture_dimensions();
        let texture = Texture::from_size(gl, width as _, height as _)?;
        Ok(Self {
            cache,
            texture,
            data: vec![],
            max_texture_size: max_texture_size(gl),
            width: width,
            height: height,
        })
    }

    pub(crate) fn add(&mut self, data: Vec<u8>) -> usize {
        self.cache.add_font_bytes(data).0
    }

    pub fn texture_dimensions(&self) -> (u32, u32) {
        self.cache.texture_dimensions()
    }

    //https://github.com/17cupsofcoffee/tetra/blob/master/src/graphics/text.rs#L178
    pub fn try_update(&mut self, gl: &GlContext, id: FontId, text: &str, size: f32) {
        self.cache.queue(create_section(id, text, size));

        //        let width = self.width as _;
        //        let height = self.height as _;
        let texture = &mut self.texture;
        let action = loop {
            let try_action = self.cache.process_queued(
                |rect, data| update_texture(gl, &texture, rect, data),
                |v| glyph_to_data(&v, texture.width(), texture.height()),
            );

            match try_action {
                Ok(a) => break a,
                Err(BrushError::TextureTooSmall { suggested }) => {
                    let (width, height) = max_suggest_size(
                        self.max_texture_size as u32,
                        suggested,
                        self.cache.texture_dimensions(),
                    );
                    log(&format!("{} {}", width, height));
                    *texture = Texture::from_size(gl, width as _, height as _).unwrap();
                    self.cache.resize_texture(width, height);
                }
            }
        };

        match action {
            BrushAction::Draw(data_list) => {
                self.data = data_list;
                //                log("lilili");
            }
            _ => {} //            BrushAction::ReDraw => log("telele..."),
        }
    }

    pub fn queue(&mut self, font: &Font, x: f32, y: f32, text: &str, size: f32, color: [f32; 4]) {
        let section = Section {
            text,
            screen_position: (x, y),
            scale: Scale::uniform(size),
            font_id: font.id(),
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
                    *texture = Texture::from_size(gl, width as _, height as _).unwrap();
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
    log(&format!(
        "{} {:?} {:?}",
        max_texture_dimensions, suggested, dimensions
    ));
    if (suggested.0 > max_texture_dimensions || suggested.1 > max_texture_dimensions)
        && (dimensions.0 < max_texture_dimensions || dimensions.1 < max_texture_dimensions)
    {
        (max_texture_dimensions, max_texture_dimensions)
    } else {
        suggested
    }
}

fn create_section(font_id: FontId, text: &str, size: f32) -> Section {
    Section {
        text,
        scale: Scale::uniform(size),
        font_id,
        ..Section::default()
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
