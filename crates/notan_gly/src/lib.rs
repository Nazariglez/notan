mod builder;
mod cache;
mod instance;
mod pipeline;

use cache::Cache;
use instance::Instance;

pub use builder::GlyphBrushBuilder;
pub use glyph_brush::ab_glyph;
pub use glyph_brush::{
    BuiltInLineBreaker, Extra, FontId, GlyphCruncher, GlyphPositioner, HorizontalAlign, Layout,
    LineBreak, LineBreaker, Section, SectionGeometry, SectionGlyph, SectionGlyphIter, SectionText,
    Text, VerticalAlign,
};
pub use pipeline::BasicGlyphPipeline;

use ab_glyph::{Font, FontArc, Rect};

use core::hash::BuildHasher;
use std::borrow::Cow;

use glyph_brush::{BrushAction, BrushError, DefaultSectionHasher};
use log::{log_enabled, warn};
use notan_app::Graphics;
use notan_graphics::Renderer;
use notan_math::glam::Mat4;

#[derive(Default)]
pub struct Glyph<'a>
{
    sections: Vec<Cow<'a, Section<'a, Extra>>>,
}

impl<'a> Glyph<'a> {
    pub fn queue<S>(&mut self, section: S)
        where
            S: Into<Cow<'a, Section<'a, Extra>>>,
    {
        self.sections.push(section.into());
    }
}

/// Object allowing glyph drawing, containing cache state. Manages glyph positioning cacheing,
/// glyph draw caching & efficient GPU texture cache updating and re-sizing on demand.
///
/// Build using a [`GlyphBrushBuilder`](struct.GlyphBrushBuilder.html).
pub struct GlyphBrush<F = FontArc, H = DefaultSectionHasher> {
    // pipeline: GlyPipeline,
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
            let n : &Section = s.as_ref();
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
    // /// Draws all queued sections onto a render target.
    // /// See [`queue`](struct.GlyphBrush.html#method.queue).
    // ///
    // /// Trims the cache, see [caching behaviour](#caching-behaviour).
    // ///
    // /// # Panics
    // /// Panics if the provided `target` has a texture format that does not match
    // /// the `render_format` provided on creation of the `GlyphBrush`.
    // #[inline]
    // pub fn draw_queued(
    //     &mut self,
    //     gfx: &mut Graphics,
    //     target_width: u32,
    //     target_height: u32,
    // ) -> Result<(), String> {
    //     self.draw_queued_with_transform(
    //         gfx,
    //         Mat4::orthographic_lh(0.0, target_width as _, target_height as _, 0.0, -1.0, 1.0),
    //     )
    // }
    //
    // /// Draws all queued sections onto a render target, applying a position
    // /// transform (e.g. a projection).
    // /// See [`queue`](struct.GlyphBrush.html#method.queue).
    // ///
    // /// Trims the cache, see [caching behaviour](#caching-behaviour).
    // ///
    // /// # Panics
    // /// Panics if the provided `target` has a texture format that does not match
    // /// the `render_format` provided on creation of the `GlyphBrush`.
    // #[inline]
    // pub fn draw_queued_with_transform(
    //     &mut self,
    //     gfx: &mut Graphics,
    //     transform: Mat4,
    // ) -> Result<(), String> {
    //     self.process_queued(gfx);
    //     self.pipeline
    //         .draw(gfx, &self.cache.texture(), transform, None);
    //
    //     Ok(())
    // }
    //
    // #[inline]
    // pub fn draw_queued_with_transform_and_scissoring(
    //     &mut self,
    //     gfx: &mut Graphics,
    //     transform: Mat4,
    //     rect: notan_math::Rect,
    // ) -> Result<(), String> {
    //     self.process_queued(gfx);
    //     self.pipeline
    //         .draw(gfx, &self.cache.texture(), transform, Some(rect));
    //
    //     Ok(())
    // }

    pub fn renderer_queue(
        &mut self,
        gfx: &mut Graphics,
        pipeline: &mut BasicGlyphPipeline,
    ) -> Renderer {
        self.process_queued(gfx, pipeline);
        let (width, height) = gfx.size();
        return pipeline.process_renderer(gfx, self.cache.texture(), width, height, None);
    }

    pub fn draw_queued(
        &mut self,
        gfx: &mut Graphics,
        pipeline: &mut BasicGlyphPipeline,
        target_width: u32,
        target_height: u32,
    ) {
        self.process_queued(gfx, pipeline);
        pipeline.draw(
            gfx,
            &self.cache.texture(),
            Mat4::orthographic_lh(0.0, target_width as _, target_height as _, 0.0, -1.0, 1.0),
            None,
        );
    }

    fn process_queued(&mut self, gfx: &mut Graphics, pipeline: &mut BasicGlyphPipeline) {
        // let pipeline = &mut self.pipeline;

        let mut brush_action;

        loop {
            brush_action = self.glyph_brush.process_queued(
                |rect, tex_data| {
                    let offset = [rect.min[0] as u16, rect.min[1] as u16];
                    let size = [rect.width() as u16, rect.height() as u16];

                    self.cache.update(gfx, offset, size, tex_data);
                },
                Instance::from_vertex,
            );

            match brush_action {
                Ok(_) => break,
                Err(BrushError::TextureTooSmall { suggested }) => {
                    let max_image_dimension = gfx.limits().max_texture_size;

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

                    self.cache = Cache::new(gfx, new_width, new_height).unwrap();
                    self.glyph_brush.resize_texture(new_width, new_height);
                }
            }
        }

        match brush_action.unwrap() {
            BrushAction::Draw(verts) => {
                pipeline.upload(gfx, &verts);
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

        GlyphBrush {
            // pipeline: GlyPipeline::new(gfx).unwrap(),
            cache,
            glyph_brush,
        }
    }
}

/// Helper function to generate a generate a transform matrix.
pub fn orthographic_projection(width: u32, height: u32) -> [f32; 16] {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    [
        2.0 / width as f32, 0.0, 0.0, 0.0,
        0.0, -2.0 / height as f32, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -1.0, 1.0, 0.0, 1.0,
    ]
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
