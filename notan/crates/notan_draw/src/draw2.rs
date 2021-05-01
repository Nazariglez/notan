use crate::manager::DrawManager;
use crate::transform::Transform;
use crate::DrawRenderer;
use glam::{Mat3, Mat4, Vec2, Vec3};
use notan_graphics::color::Color;
use notan_graphics::prelude::*;

#[derive(Debug, Clone)]
pub(crate) enum DrawBatch {
    None,
    Shape {
        pipeline: Option<Pipeline>,
        vertices: Vec<f32>,
        indices: Vec<u32>,
    },
    Image {
        pipeline: Option<Pipeline>,
        vertices: Vec<f32>,
        indices: Vec<u32>,
        texture: Texture,
    },
    Pattern {
        pipeline: Option<Pipeline>,
        vertices: Vec<f32>,
        indices: Vec<u32>,
        texture: Texture,
    },
}

impl DrawBatch {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

    pub fn is_shape(&self) -> bool {
        match self {
            Self::Shape { .. } => true,
            _ => false,
        }
    }

    pub fn is_image(&self) -> bool {
        match self {
            Self::Image { .. } => true,
            _ => false,
        }
    }
}

pub struct Draw2 {
    pub(crate) background: Option<Color>,
    pub(crate) initialized: bool,
    pub(crate) color: Color,
    pub(crate) alpha: f32,
    pub(crate) batches: Vec<DrawBatch>,
    pub(crate) current_batch: DrawBatch,
    transform: Transform,
    base_projection: Mat4,
    projection: Option<Mat4>,
    size: (f32, f32),
}

impl Draw2 {
    pub fn new(width: i32, height: i32) -> Self {
        Draw2 {
            initialized: false,
            color: Color::WHITE,
            alpha: 1.0,
            background: None,
            batches: vec![],
            current_batch: DrawBatch::None,
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

    pub fn add_image<'a>(&mut self, info: &ImageInfo<'a>) {
        let needs_new_batch = match &self.current_batch {
            DrawBatch::Image { texture, .. } => texture != info.texture,
            _ => true,
        };

        if needs_new_batch {
            let old = std::mem::replace(
                &mut self.current_batch,
                DrawBatch::Image {
                    pipeline: None,
                    vertices: vec![],
                    indices: vec![],
                    texture: info.texture.clone(),
                },
            );
            if !old.is_none() {
                self.batches.push(old);
            }
        }

        let global_matrix = *self.transform.matrix();
        let matrix = match info.transform {
            Some(m) => *m * global_matrix,
            _ => global_matrix,
        };

        match &mut self.current_batch {
            DrawBatch::Image {
                texture,
                vertices,
                indices,
                ..
            } => {
                let last_index = (vertices.len() as u32) / 8;
                add_indices(indices, info.indices, last_index);
                add_image_vertices(vertices, info.vertices, matrix, self.alpha);
            }
            _ => {}
        }
    }

    pub fn add_shape<'a>(&mut self, info: &ShapeInfo<'a>) {
        //new batch if pipelines changes, otherwise add to the current one the vertices
        if !self.current_batch.is_shape() {
            let old = std::mem::replace(
                &mut self.current_batch,
                DrawBatch::Shape {
                    pipeline: None,
                    vertices: vec![],
                    indices: vec![],
                },
            );
            if !old.is_none() {
                self.batches.push(old);
            }
        }

        let global_matrix = *self.transform.matrix();
        let matrix = match info.transform {
            Some(m) => *m * global_matrix,
            _ => global_matrix,
        };

        match &mut self.current_batch {
            DrawBatch::Shape {
                vertices, indices, ..
            } => {
                let last_index = (vertices.len() as u32) / 6;
                add_indices(indices, info.indices, last_index);
                add_shape_vertices(vertices, info.vertices, matrix, self.alpha);
            }
            _ => {}
        }
    }

