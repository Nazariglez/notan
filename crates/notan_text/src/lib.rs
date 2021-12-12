mod config;

use notan_app::{ExtContainer, GfxExtension, GfxRenderer, Graphics};
use notan_gly::ab_glyph::FontArc;
use notan_gly::{
    ab_glyph, BuiltInLineBreaker, DefaultGlyphPipeline, FontId, GlyphBrush, GlyphBrushBuilder,
    GlyphPipeline, Layout, LineBreaker, Section, Text,
};
use notan_graphics::color::Color;
use notan_graphics::commands::Commands;
use notan_graphics::pipeline::ClearOptions;
use notan_graphics::{Device, RenderTexture, Renderer};
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::ptr::addr_of;

use crate::ab_glyph::PxScale;
pub use config::TextConfig;

pub struct Font {
    id: u64,
    inner: FontId,
}

pub struct TextExtension {
    glyph_brush: GlyphBrush,
    pipelines: HashMap<TypeId, Box<dyn GlyphPipeline>>,
}

impl TextExtension {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let glyph_brush = GlyphBrushBuilder::using_fonts(vec![]).build(gfx);
        let pipelines = HashMap::default();
        let mut ext = Self {
            glyph_brush,
            pipelines,
        };

        ext.add_pipeline(DefaultGlyphPipeline::new(gfx)?);
        Ok(ext)
    }

    pub fn create_font(&mut self, data: &[u8]) -> Result<Font, String> {
        let font = FontArc::try_from_vec(data.to_vec()).unwrap();
        let id = self.glyph_brush.add_font(font);
        Ok(Font {
            id: id.0 as _,
            inner: id,
        })
    }

    #[inline]
    pub fn add_pipeline<T>(&mut self, value: T)
    where
        T: GlyphPipeline + 'static,
    {
        self.pipelines.insert(TypeId::of::<T>(), Box::new(value));
    }

    #[inline]
    pub fn remove_pipeline<T>(&mut self)
    where
        T: GlyphPipeline + 'static,
    {
        self.pipelines.remove(&TypeId::of::<T>());
    }

    fn create_renderer(&mut self, device: &mut Device, text: &TT) -> Renderer {
        let mut glyph_brush = &mut self.glyph_brush;
        text.sections.iter().for_each(|s| glyph_brush.queue(s));
        glyph_brush.queue(&text.current_section);

        let mut pipeline = self
            .pipelines
            .get_mut(&TypeId::of::<DefaultGlyphPipeline>())
            .unwrap();
        glyph_brush
            .create_renderer(pipeline.deref_mut())
            .clear(ClearOptions::color(Color::BLACK))
            .process(device)
    }
}

impl GfxExtension<TT<'_>> for TextExtension {
    fn commands<'a>(&'a mut self, device: &mut Device, renderer: &'a TT) -> &'a [Commands] {
        &[]
    }
}

pub struct AddTextBuilder<'b, 'a: 'b> {
    section: &'b mut Section<'a>,
    text: Option<&'a str>,
    color: notan_graphics::color::Color,
    z: f32,
    size: f32,
}

impl<'b, 'a: 'b> AddTextBuilder<'b, 'a> {
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.section.screen_position = (x, y);
        self
    }

    pub fn color(mut self, color: notan_graphics::color::Color) -> Self {
        self.color = color;
        self
    }

    pub fn depth(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl Drop for AddTextBuilder<'_, '_> {
    fn drop(&mut self) {
        if let Some(text) = self.text.take() {
            if text.is_empty() {
                return;
            }

            self.section.text.push(
                Text::new(text)
                    .with_color(self.color.rgba())
                    .with_scale(self.size)
                    .with_z(self.z),
            );
        }
    }
}

pub struct TT<'a> {
    width: f32,
    height: f32,
    sections: Vec<Section<'a>>,
    current_section: Section<'a>,
    dirty: bool,
}

impl<'a> TT<'a> {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            sections: vec![],
            current_section: Default::default(),
            width,
            height,
            dirty: true,
        }
    }

    pub fn add_text<'b>(&'b mut self, text: &'a str) -> AddTextBuilder<'b, 'a>
    where
        'a: 'b,
    {
        self.sections.push(std::mem::replace(
            &mut self.current_section,
            Default::default(),
        ));
        AddTextBuilder {
            section: &mut self.current_section,
            text: Some(text),
            color: notan_graphics::color::Color::WHITE,
            z: 0.0,
            size: 16.0,
        }
    }
}

impl GfxRenderer for TT<'_> {
    fn render(
        &self,
        device: &mut Device,
        extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    ) {
        let mut ext = extensions.get_mut::<TT, TextExtension>().unwrap();
        let renderer = ext.create_renderer(device, self);
        match target {
            None => device.render(renderer.commands()),
            Some(rt) => device.render_to(rt, renderer.commands()),
        }
    }
}
