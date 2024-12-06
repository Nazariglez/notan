use crate::batch::*;
pub(crate) use crate::custom_pipeline::CustomPipeline;
use crate::transform::Transform;
use crate::{local_to_screen_position, screen_to_local_position};
use notan_glyph::Section;
use notan_graphics::color::Color;
use notan_graphics::prelude::*;
use notan_math::{vec2, Mat3, Mat4, Rect, Vec2};
use notan_text::{Calculator, Font};

#[derive(Debug)]
pub struct Draw {
    pub(crate) clear_color: Option<Color>,
    alpha: f32,
    transform: Transform,
    base_projection: Mat4,
    projection: Option<Mat4>,
    pub(crate) inverse_projection: Option<Mat4>,
    size: (f32, f32),
    blend_mode: Option<BlendMode>,
    alpha_mode: Option<BlendMode>,
    pub(crate) batches: Vec<Batch>,
    pub(crate) current_batch: Option<Batch>,
    pub(crate) shape_pipeline: CustomPipeline,
    pub(crate) image_pipeline: CustomPipeline,
    pub(crate) pattern_pipeline: CustomPipeline,
    pub(crate) text_pipeline: CustomPipeline,
    pub(crate) text_batch_indices: Option<Vec<usize>>,
    pub(crate) masking: bool,
    pub(crate) needs_to_clean_stencil: bool,
    pub(crate) glyphs_calculator: Calculator,
    mask_batches: Option<Vec<Batch>>,
}

impl Clone for Draw {
    fn clone(&self) -> Self {
        Self {
            alpha: self.alpha,
            clear_color: self.clear_color,
            batches: self.batches.clone(),
            current_batch: self.current_batch.clone(),
            transform: self.transform.clone(),
            base_projection: self.base_projection,
            projection: self.projection,
            inverse_projection: self.inverse_projection,
            size: self.size,
            blend_mode: self.blend_mode,
            alpha_mode: self.alpha_mode,
            shape_pipeline: self.shape_pipeline.clone(),
            image_pipeline: self.image_pipeline.clone(),
            pattern_pipeline: self.pattern_pipeline.clone(),
            text_pipeline: self.text_pipeline.clone(),
            masking: self.masking,
            needs_to_clean_stencil: self.needs_to_clean_stencil,
            text_batch_indices: self.text_batch_indices.clone(),
            glyphs_calculator: Calculator::new(),
            mask_batches: self.mask_batches.clone(),
        }
    }
}

impl Draw {
    pub fn new(width: u32, height: u32) -> Self {
        let base_projection =
            Mat4::orthographic_rh_gl(0.0, width as _, height as _, 0.0, -1.0, 1.0);

        Draw {
            alpha: 1.0,
            clear_color: None,
            batches: vec![],
            current_batch: None,
            transform: Transform::new(),
            base_projection,
            projection: None,
            inverse_projection: None,
            size: (width as _, height as _),
            blend_mode: Some(BlendMode::NORMAL),
            alpha_mode: None,
            shape_pipeline: Default::default(),
            image_pipeline: Default::default(),
            pattern_pipeline: Default::default(),
            text_pipeline: Default::default(),
            masking: false,
            needs_to_clean_stencil: false,
            text_batch_indices: None,
            glyphs_calculator: Calculator::new(),
            mask_batches: None,
        }
    }

    /// Returns the global matrix
    #[inline]
    pub fn matrix(&self) -> &Mat3 {
        self.transform.matrix()
    }

    fn process_mask_batches(&mut self) {
        if let Some(mask_batches) = self.mask_batches.take() {
            //Move the current batch to the queue
            if let Some(b) = self.current_batch.take() {
                self.batches.push(b);
            }

            self.batches.extend(mask_batches.iter().map(|batch| {
                let mut b = batch.clone();
                b.is_mask = true;
                b
            }));
        }
    }

    pub fn mask(&mut self, mask: Option<&Self>) {
        debug_assert!(!(self.masking && mask.is_some()), "Already using mask.");

        match mask {
            Some(m) if !self.masking => {
                let mut mask_batches = m.batches.clone();
                if let Some(b) = m.current_batch.as_ref() {
                    mask_batches.push(b.clone());
                }

                if !mask_batches.is_empty() {
                    self.masking = true;
                    self.mask_batches = Some(mask_batches);
                }
            }
            None if self.masking => {
                self.masking = false;
                self.mask_batches = None;

                //Move the current batch to the queue
                if let Some(b) = self.current_batch.take() {
                    self.batches.push(b);
                }
            }
            _ => {
                #[cfg(debug_assertions)]
                {
                    log::warn!(
                        "Draw setting mask as: {:?} when the value is {:?} is a no-op",
                        mask.is_some(),
                        self.masking
                    );
                }
            }
        }
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.size = (width, height);
        self.base_projection = Mat4::orthographic_rh_gl(0.0, width, height, 0.0, -1.0, 1.0);
    }

    pub fn size(&self) -> (f32, f32) {
        self.size
    }

