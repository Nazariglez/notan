mod builder;
mod cache;
mod config;
mod extension;
mod instance;
mod pipeline;

use cache::Cache;
use instance::Instance;

pub use builder::GlyphBrushBuilder;
pub use config::GlyConfig;
pub use extension::{Glyph, GlyphExtension};
pub use glyph_brush::ab_glyph;
pub use glyph_brush::{
    BuiltInLineBreaker, Extra, FontId, GlyphCruncher, GlyphPositioner, HorizontalAlign, Layout,
    LineBreak, LineBreaker, Section, SectionGeometry, SectionGlyph, SectionGlyphIter, SectionText,
    Text, VerticalAlign,
};
pub use pipeline::DefaultGlyphPipeline;

use ab_glyph::{Font, FontArc, Rect};

use core::hash::BuildHasher;
use std::borrow::Cow;

use crate::pipeline::GlyphPipeline;
use glyph_brush::{BrushAction, BrushError, DefaultSectionHasher};
use log::{log_enabled, warn};
use notan_app::Graphics;
use notan_graphics::{Device, Renderer};
use notan_math::glam::Mat4;

/// Object allowing glyph drawing, containing cache state. Manages glyph positioning cacheing,
/// glyph draw caching & efficient GPU texture cache updating and re-sizing on demand.
///
/// Build using a [`GlyphBrushBuilder`](struct.GlyphBrushBuilder.html).
pub struct GlyphBrush<F = FontArc, H = DefaultSectionHasher> {
    cache: Cache,
    glyph_brush: glyph_brush::GlyphBrush<Instance, Extra, F, H>,
}

impl<F: Font, H: BuildHasher> GlyphBrush<F, H> {
    /// Queues a section/layout to be drawn by the next call of
    /// [`draw_queued`](struct.GlyphBrush.html#method.draw_queued). Can be
    /// called multiple times to queue multiple sections for drawing.
    ///
    /// Benefits from caching, see [caching behaviour](#caching-behaviour).
    #[inline]
    pub fn queue<'a, S>(&mut self, section: S)
    where
        S: Into<Cow<'a, Section<'a>>>,
    {
        self.glyph_brush.queue(section)
    }

    pub fn process<'a>(&mut self, glyphs: &Glyph<'a>) {
        glyphs.sections.iter().for_each(|s| {
            let n: &Section = s.as_ref();
            self.queue(n);
        });
    }

    /// Queues a section/layout to be drawn by the next call of
    /// [`draw_queued`](struct.GlyphBrush.html#method.draw_queued). Can be
    /// called multiple times to queue multiple sections for drawing.
    ///
    /// Used to provide custom `GlyphPositioner` logic, if using built-in
    /// [`Layout`](enum.Layout.html) simply use
    /// [`queue`](struct.GlyphBrush.html#method.queue)
    ///
    /// Benefits from caching, see [caching behaviour](#caching-behaviour).
    #[inline]
    pub fn queue_custom_layout<'a, S, G>(&mut self, section: S, custom_layout: &G)
    where
        G: GlyphPositioner,
        S: Into<Cow<'a, Section<'a>>>,
    {
        self.glyph_brush.queue_custom_layout(section, custom_layout)
    }

    /// Queues pre-positioned glyphs to be processed by the next call of
    /// [`draw_queued`](struct.GlyphBrush.html#method.draw_queued). Can be
    /// called multiple times.
    #[inline]
    pub fn queue_pre_positioned(
        &mut self,
        glyphs: Vec<SectionGlyph>,
        extra: Vec<Extra>,
        bounds: Rect,
    ) {
        self.glyph_brush.queue_pre_positioned(glyphs, extra, bounds)
    }

    /// Retains the section in the cache as if it had been used in the last
    /// draw-frame.
    ///
    /// Should not be necessary unless using multiple draws per frame with
    /// distinct transforms, see [caching behaviour](#caching-behaviour).
    #[inline]
    pub fn keep_cached_custom_layout<'a, S, G>(&mut self, section: S, custom_layout: &G)
    where
        S: Into<Cow<'a, Section<'a>>>,
        G: GlyphPositioner,
    {
        self.glyph_brush
            .keep_cached_custom_layout(section, custom_layout)
    }

    /// Retains the section in the cache as if it had been used in the last
    /// draw-frame.
    ///
    /// Should not be necessary unless using multiple draws per frame with
    /// distinct transforms, see [caching behaviour](#caching-behaviour).
    #[inline]
    pub fn keep_cached<'a, S>(&mut self, section: S)
    where
        S: Into<Cow<'a, Section<'a>>>,
    {
        self.glyph_brush.keep_cached(section)
    }

    /// Returns the available fonts.
    ///
    /// The `FontId` corresponds to the index of the font data.
    #[inline]
    pub fn fonts(&self) -> &[F] {
        self.glyph_brush.fonts()
    }

    /// Adds an additional font to the one(s) initially added on build.
    ///
    /// Returns a new [`FontId`](struct.FontId.html) to reference this font.
    pub fn add_font(&mut self, font: F) -> FontId {
        self.glyph_brush.add_font(font)
    }
}

