use super::color_batcher::ColorBatcher;
use super::draw::Draw;
use crate::color::*;
use crate::graphics::*;
use crate::renderer::Renderer;
use glam::Mat4;

#[derive(Debug, Copy, Clone)]
pub enum DrawPipeline {
    Color,
    //Image,
    //Text,
}

pub(crate) struct DrawManager {
    color_batcher: ColorBatcher,
}

impl DrawManager {
    pub fn new(backend: &mut GraphicsBackend) -> Result<Self, String> {
        let color_batcher = ColorBatcher::new(backend)?;

        Ok(Self { color_batcher })
    }
}
