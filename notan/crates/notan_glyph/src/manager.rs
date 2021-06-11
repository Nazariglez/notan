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
    pub vertices: Vec<[f32; 13]>,
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
        let texture = create_texture(device, ww, hh)?;
        let vertices = vec![];

        Ok(Self {
            cache,
            texture,
            render,
            vertices,
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
        self.render
            .render(&mut self.texture, &self.vertices, renderer);
    }

    pub fn update_texture(&mut self, device: &mut Device) -> Result<(), String> {
        let action = loop {
            let mut result: Result<(), String> = Ok(());

            let max_texture_size = device.limits().max_texture_size;
            let mut texture = &mut self.texture;

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
                            bytes: &data,
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

        if let BrushAction::Draw(data) = action {
            self.vertices = data;
        }

        Ok(())
    }
}

#[inline]
pub fn to_vertex(
    GlyphVertex {
        mut tex_coords,
        pixel_coords,
        bounds,
        extra,
    }: GlyphVertex,
) -> [f32; 13] {
    let gl_bounds = bounds;

    let mut pixel_coords = Rect {
        min: point(pixel_coords.min.x as f32, pixel_coords.min.y as f32),
        max: point(pixel_coords.max.x as f32, pixel_coords.max.y as f32),
    };

    let x = pixel_coords.min.x;
    let y = pixel_coords.min.y;
    let z = extra.z;
    let width = pixel_coords.max.x - x;
    let height = pixel_coords.max.y - y;
    let [r, g, b, a] = extra.color;

    #[rustfmt::skip]
   let vertices = [
        x, y, z,
        width, height,
        tex_coords.min.x,
        tex_coords.min.y,
        tex_coords.max.x,
        tex_coords.max.y,
        r, g, b, a,
    ];

    vertices
}

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
