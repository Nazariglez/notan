use crate::manager::DrawManager;
use crate::transform::Transform;
use crate::DrawRenderer;
use glam::{Mat3, Vec2, Vec3};
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
}

pub struct Draw2 {
    pub(crate) background: Option<Color>,
    pub(crate) initialized: bool,
    pub(crate) color: Color,
    pub(crate) alpha: f32,
    pub(crate) batches: Vec<DrawBatch>,
    pub(crate) current_batch: DrawBatch,
    transform: Transform,
}

impl Draw2 {
    pub fn new() -> Self {
        Draw2 {
            initialized: false,
            color: Color::WHITE,
            alpha: 1.0,
            background: None,
            batches: vec![],
            current_batch: DrawBatch::None,
            transform: Transform::new(),
        }
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

    pub fn shape<'a>(&mut self, info: &ShapeInfo<'a>) {
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
                add_vertices(vertices, info.vertices, matrix, self.alpha);
            }
            _ => {}
        }
    }
}

#[inline]
fn add_vertices(to: &mut Vec<f32>, from: &[f32], matrix: Mat3, alpha: f32) {
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
