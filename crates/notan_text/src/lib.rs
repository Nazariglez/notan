mod config;

use notan_app::{ExtContainer, GfxExtension, GfxRenderer, Graphics};
use notan_gly::ab_glyph::FontArc;
use notan_gly::{
    ab_glyph, BuiltInLineBreaker, DefaultGlyphPipeline, FontId, GlyphBrush, GlyphBrushBuilder,
    GlyphPipeline, HorizontalAlign, Layout, LineBreaker, Section, Text as GText, VerticalAlign,
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
use notan_math::glam::Mat4;
use notan_math::Rect;

pub struct Font {
    id: u64,
    inner: FontId,
}

pub struct TextExtension {
    glyph_brush: GlyphBrush,
    pipelines: HashMap<TypeId, Box<dyn GlyphPipeline>>,
    font_loaded: bool,
}

impl TextExtension {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let glyph_brush = GlyphBrushBuilder::using_fonts(vec![]).build(gfx);
        let pipelines = HashMap::default();
        let mut ext = Self {
            glyph_brush,
            pipelines,
            font_loaded: false,
        };

        ext.add_pipeline(DefaultGlyphPipeline::new(gfx)?);
        Ok(ext)
    }

    pub fn create_font(&mut self, data: &[u8]) -> Result<Font, String> {
        let font = FontArc::try_from_vec(data.to_vec()).map_err(|err| err.to_string())?;
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

    fn create_renderer(&mut self, device: &mut Device, text: &Text) -> Renderer {
        let mut glyph_brush = &mut self.glyph_brush;
        text.sections.iter().for_each(|s| glyph_brush.queue(s));
        glyph_brush.queue(&text.current_section);

        let pipeline_type = text
            .pipeline_type
            .unwrap_or_else(|| TypeId::of::<DefaultGlyphPipeline>());

        let clear_options = text.clear_options.unwrap_or_else(|| ClearOptions::none());
        let mut pipeline = self.pipelines.get_mut(&pipeline_type).unwrap();

        let mut builder = glyph_brush
            .create_renderer(pipeline.deref_mut())
            .clear(clear_options)
            .size(text.width, text.height);

        if let Some(transform) = text.transform {
            builder = builder.transform(transform);
        }

        if let Some(region) = text.region {
            builder = builder.region(region.x, region.y, region.width, region.height);
        }

        builder.process(device)
    }
}

impl GfxExtension<Text<'_>> for TextExtension {
    fn commands<'a>(&'a mut self, device: &mut Device, renderer: &'a Text) -> &'a [Commands] {
        &[]
    }
}

pub struct AddTextBuilder<'b, 'a: 'b> {
    text: &'b mut Text<'a>,
    text_str: Option<&'a str>,
    section: Option<Section<'a>>,
    color: Color,
    z: f32,
    size: f32,
    font: FontId,
    h_align: HorizontalAlign,
    v_align: VerticalAlign,
}

impl<'b, 'a: 'b> AddTextBuilder<'b, 'a> {
    pub fn font(mut self, font: &Font) -> Self {
        self.font = font.inner;
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        if let Some(s) = &mut self.section {
            s.screen_position = (x, y);
        }
        self
    }

    pub fn max_width(mut self, width: f32) -> Self {
        if let Some(s) = &mut self.section {
            s.bounds.0 = width;
        }
        self
    }

    pub fn h_align_left(mut self) -> Self {
        self.h_align = HorizontalAlign::Left;
        self
    }

    pub fn h_align_center(mut self) -> Self {
        self.h_align = HorizontalAlign::Center;
        self
    }

    pub fn h_align_right(mut self) -> Self {
        self.h_align = HorizontalAlign::Right;
        self
    }

    pub fn v_align_top(mut self) -> Self {
        self.v_align = VerticalAlign::Top;
        self
    }

    pub fn v_align_middle(mut self) -> Self {
        self.v_align = VerticalAlign::Center;
        self
    }

    pub fn v_align_bottom(mut self) -> Self {
        self.v_align = VerticalAlign::Bottom;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
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
        if let (Some(text), Some(mut section)) = (self.text_str.take(), self.section.take()) {
            if !text.is_empty() {
                section.text.push(
                    GText::new(text)
                        .with_color(self.color.rgba())
                        .with_scale(self.size)
                        .with_z(self.z)
                        .with_font_id(self.font),
                );
            }

            section.layout = Layout::default()
                .h_align(self.h_align)
                .v_align(self.v_align);

            self.text.sections.push(section);
        }
    }
}