    pub fn add_pattern<'a>(&mut self, info: &ImageInfo<'a>) {
        let needs_new_batch = match &self.current_batch {
            DrawBatch::Pattern { texture, .. } => texture != info.texture,
            _ => true,
        };

        if needs_new_batch {
            let old = std::mem::replace(
                &mut self.current_batch,
                DrawBatch::Pattern {
                    pipeline: None,
                    vertices: vec![],
                    indices: vec![],
                    texture: info.texture.clone(),
                },
            );
            if !old.is_none() {
                self.batches.push(old);
            }
        }

        let global_matrix = *self.transform.matrix();
        let matrix = match info.transform {
            Some(m) => *m * global_matrix,
            _ => global_matrix,
        };

        match &mut self.current_batch {
            DrawBatch::Pattern {
                texture,
                vertices,
                indices,
                ..
            } => {
                let last_index = (vertices.len() as u32) / 12;
                add_indices(indices, info.indices, last_index);
                add_pattern_vertices(vertices, info.vertices, matrix, self.alpha);
            }
            _ => {}
        }
    }

    /*
    pub fn add_instanced<'a>(&mut self, info: &InstancedInfo<'a>) {
        //provide a way to draw images with draw_instanced
    }
     */
}

#[inline]
fn add_pattern_vertices(to: &mut Vec<f32>, from: &[f32], matrix: Mat3, alpha: f32) {
    let computed = (0..from.len())
        .step_by(12)
        .map(|i| {
            let xyz = matrix * Vec3::new(from[i], from[i + 1], 1.0);
            [
                xyz.x,
                xyz.y,
                from[i + 2],          //uv1
                from[i + 3],          //uv2
                from[i + 4],          //fx
                from[i + 5],          //fy
                from[i + 6],          //fw
                from[i + 7],          //fh
                from[i + 8],          //r
                from[i + 9],          //g
                from[i + 10],         //b
                from[i + 11] * alpha, //a
            ]
        })
        .collect::<Vec<_>>()
        .concat();
    to.extend(computed);
}

#[inline]
fn add_image_vertices(to: &mut Vec<f32>, from: &[f32], matrix: Mat3, alpha: f32) {
    let computed = (0..from.len())
        .step_by(8)
        .map(|i| {
            let xyz = matrix * Vec3::new(from[i], from[i + 1], 1.0);
            [
                xyz.x,
                xyz.y,
                from[i + 2],         //uv1
                from[i + 3],         //uv2
                from[i + 4],         //r
                from[i + 5],         //g
                from[i + 6],         //b
                from[i + 7] * alpha, //a
            ]
        })
        .collect::<Vec<_>>()
        .concat();
    to.extend(computed);
}

#[inline]
fn add_shape_vertices(to: &mut Vec<f32>, from: &[f32], matrix: Mat3, alpha: f32) {
    let computed = (0..from.len())
        .step_by(6)
        .map(|i| {
            let xyz = matrix * Vec3::new(from[i], from[i + 1], 1.0);
            [
                xyz.x,
                xyz.y,
                from[i + 2],         //r
                from[i + 3],         //g
                from[i + 4],         //b
                from[i + 5] * alpha, //a
            ]
        })
        .collect::<Vec<_>>()
        .concat();
    to.extend(computed);
}

#[inline]
fn add_indices(to: &mut Vec<u32>, from: &[u32], last_index: u32) {
    to.extend(from.iter().map(|i| i + last_index).collect::<Vec<_>>());
}

pub struct ImageInfo<'a> {
    pub texture: &'a Texture,
    pub transform: Option<&'a Mat3>,
    pub vertices: &'a [f32],
    pub indices: &'a [u32],
}

pub struct ShapeInfo<'a> {
    pub transform: Option<&'a Mat3>,
    pub vertices: &'a [f32],
    pub indices: &'a [u32],
}

impl DrawRenderer for Draw2 {
    fn commands<'a>(&self, _: &mut Device, draw_manager: &'a mut DrawManager) -> &'a [Commands] {
        draw_manager.process_draw2(self)
    }
}
