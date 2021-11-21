use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use glyph_brush::{Extra, Section};
use notan_app::{Commands, Device, ExtContainer, GfxExtension, GfxRenderer, Graphics, RenderTexture};
use crate::{GlyphBrush, GlyphBrushBuilder};

pub struct GlyphExtension {
    g: GlyphBrush
}

impl GlyphExtension {
    pub fn new(gfx: &mut Graphics) -> Self {
        Self {
            g: GlyphBrushBuilder::using_fonts(vec![]).build(gfx),
        }
    }
}

impl Deref for GlyphExtension {
    type Target = GlyphBrush;

    fn deref(&self) -> &Self::Target {
        &self.g
    }
}

impl DerefMut for GlyphExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.g
    }
}

impl GfxExtension<Glyph<'_>> for GlyphExtension {
    fn commands<'a>(&'a mut self, device: &mut Device, renderer: &'a Glyph) -> &'a [Commands] {
        &[] // todo
    }
}

#[derive(Default)]
pub struct Glyph<'a>
{
    pub(crate) sections: Vec<Cow<'a, Section<'a, Extra>>>,
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
    fn render(&self, device: &mut Device, extensions: &mut ExtContainer, target: Option<&RenderTexture>) {
        todo!()
    }
}