impl<F: Font + Sync, H: BuildHasher> GlyphBrush<F, H> {
    pub fn create_renderer_from_queue<T: GlyphPipeline>(
        &mut self,
        device: &mut Device,
        pipeline: &mut T,
    ) -> Renderer {
        // TODO pattern builder to add size, scissor/region and clear options
        self.process_queued(device, pipeline);
        let (width, height) = device.size();
        let transform = Mat4::orthographic_lh(0.0, width as _, height as _, 0.0, -1.0, 1.0);
        return pipeline.create_renderer(
            device,
            self.cache.texture(),
            transform,
            width,
            height,
            None,
        );
    }

    fn process_queued<T: GlyphPipeline>(&mut self, device: &mut Device, pipeline: &mut T) {
        let mut brush_action;

        loop {
            brush_action = self.glyph_brush.process_queued(
                |rect, tex_data| {
                    let offset = [rect.min[0] as u16, rect.min[1] as u16];
                    let size = [rect.width() as u16, rect.height() as u16];

                    self.cache.update(device, offset, size, tex_data);
                },
                Instance::from_vertex,
            );

            match brush_action {
                Ok(_) => break,
                Err(BrushError::TextureTooSmall { suggested }) => {
                    let max_image_dimension = device.limits().max_texture_size;

                    let (new_width, new_height) = if (suggested.0 > max_image_dimension
                        || suggested.1 > max_image_dimension)
                        && (self.glyph_brush.texture_dimensions().0 < max_image_dimension
                            || self.glyph_brush.texture_dimensions().1 < max_image_dimension)
                    {
                        (max_image_dimension, max_image_dimension)
                    } else {
                        suggested
                    };

                    if log_enabled!(log::Level::Warn) {
                        warn!(
                            "Increasing glyph texture size {old:?} -> {new:?}. \
                             Consider building with `.initial_cache_size({new:?})` to avoid \
                             resizing",
                            old = self.glyph_brush.texture_dimensions(),
                            new = (new_width, new_height),
                        );
                    }

                    self.cache = Cache::new(device, new_width, new_height).unwrap();
                    self.glyph_brush.resize_texture(new_width, new_height);
                }
            }
        }

        match brush_action.unwrap() {
            BrushAction::Draw(verts) => {
                pipeline.upload(device, &verts);
            }
            BrushAction::ReDraw => {}
        };
    }
}

impl<F: Font, H: BuildHasher> GlyphBrush<F, H> {
    fn new(gfx: &mut Graphics, raw_builder: glyph_brush::GlyphBrushBuilder<F, H>) -> Self {
        let glyph_brush = raw_builder.build();
        let (cache_width, cache_height) = glyph_brush.texture_dimensions();
        let cache = Cache::new(gfx, cache_width as _, cache_height as _).unwrap();

        GlyphBrush { cache, glyph_brush }
    }
}

impl<F: Font, H: BuildHasher> GlyphCruncher<F> for GlyphBrush<F, H> {
    #[inline]
    fn glyphs_custom_layout<'a, 'b, S, L>(
        &'b mut self,
        section: S,
        custom_layout: &L,
    ) -> SectionGlyphIter<'b>
    where
        L: GlyphPositioner + std::hash::Hash,
        S: Into<Cow<'a, Section<'a>>>,
    {
        self.glyph_brush
            .glyphs_custom_layout(section, custom_layout)
    }

    #[inline]
    fn glyph_bounds_custom_layout<'a, S, L>(
        &mut self,
        section: S,
        custom_layout: &L,
    ) -> Option<Rect>
    where
        L: GlyphPositioner + std::hash::Hash,
        S: Into<Cow<'a, Section<'a>>>,
    {
        self.glyph_brush
            .glyph_bounds_custom_layout(section, custom_layout)
    }

    #[inline]
    fn fonts(&self) -> &[F] {
        self.glyph_brush.fonts()
    }
}

impl<F, H> std::fmt::Debug for GlyphBrush<F, H> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GlyphBrush")
    }
}
