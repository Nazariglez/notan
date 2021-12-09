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
        // glyph_brush.process(glyphs);

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

impl GfxExtension<TT> for TextExtension {
    fn commands<'a>(&'a mut self, device: &mut Device, renderer: &'a TT) -> &'a [Commands] {
        &[]
    }
}

pub struct TT {
    width: f32,
    height: f32,
    sections: Vec<Section<'static>>,
    current_section: Section<'static>,
    text_added: bool,
}

impl TT {
    pub fn new(width: f32, height: f32) -> Self {
        let section = Section::default();

        Self {
            width,
            height,
            sections: vec![],
            current_section: section,
            text_added: false,
        }
    }

    pub fn position(&mut self, x: f32, y: f32) {
        let position = (x, y);
        let needs_update = self.current_section.screen_position != position;
        if needs_update {
            new_section_if_necessary(self);
            self.current_section.screen_position = position;
        }
    }

    pub fn bounds(&mut self, width: f32, height: f32) {
        let bounds = (width, height);
        let needs_update = self.current_section.bounds != bounds;
        if needs_update {
            new_section_if_necessary(self);
            self.current_section.bounds = bounds;
        }
    }

    pub fn layout(&mut self, layout: BuiltInLineBreaker) {
        // manage_dirty(self);
        // if let Some(section) = &mut self.current_section {
        // }
    }

    pub fn add_text<'a>(&mut self, text: &'a str) {
        self.text_added = true;
        self.current_section = self.current_section.add_text(Text::new(text));
    }
}

fn new_section_if_necessary(tt: &mut TT) {
    if tt.text_added {
        tt.sections.push(std::mem::take(&mut tt.current_section));
        tt.text_added = false;
    }
}

impl GfxRenderer for TT {
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
