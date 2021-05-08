// use super::draw::{Draw, DrawCommands, GraphicCommands};
use super::fonts::*;
use super::images::*;
use super::patterns::*;
use super::shapes::*;
use crate::batch::*;
use crate::draw::*;
use glam::Mat4;
use notan_graphics::prelude::*;

pub struct DrawManager {
    pub(crate) commands: Vec<Commands>,
    shape_painter: ShapePainter,
    image_painter: ImagePainter,
    pattern_painter: PatternPainter,
    renderer: Renderer,
    drawing_mask: bool,
    masking: bool,
}

impl DrawManager {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let shape_painter = ShapePainter::new(device)?;
        let image_painter = ImagePainter::new(device)?;
        let pattern_painter = PatternPainter::new(device)?;
        let renderer = device.create_renderer();
        Ok(Self {
            commands: vec![],
            shape_painter,
            image_painter,
            pattern_painter,
            renderer,
            drawing_mask: false,
            masking: false,
        })
    }

    pub(crate) fn process_draw(&mut self, draw: &Draw) -> &[Commands] {
        self.renderer.clear();
        process_draw(self, draw);
        &self.renderer.commands()
    }

    pub fn create_draw(&self, width: i32, height: i32) -> Draw {
        Draw::new(width, height)
    }

    pub fn create_image_pipeline(
        &self,
        device: &mut Device,
        fragment: Option<&ShaderSource>,
    ) -> Result<Pipeline, String> {
        create_image_pipeline(device, fragment)
    }

    pub fn create_pattern_pipeline(
        &self,
        device: &mut Device,
        fragment: Option<&ShaderSource>,
    ) -> Result<Pipeline, String> {
        create_pattern_pipeline(device, fragment)
    }

    pub fn create_shape_pipeline(
        &self,
        device: &mut Device,
        fragment: Option<&ShaderSource>,
    ) -> Result<Pipeline, String> {
        create_shape_pipeline(device, fragment)
    }

    pub fn create_text_pipeline(
        &self,
        device: &mut Device,
        fragment: Option<&ShaderSource>,
    ) -> Result<Pipeline, String> {
        create_text_pipeline(device, fragment)
    }
}

fn paint_batch(manager: &mut DrawManager, b: &Batch, projection: &Mat4) {
    if b.is_mask && !manager.drawing_mask {
        manager.renderer.end();
        manager.drawing_mask = true;
    } else if !b.is_mask && manager.drawing_mask {
        manager.drawing_mask = false;
        manager.renderer.begin(Some(&Default::default()));
    }

    match &b.typ {
        BatchType::Image { .. } => manager
            .image_painter
            .push(&mut manager.renderer, b, projection),
        BatchType::Shape => manager
            .shape_painter
            .push(&mut manager.renderer, b, projection),
        BatchType::Pattern { .. } => {
            manager
                .pattern_painter
                .push(&mut manager.renderer, b, projection)
        }
        _ => {} //TODO text
    }
}

fn process_draw(manager: &mut DrawManager, draw: &Draw) {
    manager.image_painter.clear();
    manager.shape_painter.clear();
    manager.pattern_painter.clear();

    manager.renderer.begin(Some(&ClearOptions {
        color: draw.background.clone(),
        ..Default::default()
    }));

    let projection = draw.projection();
    draw.batches
        .iter()
        .for_each(|b| paint_batch(manager, b, &projection));
    if let Some(current) = &draw.current_batch {
        paint_batch(manager, current, &projection);
    }

    manager.renderer.end();
}
