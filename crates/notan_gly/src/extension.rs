use crate::{DefaultGlyphPipeline, GlyphBrush, GlyphBrushBuilder, GlyphPipeline};
use glyph_brush::{ab_glyph, Extra, Section};
use notan_app::{
    Commands, Device, ExtContainer, GfxExtension, GfxRenderer, Graphics, RenderTexture,
};
use notan_graphics::prelude::ClearOptions;
use notan_graphics::Renderer;
use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

pub struct GlyphExtension {
    glyph_brush: GlyphBrush,
    pipelines: HashMap<TypeId, Box<dyn Any>>,
}

impl GlyphExtension {
    pub fn new(gfx: &mut Graphics) -> Self {
        let glyph_brush = GlyphBrushBuilder::using_fonts(vec![]).build(gfx);
        let pipelines = HashMap::default();
        let mut ext = Self {
            glyph_brush,
            pipelines,
        };

        ext.add_pipeline(DefaultGlyphPipeline::new(gfx).unwrap());
        ext
    }

    pub fn create_font(&mut self, data: &'static [u8]) {
        let font = ab_glyph::FontArc::try_from_slice(data).unwrap();
        self.glyph_brush.add_font(font);
    }

    #[inline]
    pub fn add_pipeline<T>(&mut self, value: T)
    where
        T: GlyphPipeline + 'static,
    {
        self.pipelines
            .insert(TypeId::of::<T>(), Box::new(RefCell::new(value)));
    }

    #[inline]
    pub fn remove_pipeline<T>(&mut self)
    where
        T: GlyphPipeline + 'static,
    {
        self.pipelines.remove(&TypeId::of::<T>());
    }

    fn create_renderer(&mut self, device: &mut Device, glyphs: &Glyph) -> Renderer {
        let mut glyph_brush = &mut self.glyph_brush;
        glyph_brush.process(glyphs);

        let mut pipeline = pipeline_mut::<DefaultGlyphPipeline>(&self.pipelines).unwrap();
        glyph_brush.create_renderer_from_queue(device, pipeline.deref_mut())
    }
}

#[inline]
fn pipeline_mut<T>(pipelines: &HashMap<TypeId, Box<dyn Any>>) -> Option<RefMut<'_, T>>
where
    T: GlyphPipeline + 'static,
{
    pipelines
        .get(&TypeId::of::<T>())?
        .downcast_ref::<RefCell<T>>()
        .map(|value| value.borrow_mut())
}

impl Deref for GlyphExtension {
    type Target = GlyphBrush;

    fn deref(&self) -> &Self::Target {
        &self.glyph_brush
    }
}

impl DerefMut for GlyphExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.glyph_brush
    }
}

impl GfxExtension<Glyph<'_>> for GlyphExtension {
    fn commands<'a>(&'a mut self, device: &mut Device, renderer: &'a Glyph) -> &'a [Commands] {
        &[]
    }
}

#[derive(Default)]
pub struct Glyph<'a> {
    pub(crate) sections: Vec<Cow<'a, Section<'a, Extra>>>,
    pipeline: Option<TypeId>,
    clear: Option<ClearOptions>,
}

impl<'a> Glyph<'a> {
    pub fn queue<S>(&mut self, section: S)
    where
        S: Into<Cow<'a, Section<'a, Extra>>>,
    {
        self.sections.push(section.into());
    }
}

impl GfxRenderer for Glyph<'_> {
    fn render(
        &self,
        device: &mut Device,
        extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    ) {
        let mut ext = extensions.get_mut::<Self, GlyphExtension>().unwrap();
        let renderer = ext.create_renderer(device, self);
        match target {
            None => device.render(renderer.commands()),
            Some(rt) => device.render_to(rt, renderer.commands()),
        }
    }
}
