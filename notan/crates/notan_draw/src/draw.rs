use crate::batch::*;
pub(crate) use crate::custom_pipeline::CustomPipeline;
use crate::manager::DrawManager;
use crate::transform::Transform;
use glam::{Mat3, Mat4};
use notan_glyph::{Font, Text};
use notan_graphics::color::Color;
use notan_graphics::prelude::*;

#[derive(Debug, Clone)]
pub struct Draw {
    pub(crate) clear_color: Option<Color>,
    initialized: bool,
    alpha: f32,
    transform: Transform,
    base_projection: Mat4,
    projection: Option<Mat4>,
    size: (f32, f32),
    pub(crate) batches: Vec<Batch>,
    pub(crate) current_batch: Option<Batch>,
    pub(crate) shape_pipeline: CustomPipeline,
    pub(crate) image_pipeline: CustomPipeline,
    pub(crate) pattern_pipeline: CustomPipeline,
    pub(crate) text_pipeline: CustomPipeline,
    masking: bool,
}

impl Draw {
    pub fn new(width: i32, height: i32) -> Self {
        Draw {
            initialized: false,
            alpha: 1.0,
            clear_color: None,
            batches: vec![],
            current_batch: None,
            transform: Transform::new(),
            base_projection: glam::Mat4::orthographic_lh(
                0.0,
                width as _,
                height as _,
                0.0,
                -1.0,
                1.0,
            ),
            projection: None,
            size: (width as _, height as _),
            shape_pipeline: Default::default(),
            image_pipeline: Default::default(),
            pattern_pipeline: Default::default(),
            text_pipeline: Default::default(),
            masking: false,
        }
    }
    //
    // pub fn round_pixels(&mut self, _round: bool) {
    //     //TODO round pixels to draw "2d pixel games"
    //     todo!("round pixels");
    // }

    pub fn mask(&mut self, mask: Option<&Self>) {
        debug_assert!(!(self.masking && mask.is_some()), "Already using mask.");

        match mask {
            Some(m) => {
                self.masking = true;

                //Move the current batch to the queue
                if let Some(b) = self.current_batch.take() {
                    self.batches.push(b);
                }

                //Reserve the space for the mask batches
                let new_capacity = self.batches.len() + m.batches.len() + 1;
                self.batches.reserve(new_capacity);
                self.batches.extend(m.batches.iter().map(|batch| {
                    let mut b = batch.clone();
                    b.is_mask = true;
                    b
                }));

                if let Some(mut b) = m.current_batch.clone() {
                    b.is_mask = true;
                    self.batches.push(b);
                }
            }
            _ => {
                self.masking = false;
                //Move the current batch to the queue
                if let Some(b) = self.current_batch.take() {
                    self.batches.push(b);
                }
            }
        }
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.size = (width, height);
        self.base_projection = glam::Mat4::orthographic_lh(0.0, width, height, 0.0, -1.0, 1.0);
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

    pub fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn clear(&mut self, color: Color) {
        self.clear_color = Some(color);
    }

    fn add_batch<I, F1, F2>(&mut self, info: &I, check_type: F1, create_type: F2)
    where
        I: DrawInfo,
        F1: Fn(&Batch, &I) -> bool,
        F2: Fn(&I) -> BatchType,
    {
        let needs_new_batch =
            needs_new_batch(info, &self.current_batch, &self.image_pipeline, check_type);

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

            self.current_batch = Some(Batch {
                typ: create_type(info),
                vertices: vec![],
                indices: vec![],
                pipeline: custom.pipeline.clone(),
                uniform_buffers: custom.uniforms.clone(),
                blend_mode: None, //todo blend_mode
                is_mask: false,
                masking: self.masking,
            });
        }

        let global_matrix = *self.transform.matrix();
        let matrix = match *info.transform() {
            Some(m) => *m * global_matrix,
            _ => global_matrix,
        };

        if let Some(b) = &mut self.current_batch {
            b.add(info.indices(), info.vertices(), matrix, self.alpha);
        }
    }

    pub fn add_image<'a>(&mut self, info: &ImageInfo<'a>) {
        let check_type = |b: &Batch, i: &ImageInfo| {
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

        self.add_batch(info, check_type, create_type);
    }

    pub fn add_shape<'a>(&mut self, info: &ShapeInfo<'a>) {
        let check_type = |b: &Batch, _: &ShapeInfo| !b.is_shape();
        let create_type = |_: &ShapeInfo| BatchType::Shape;

        self.add_batch(info, check_type, create_type);
    }

    pub fn add_pattern<'a>(&mut self, info: &ImageInfo<'a>) {
        let check_type = |b: &Batch, i: &ImageInfo| {
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

        self.add_batch(info, check_type, create_type);
    }

    pub fn add_text<'a>(&mut self, info: &TextInfo<'a>) {
        // TODO manage text to batch?
        // self.add_batch(info, check_type, create_type);
    }

    //
    // /*
    // pub fn add_instanced<'a>(&mut self, info: &InstancedInfo<'a>) {
    //     //provide a way to draw images with draw_instanced
    // }
    //  */
}

trait DrawInfo {
    fn transform(&self) -> &Option<&Mat3>;
    fn vertices(&self) -> &[f32];
    fn indices(&self) -> &[u32];
}

/// Information to render the image or pattern
pub struct ImageInfo<'a> {
    pub texture: &'a Texture,
    pub transform: Option<&'a Mat3>,
    pub vertices: &'a [f32],
    pub indices: &'a [u32],
}

impl DrawInfo for ImageInfo<'_> {
    fn transform(&self) -> &Option<&Mat3> {
        &self.transform
    }

    fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn indices(&self) -> &[u32] {
        &self.indices
    }
}

/// Information to render the shape
pub struct ShapeInfo<'a> {
    pub transform: Option<&'a Mat3>,
    pub vertices: &'a [f32],
    pub indices: &'a [u32],
}

impl DrawInfo for ShapeInfo<'_> {
    fn transform(&self) -> &Option<&Mat3> {
        &self.transform
    }

    fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn indices(&self) -> &[u32] {
        &self.indices
    }
}

pub struct TextInfo<'a> {
    pub transform: Option<&'a Mat3>,
    pub text: &'a Text<'a>,
    pub font: &'a Font,
}

pub trait DrawRenderer {
    fn commands<'a>(
        &self,
        device: &mut Device,
        draw_manager: &'a mut DrawManager,
    ) -> &'a [Commands];
}

impl DrawRenderer for Draw {
    fn commands<'a>(&self, _: &mut Device, draw_manager: &'a mut DrawManager) -> &'a [Commands] {
        draw_manager.process_draw(self)
    }
}

fn needs_new_batch<I: DrawInfo, F: Fn(&Batch, &I) -> bool>(
    info: &I,
    current: &Option<Batch>,
    custom: &CustomPipeline,
    check_type: F,
) -> bool {
    match current {
        Some(b) => {
            if b.pipeline.as_ref() != custom.pipeline.as_ref() {
                return true; // different pipeline
            }

            //TODO check blend mode here

            check_type(b, info)
        }
        _ => true, // no previous batch
    }
}
