use crate::batch::*;
use crate::manager::process_pipeline;
use glam::Mat4;
use notan_glyph::{GlyphManager, GlyphRenderer};
use notan_graphics::prelude::*;
use notan_macro::{fragment_shader, vertex_shader};

pub(crate) struct TextPainter {}

impl TextPainter {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        Ok(Self {})
    }

    pub fn push(&mut self, renderer: &mut Renderer, batch: &Batch, projection: &Mat4) {}

    pub fn clear(&mut self) {}
}
