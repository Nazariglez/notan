use super::images::*;
use super::patterns::*;
use super::shapes::*;
use crate::batch::*;
use crate::draw::*;
use glam::Mat4;
use notan_graphics::prelude::*;

pub struct DrawManager {
    shape_painter: ShapePainter,
    image_painter: ImagePainter,
    pattern_painter: PatternPainter,
    renderer: Renderer,
    drawing_mask: bool,
}

impl DrawManager {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let shape_painter = ShapePainter::new(device)?;
        let image_painter = ImagePainter::new(device)?;
        let pattern_painter = PatternPainter::new(device)?;
        let renderer = device.create_renderer();
        Ok(Self {
            shape_painter,
            image_painter,
            pattern_painter,
            renderer,
            drawing_mask: false,
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
        _device: &mut Device,
        _fragment: Option<&ShaderSource>,
    ) -> Result<Pipeline, String> {
        unimplemented!()
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
        BatchType::Text { .. } => {
            unimplemented!()
        }
    }
}

fn process_draw(manager: &mut DrawManager, draw: &Draw) {
    manager.image_painter.clear();
    manager.shape_painter.clear();
    manager.pattern_painter.clear();

    manager.renderer.begin(Some(&ClearOptions {
        color: draw.clear_color,
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

fn override_pipeline_options(
    pipeline: &Pipeline,
    is_mask: bool,
    masking: bool,
) -> Option<Pipeline> {
    if is_mask {
        let mut pip = pipeline.clone();
        pip.options.stencil = Some(StencilOptions {
            stencil_fail: StencilAction::Keep,
            depth_fail: StencilAction::Keep,
            pass: StencilAction::Replace,
            compare: CompareMode::Always,
            read_mask: 0xff,
            write_mask: 0xff,
            reference: 1,
        });
        pip.options.depth_stencil.write = false;
        pip.options.color_mask = ColorMask::NONE;
        return Some(pip);
    }

    if masking {
        let mut pip = pipeline.clone();
        pip.options.stencil = Some(StencilOptions {
            stencil_fail: StencilAction::Keep,
            depth_fail: StencilAction::Keep,
            pass: StencilAction::Replace,
            compare: CompareMode::Equal,
            read_mask: 0xff,
            write_mask: 0x00,
            reference: 1,
        });
        pip.options.depth_stencil.write = true;
        pip.options.color_mask = ColorMask::ALL;
        return Some(pip);
    }

    None
}

pub(crate) fn process_pipeline(
    renderer: &mut Renderer,
    batch: &Batch,
    default_pipeline: &Pipeline,
) {
    match &batch.pipeline {
        Some(pip) => {
            match override_pipeline_options(pip, batch.is_mask, batch.masking) {
                Some(pip) => renderer.set_pipeline(&pip),
                _ => renderer.set_pipeline(pip),
            };

            if let Some(buffers) = &batch.uniform_buffers {
                buffers.iter().for_each(|u| renderer.bind_uniform_buffer(u));
            }
        }
        _ => {
            match override_pipeline_options(default_pipeline, batch.is_mask, batch.masking) {
                Some(pip) => renderer.set_pipeline(&pip),
                _ => renderer.set_pipeline(default_pipeline),
            };
        }
    }
}
