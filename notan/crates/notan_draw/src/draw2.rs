pub(crate) use crate::custom_pipeline::CustomPipeline;
use crate::manager::DrawManager;
use crate::transform::Transform;
// use crate::DrawRenderer;
use glam::{Mat3, Mat4, Vec2, Vec3};
use notan_graphics::color::Color;
use notan_graphics::prelude::*;

#[derive(Clone, Debug)]
pub(crate) enum BatchType {
    Image { texture: Texture },
    Pattern { texture: Texture },
    Shape,
    // Text {
    //     font: Font
    // }
}

pub(crate) struct Batch {
    pub typ: BatchType,
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub pipeline: Option<Pipeline>,
    pub uniform_buffers: Option<Vec<Buffer<f32>>>,
    pub blend_mode: Option<BlendMode>,
}

impl Batch {
    fn new(typ: BatchType) -> Self {
        Self {
            typ,
            vertices: vec![],
            indices: vec![],
            pipeline: None,
            uniform_buffers: None,
            blend_mode: None,
        }
    }

    fn is_shape(&self) -> bool {
        match &self.typ {
            BatchType::Shape => true,
            _ => false,
        }
    }

    fn is_image(&self) -> bool {
        match &self.typ {
            BatchType::Image { .. } => true,
            _ => false,
        }
    }

    fn is_pattern(&self) -> bool {
        match &self.typ {
            BatchType::Pattern { .. } => true,
            _ => false,
        }
    }

    fn add(&mut self, indices: &[u32], vertices: &[f32], matrix: Mat3, alpha: f32) {
        let offset = self.offset();

        //compute indices
        let last_index = (self.vertices.len() / offset) as u32;
        self.indices.reserve(self.indices.len() + indices.len());
        self.indices.extend(indices.iter().map(|i| i + last_index));

        //compute vertices
        self.vertices.reserve(self.vertices.len() + vertices.len());
        (0..vertices.len()).step_by(offset).for_each(|i| {
            let start = i + 2;
            let end = i + offset - 1;
            let xyz = matrix * Vec3::new(vertices[i], vertices[i + 1], 1.0);
            self.vertices.extend(&[xyz.x, xyz.y]); //pos
            self.vertices.extend(&vertices[start..end]); //pipeline attrs and rgb
            self.vertices.push(vertices[i + offset - 1] * alpha); //alpha
        });
    }

    fn offset(&self) -> usize {
        match &self.typ {
            BatchType::Image { .. } => 8,
            BatchType::Pattern { .. } => 12,
            BatchType::Shape => 6,
            _ => 0, //todo text
        }
    }
}

pub struct Draw2 {
    pub(crate) background: Option<Color>,
    pub(crate) initialized: bool,
    pub(crate) color: Color,
    pub(crate) alpha: f32,
    pub(crate) batches: Vec<Batch>,
    pub(crate) current_batch: Option<Batch>,
    transform: Transform,
    base_projection: Mat4,
    projection: Option<Mat4>,
    size: (f32, f32),
    pub(crate) shape_pipeline: CustomPipeline,
    pub(crate) image_pipeline: CustomPipeline,
    pub(crate) pattern_pipeline: CustomPipeline,
    pub(crate) text_pipeline: CustomPipeline,
}

impl Draw2 {
    pub fn new(width: i32, height: i32) -> Self {
        Draw2 {
            initialized: false,
            color: Color::WHITE,
            alpha: 1.0,
            background: None,
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
        self.projection.unwrap_or_else(|| self.base_projection)
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

    pub fn background(&mut self, color: Color) {
        self.background = Some(color);
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
                //TODO text
            };

            self.current_batch = Some(Batch {
                typ: create_type(info),
                vertices: vec![],
                indices: vec![],
                pipeline: custom.pipeline.clone(),
                uniform_buffers: custom.uniforms.clone(),
                blend_mode: None, //todo blend_mode
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
    //
    // /*
    // pub fn add_instanced<'a>(&mut self, info: &InstancedInfo<'a>) {
    //     //provide a way to draw images with draw_instanced
    // }
    //  */
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

            return check_type(b, info);
        }
        _ => true, // no previous batch
    }
}

trait DrawInfo {
    fn transform(&self) -> &Option<&Mat3>;
    fn vertices(&self) -> &[f32];
    fn indices(&self) -> &[u32];
}

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

pub trait DrawRenderer {
    fn commands<'a>(
        &self,
        device: &mut Device,
        draw_manager: &'a mut DrawManager,
    ) -> &'a [Commands];
}

impl DrawRenderer for Draw2 {
    fn commands<'a>(&self, _: &mut Device, draw_manager: &'a mut DrawManager) -> &'a [Commands] {
        draw_manager.process_draw2(self)
    }
}
