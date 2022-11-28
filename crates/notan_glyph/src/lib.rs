mod builder;
mod cache;
mod instance;
mod pipeline;

use cache::Cache;
pub use instance::GlyphInstance;

pub use builder::GlyphBrushBuilder;
pub use glyph_brush::ab_glyph;
pub use glyph_brush::{
    BuiltInLineBreaker, Extra, FontId, GlyphCalculator, GlyphCalculatorBuilder, GlyphCruncher,
    GlyphPositioner, HorizontalAlign, Layout, LineBreak, LineBreaker, OwnedSection, OwnedText,
    Section, SectionGeometry, SectionGlyph, SectionGlyphIter, SectionText, Text, VerticalAlign,
};
pub use pipeline::{DefaultGlyphPipeline, GlyphPipeline};

use ab_glyph::{Font, FontArc, Rect};

use core::hash::BuildHasher;
use std::borrow::Cow;

use glyph_brush::{BrushAction, BrushError, DefaultSectionHasher};
use notan_app::Graphics;
use notan_graphics::prelude::ClearOptions;
use notan_graphics::{Device, Renderer, Texture};
use notan_math::Mat4;

/// Object allowing glyph drawing, containing cache state. Manages glyph positioning cacheing,
/// glyph draw caching & efficient GPU texture cache updating and re-sizing on demand.
///
/// Build using a [`GlyphBrushBuilder`](struct.GlyphBrushBuilder.html).
pub struct GlyphBrush<F = FontArc, H = DefaultSectionHasher> {
    cache: Cache,
    glyph_brush: glyph_brush::GlyphBrush<GlyphInstance, Extra, F, H>,
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

    /// Returns the texture used to cache the glyphs
    #[inline]
    pub fn texture(&self) -> &Texture {
        self.cache.texture()
    }
}

pub struct RenderQueueBuilder<'a, F = FontArc, H = DefaultSectionHasher> {
    device: &'a mut Device,
    glyph_brush: &'a mut GlyphBrush<F, H>,
    pipeline: &'a mut dyn GlyphPipeline,
    clear: Option<ClearOptions>,
    region: Option<notan_math::Rect>,
    size: Option<(i32, i32)>,
    transform: Option<Mat4>,
}

impl<'a, F: Font + Sync, H: BuildHasher> RenderQueueBuilder<'a, F, H> {
    fn new(
        glyph_brush: &'a mut GlyphBrush<F, H>,
        device: &'a mut Device,
        pipeline: &'a mut dyn GlyphPipeline,
    ) -> Self {
        Self {
            device,
            glyph_brush,
            pipeline,
            clear: None,
            region: None,
            size: None,
            transform: None,
        }
    }

    pub fn transform(mut self, projection: Mat4) -> Self {
        self.transform = Some(projection);
        self
    }

    pub fn clear(mut self, options: ClearOptions) -> Self {
        self.clear = Some(options);
        self
    }

    pub fn region(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.region = Some(notan_math::Rect {
            x,
            y,
            width,
            height,
        });
        self
    }

    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.size = Some((width, height));
        self
    }

    pub fn build(self) -> Renderer {
        let Self {
            device,
            glyph_brush,
            pipeline,
            clear,
            region,
            size,
            transform,
        } = self;

        let (width, height) = size.unwrap_or_else(|| device.size());
        let projection = transform.unwrap_or_else(|| {
            Mat4::orthographic_rh_gl(0.0, width as _, height as _, 0.0, -1.0, 1.0)
        });

        pipeline.create_renderer(
            device,
            glyph_brush.cache.texture(),
            clear,
            projection,
            (width, height),
            region,
        )
    }
}

impl<F: Font + Sync, H: BuildHasher> GlyphBrush<F, H> {
    pub fn render_queue<'a>(
        &'a mut self,
        device: &'a mut Device,
        pipeline: &'a mut dyn GlyphPipeline,
    ) -> RenderQueueBuilder<F, H> {
        RenderQueueBuilder::new(self, device, pipeline)
    }

    pub fn process_queued(&mut self, device: &mut Device, pipeline: &mut dyn GlyphPipeline) {
        debug_assert!(
            !self.fonts().is_empty(),
            "You need to add at least one Font to be used as default."
        );

        let mut brush_action;

        loop {
            brush_action = self.glyph_brush.process_queued(
                |rect, tex_data| {
                    let offset = [rect.min[0] as u16, rect.min[1] as u16];
                    let size = [rect.width() as u16, rect.height() as u16];

                    self.cache.update(device, offset, size, tex_data).unwrap();
                },
                GlyphInstance::from_vertex,
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

                    log::debug!(
                        "Increasing glyph texture size {old:?} -> {new:?}. \
                             Consider building with `.initial_cache_size({new:?})` to avoid \
                             resizing",
                        old = self.glyph_brush.texture_dimensions(),
                        new = (new_width, new_height),
                    );

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
