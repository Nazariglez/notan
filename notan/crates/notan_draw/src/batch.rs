use glam::{Mat3, Vec3};
use notan_glyph::{Font, Text};
use notan_graphics::prelude::*;

#[derive(Clone, Debug)]
pub(crate) enum BatchType {
    Image { texture: Texture },
    Pattern { texture: Texture },
    Shape,
    Text { font: Font },
}

#[derive(Clone, Debug)]
pub(crate) struct Batch {
    pub typ: BatchType,
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub pipeline: Option<Pipeline>,
    pub uniform_buffers: Option<Vec<Buffer<f32>>>,
    pub blend_mode: Option<BlendMode>,
    pub is_mask: bool,
    pub masking: bool,
}

impl Batch {
    pub fn is_shape(&self) -> bool {
        matches!(self.typ, BatchType::Shape)
    }

    pub fn add(&mut self, indices: &[u32], vertices: &[f32], matrix: Mat3, alpha: f32) {
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
            BatchType::Text { .. } => 8, //TODO check offset
        }
    }
}