    pub fn width(&self) -> f32 {
        self.size.0
    }

    pub fn height(&self) -> f32 {
        self.size.1
    }

    pub fn set_projection(&mut self, matrix: Option<Mat4>) {
        self.projection = matrix;
        self.inverse_projection = None;
    }

    pub fn projection(&self) -> Mat4 {
        self.projection.unwrap_or(self.base_projection)
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
    }

    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    pub fn blend_mode(&self) -> Option<BlendMode> {
        self.blend_mode
    }

    pub fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.blend_mode = mode;
    }

    pub fn alpha_mode(&self) -> Option<BlendMode> {
        self.alpha_mode
    }

    pub fn set_alpha_mode(&mut self, mode: Option<BlendMode>) {
        self.alpha_mode = mode;
    }

    pub fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn clear(&mut self, color: Color) {
        self.clear_color = Some(color);
    }

    fn add_batch<I, F1, F2>(&mut self, info: &I, is_diff_type: F1, create_type: F2)
    where
        I: DrawInfo,
        F1: Fn(&Batch, &I) -> bool,
        F2: Fn(&I) -> BatchType,
    {
        if self.masking {
            self.needs_to_clean_stencil = true;
            self.process_mask_batches();
        }

        let needs_new_batch = needs_new_batch(self, info, is_diff_type);
        if needs_new_batch {
            if let Some(old) = self.current_batch.take() {
                self.batches.push(old);
            }

            let typ = create_type(info);
            let custom = match typ {
                BatchType::Image { .. } => &self.image_pipeline,
                BatchType::Pattern { .. } => &self.pattern_pipeline,
                BatchType::Shape => &self.shape_pipeline,
                BatchType::Text { .. } => &self.text_pipeline,
            };

            // blending modes, by priority:
            // 1. element draw
            // 2. global draw blending
            // 3. in some cases (like text), default mode
            let cbm = info.blend_mode().or(self.blend_mode);
            let abm = info.alpha_mode().or(self.alpha_mode).or(match typ {
                // text is drawn from a RT we need to set Over alpha by default
                BatchType::Text { .. } => Some(BlendMode::OVER),
                _ => None,
            });

            self.current_batch = Some(Batch {
                typ: create_type(info),
                vertices: vec![],
                indices: vec![],
                pipeline: custom.pipeline.clone(),
                uniform_buffers: custom.uniforms.clone(),
                blend_mode: cbm,
                alpha_mode: abm,
                is_mask: false,
                masking: self.masking,
            });
        }

        let global_matrix = *self.transform.matrix();
        let matrix = match *info.transform() {
            Some(m) => global_matrix * *m,
            _ => global_matrix,
        };

        if let Some(b) = &mut self.current_batch {
            b.add(info.indices(), info.vertices(), matrix, self.alpha);
        }
    }

    pub fn add_image(&mut self, info: &ImageInfo) {
        let is_diff_type = |b: &Batch, i: &ImageInfo| {
            match &b.typ {
                //different texture
                BatchType::Image { texture } => texture != i.texture,

                //different batch type
                _ => true,
            }
        };

        let create_type = |i: &ImageInfo| BatchType::Image {
            texture: i.texture.clone(),
        };

        self.add_batch(info, is_diff_type, create_type);
    }

    pub fn add_shape(&mut self, info: &ShapeInfo) {
        let is_diff_type = |b: &Batch, _: &ShapeInfo| !b.is_shape();
        let create_type = |_: &ShapeInfo| BatchType::Shape;

        self.add_batch(info, is_diff_type, create_type);
    }

    pub fn add_pattern(&mut self, info: &ImageInfo) {
        let is_diff_type = |b: &Batch, i: &ImageInfo| {
            match &b.typ {
                //different texture
                BatchType::Pattern { texture } => texture != i.texture,

                //different batch type
                _ => true,
            }
        };

        let create_type = |i: &ImageInfo| BatchType::Pattern {
            texture: i.texture.clone(),
        };

        self.add_batch(info, is_diff_type, create_type);
    }

    pub fn add_text(&mut self, info: &TextInfo) {
        let is_diff_type = |b: &Batch, _: &TextInfo| !b.is_text();
        let create_type = |_: &TextInfo| BatchType::Text { texts: vec![] };

        self.add_batch(info, is_diff_type, create_type);

        if let Some(b) = &mut self.current_batch {
            // vertices and indices are calculated before the flush to the gpu,
            // so we need to store the text until that time
            if let BatchType::Text { texts } = &mut b.typ {
                let global_matrix = *self.transform.matrix();
                let matrix = match *info.transform() {
                    Some(m) => global_matrix * *m,
                    _ => global_matrix,
                };

                texts.push(TextData {
                    section: info.section.to_owned(),
                    transform: matrix,
                    alpha: self.alpha,
                    count: info.count,
                    flip: info.flip,
                });
            }
        }

        let batch_len = self.batches.len();
        let indices = self.text_batch_indices.get_or_insert(vec![]);
        indices.push(batch_len);
    }

    /// Get the bounds of the last text immediately after draw it
    /// The bounds doesn't take in account the Transformation matrix
    pub fn last_text_bounds(&mut self) -> Rect {
        if let Some(batch) = &self.current_batch {
            if let BatchType::Text { texts } = &batch.typ {
                if let Some(text) = texts.last() {
                    return self.glyphs_calculator.bounds(&text.section.to_borrowed());
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            log::debug!(
                "'draw.last_text_bounds()' must be called immediately after 'draw.text(..)"
            );
        }

        Rect::default()
    }

    pub fn screen_to_world_position(&mut self, screen_x: f32, screen_y: f32) -> Vec2 {
        let inverse = *self
            .inverse_projection
            .get_or_insert(self.projection().inverse());

        let view = *self.transform().matrix();
        screen_to_local_position(vec2(screen_x, screen_y), self.size.into(), inverse, view)
    }

    pub fn world_to_screen_position(&mut self, world_x: f32, world_y: f32) -> Vec2 {
        let projection = self.projection();
        let view = *self.transform().matrix();
        local_to_screen_position(vec2(world_x, world_y), self.size.into(), projection, view)
    }
}

trait DrawInfo {
    fn transform(&self) -> &Option<&Mat3>;
    fn vertices(&self) -> &[f32];
    fn indices(&self) -> &[u32];
    fn blend_mode(&self) -> Option<BlendMode>;
    fn alpha_mode(&self) -> Option<BlendMode>;
}

/// Information to render the image or pattern
pub struct ImageInfo<'a> {
    pub texture: &'a Texture,
    pub transform: Option<&'a Mat3>,
    pub vertices: &'a [f32],
    pub indices: &'a [u32],
    pub blend_mode: Option<BlendMode>,
    pub alpha_mode: Option<BlendMode>,
}

