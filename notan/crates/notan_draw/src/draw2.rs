use crate::manager::DrawManager;
use crate::transform::Transform;
use crate::DrawRenderer;
use glam::{Mat3, Vec2};
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

        if let Some(matrix) = info.transform {
            self.transform.push(*matrix);
        }

        match &mut self.current_batch {
            DrawBatch::Shape {
                vertices, indices, ..
            } => {
                //multiply by matrix, alpha and color
                let index = (vertices.len() as u32) / 6;
                vertices.extend(info.vertices);
                indices.extend(info.indices.iter().map(|i| i + index).collect::<Vec<_>>());
            }
            _ => {}
        }

        if info.transform.is_some() {
            self.transform.pop();
        }
    }
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