pub struct ChainTextBuilder<'b, 'a: 'b> {
    section: &'b mut Section<'a>,
    text: Option<&'a str>,
    color: Color,
    z: f32,
    size: f32,
    font: FontId,
}

impl<'b, 'a: 'b> ChainTextBuilder<'b, 'a> {
    pub fn font(mut self, font: &Font) -> Self {
        self.font = font.inner;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
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

impl Drop for ChainTextBuilder<'_, '_> {
    fn drop(&mut self) {
        if let Some(text) = self.text.take() {
            if !text.is_empty() {
                self.section.text.push(
                    GText::new(text)
                        .with_color(self.color.rgba())
                        .with_scale(self.size)
                        .with_z(self.z)
                        .with_font_id(self.font),
                );
            }
        }
    }
}

pub struct Text<'a> {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) sections: Vec<Section<'a>>,
    pub(crate) current_section: Section<'a>,
    pub(crate) pipeline_type: Option<TypeId>,
    pub(crate) clear_options: Option<ClearOptions>,
    pub(crate) transform: Option<Mat4>,
    pub(crate) region: Option<Rect>,
}

impl<'a> Text<'a> {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            sections: vec![],
            current_section: Default::default(),
            width,
            height,
            pipeline_type: None,
            clear_options: None,
            transform: None,
            region: None,
        }
    }

    pub fn clear_options(&mut self, options: ClearOptions) {
        self.clear_options = Some(options);
    }

    pub fn clear_color(&mut self, color: Color) {
        if self.clear_options.is_none() {
            self.clear_options = Some(ClearOptions::color(color));
            return;
        }

        if let Some(opts) = &mut self.clear_options {
            opts.color = Some(color);
        }
    }

    pub fn use_pipeline<T: GlyphPipeline + 'static>(&mut self) {
        self.pipeline_type = Some(TypeId::of::<T>());
    }

    pub fn transform(&mut self, transform: Mat4) {
        self.transform = Some(transform);
    }

    pub fn region(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.region = Some(Rect {
            x,
            y,
            width,
            height,
        });
    }

    pub fn add<'b>(&'b mut self, text: &'a str) -> AddTextBuilder<'b, 'a>
    where
        'a: 'b,
    {
        AddTextBuilder {
            text: self,
            section: Some(Default::default()),
            text_str: Some(text),
            color: Color::WHITE,
            z: 0.0,
            size: 16.0,
            font: Default::default(),
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
        }
    }

    pub fn chain<'b>(&'b mut self, text: &'a str) -> ChainTextBuilder<'b, 'a>
    where
        'a: 'b,
    {
        if self.sections.is_empty() {
            self.sections.push(Default::default());
        }

        let section = self.sections.last_mut().unwrap();

        ChainTextBuilder {
            section,
            text: Some(text),
            color: Color::WHITE,
            z: 0.0,
            size: 16.0,
            font: Default::default(),
        }
    }
}

impl GfxRenderer for Text<'_> {
    fn render(
        &self,
        device: &mut Device,
        extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    ) {
        let mut ext = extensions.get_mut::<Text, TextExtension>().unwrap();
        let renderer = ext.create_renderer(device, self);
        match target {
            None => device.render(renderer.commands()),
            Some(rt) => device.render_to(rt, renderer.commands()),
        }
    }
}

pub trait CreateText {
    fn create_text<'a>(&self) -> Text<'a>;
    fn create_font(&mut self, data: &[u8]) -> Result<Font, String>;
}

impl CreateText for Graphics {
    fn create_text<'a>(&self) -> Text<'a> {
        let (width, height) = self.device.size();
        Text::new(width, height)
    }

    fn create_font(&mut self, data: &[u8]) -> Result<Font, String> {
        self.extension_mut::<Text<'_>, TextExtension>()
            .ok_or_else(|| "The TextExtension is not in use".to_string())?
            .create_font(data)
    }
}
