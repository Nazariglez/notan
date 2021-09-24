use crate::font::Font;
use crate::font_vertex::*;
use crate::pipeline::GlyphPipeline;
use crate::text::{section_from_text, Text};
use glyph_brush::{ab_glyph::*, *};
use notan_app::graphics::*;
use std::sync::Arc;

/// The FontManager take care of process the text and prepare them to renderer it on an inner texture
#[derive(Debug)]
pub struct GlyphPlugin {
    /// Glyph Brush cache
    pub cache: GlyphBrush<FontVertex, Extra, FontRef<'static>>,

    /// Texture that contains the glyphs
    pub texture: Texture,
}

impl GlyphPlugin {
    /// Creates a new manager using a custom Render
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let cache = GlyphBrushBuilder::using_fonts::<FontRef>(vec![]).build();
        let (ww, hh) = cache.texture_dimensions();
        let texture = create_texture(device, ww, hh)?;

        Ok(Self { cache, texture })
    }

    /// Loads a fonts into the manager and returns the Font object to be used on Text objects
    pub fn create_font(&mut self, data: &'static [u8]) -> Result<Font, String> {
        let font = FontRef::try_from_slice(data).map_err(|e| e.to_string())?;
        let glyphs = GlyphCalculatorBuilder::using_font(font.clone()).build();

        Ok(Font {
            id: self.cache.add_font(font),
            glyphs: Arc::new(glyphs),
        })
    }

    /// Add a Text object to the process queue to be prepared to render
    pub fn process_text(&mut self, font: &Font, text: &Text) {
        self.cache.queue(section_from_text(font, text));
    }

    /// Process and prepare the inner texture with the glyph necessary to render the text
    pub fn update(
        &mut self,
        device: &mut Device,
        render: &mut dyn GlyphPipeline,
    ) -> Result<(), String> {
        let action = loop {
            let mut result: Result<(), String> = Ok(());

            let max_texture_size = device.limits().max_texture_size;
            let texture = &mut self.texture;

            let brush_action = self.cache.process_queued(
                |rect, data| {
                    let x_offset = rect.min[0] as _;
                    let y_offset = rect.min[1] as _;
                    let width = (rect.max[0] - rect.min[0]) as _;
                    let height = (rect.max[1] - rect.min[1]) as _;

                    result = device.update_texture(
                        texture,
                        &TextureUpdate {
                            x_offset,
                            y_offset,
                            width,
                            height,
                            format: TextureFormat::Red,
                            bytes: data,
                        },
                    );
                },
                to_vertex,
            );

            if let Err(e) = result {
                return Err(e);
            }

            match brush_action {
                Ok(action) => break action,
                Err(BrushError::TextureTooSmall { suggested, .. }) => {
                    let cache_tex_dimensions = self.cache.texture_dimensions();

                    let (new_width, new_height) = if (suggested.0 > max_texture_size
                        || suggested.1 > max_texture_size)
                        && (cache_tex_dimensions.0 < max_texture_size
                            || cache_tex_dimensions.1 < max_texture_size)
                    {
                        (max_texture_size, max_texture_size)
                    } else {
                        suggested
                    };

                    *texture = create_texture(device, new_width, new_height)?;
                    self.cache.resize_texture(new_width, new_height);
                }
            }
        };

        match action {
            BrushAction::Draw(data) => render.update(device, Some(&data)),
            _ => render.update(device, None),
        };

        Ok(())
    }
}

impl notan_app::Plugin for GlyphPlugin {}

fn create_texture(device: &mut Device, ww: u32, hh: u32) -> Result<Texture, String> {
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

    device.create_texture(image)
}