impl DrawInfo for ImageInfo<'_> {
    fn transform(&self) -> &Option<&Mat3> {
        &self.transform
    }

    fn vertices(&self) -> &[f32] {
        self.vertices
    }

    fn indices(&self) -> &[u32] {
        self.indices
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.blend_mode
    }

    fn alpha_mode(&self) -> Option<BlendMode> {
        self.alpha_mode
    }
}

/// Information to render the shape
pub struct ShapeInfo<'a> {
    pub transform: Option<&'a Mat3>,
    pub vertices: &'a [f32],
    pub indices: &'a [u32],
    pub blend_mode: Option<BlendMode>,
    pub alpha_mode: Option<BlendMode>,
}

impl DrawInfo for ShapeInfo<'_> {
    fn transform(&self) -> &Option<&Mat3> {
        &self.transform
    }

    fn vertices(&self) -> &[f32] {
        self.vertices
    }

    fn indices(&self) -> &[u32] {
        self.indices
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.blend_mode
    }

    fn alpha_mode(&self) -> Option<BlendMode> {
        self.alpha_mode
    }
}

pub struct TextInfo<'a> {
    pub count: usize,
    pub transform: Option<&'a Mat3>,
    pub section: &'a Section<'a>,
    pub font: &'a Font,
    pub blend_mode: Option<BlendMode>,
    pub alpha_mode: Option<BlendMode>,
    pub flip: (bool, bool),
}

impl DrawInfo for TextInfo<'_> {
    fn transform(&self) -> &Option<&Mat3> {
        &self.transform
    }

    fn vertices(&self) -> &[f32] {
        &[]
    }

    fn indices(&self) -> &[u32] {
        &[]
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.blend_mode
    }

    fn alpha_mode(&self) -> Option<BlendMode> {
        self.alpha_mode
    }
}

fn needs_new_batch<I: DrawInfo, F: Fn(&Batch, &I) -> bool>(
    draw: &Draw,
    info: &I,
    is_diff_type: F,
) -> bool {
    match &draw.current_batch {
        None => true, // no previous batch, so we need a new one
        Some(b) => {
            // if the current and the new batch type are different
            if is_diff_type(b, info) {
                return true;
            }

            // we need to check the custom pipeline to see if it's different
            let custom = match b.typ {
                BatchType::Image { .. } => &draw.image_pipeline,
                BatchType::Pattern { .. } => &draw.pattern_pipeline,
                BatchType::Shape => &draw.shape_pipeline,
                BatchType::Text { .. } => &draw.text_pipeline,
            };

            if b.pipeline.as_ref() != custom.pipeline.as_ref() {
                return true;
            }

            // new batch if the blend_mode is different
            let cbm = info.blend_mode().or(draw.blend_mode);
            if cbm != b.blend_mode {
                return true;
            }

            let abm = info.alpha_mode().or(draw.alpha_mode);
            if abm != b.alpha_mode {
                return true;
            }

            // if cfg!(not(target_os = "osx")) {
            // if b.indices.len() + info.indices().len() >= u16::MAX as usize {
            //     return true;
            // }
            // }

            // by default we batch calls
            false
        }
    }
}
