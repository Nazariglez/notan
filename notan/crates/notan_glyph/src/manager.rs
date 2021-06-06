use crate::font::Font;
use crate::render::{DefaultFontRenderer, FontRender};
use crate::text::{section_from_text, Text};
use glyph_brush::{ab_glyph::*, *};
use notan_graphics::prelude::*;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct FontManager<R>
where
    R: FontRender,
{
    pub cache: GlyphBrush<[f32; 13], Extra, FontRef<'static>>,
    pub texture: Texture,
    pub render: R,
}

impl FontManager<DefaultFontRenderer> {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let render = DefaultFontRenderer::new(device)?;
        Self::with_render(device, render)
    }
}

impl<R: FontRender> FontManager<R> {
    pub fn with_render(device: &mut Device, render: R) -> Result<Self, String> {
        let cache = GlyphBrushBuilder::using_fonts::<FontRef>(vec![]).build();
        let (ww, hh) = cache.texture_dimensions();
        let size = (ww * hh) as usize;
        let image = TextureInfo::from_bytes_with_options(
            &vec![0; size],
            ww as _,
            hh as _,
            TextureFormat::Red,
            TextureFormat::R8,
            TextureFilter::Linear,
            TextureFilter::Linear,
        )?;

        let texture = device.create_texture(image)?;

        Ok(Self {
            cache,
            texture,
            render,
        })
    }

    pub fn load_font(&mut self, data: &'static [u8]) -> Result<Font, String> {
        let font = FontRef::try_from_slice(data).map_err(|e| e.to_string())?;

        Ok(Font {
            id: self.cache.add_font(font),
        })
    }

    pub fn process(&mut self, font: &Font, text: &Text) {
        self.cache.queue(section_from_text(font, text));
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        self.prepare();
        self.render.render(&mut self.texture, &[], renderer);
    }

    fn prepare(&mut self) {
        // todo: prepare vertices
    }
}
