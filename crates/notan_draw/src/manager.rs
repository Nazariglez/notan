use super::images::*;
use super::patterns::*;
use super::shapes::*;
use super::texts::*;
use crate::batch::*;
use crate::draw::*;
use notan_glyph::GlyphBrush;
use notan_graphics::prelude::*;
use notan_math::Mat4;

pub struct DrawManager {
    shape_painter: ShapePainter,
    image_painter: ImagePainter,
    pattern_painter: PatternPainter,
    text_painter: TextPainter,
    renderer: Renderer,
    drawing_mask: bool,
}

impl DrawManager {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let shape_painter = ShapePainter::new(device)?;
        let image_painter = ImagePainter::new(device)?;
        let pattern_painter = PatternPainter::new(device)?;
        let text_painter = TextPainter::new(device)?;
        let renderer = device.create_renderer();
        Ok(Self {
            shape_painter,
            image_painter,
            pattern_painter,
            text_painter,
            renderer,
            drawing_mask: false,
        })
    }

    pub(crate) fn process_draw(
        &mut self,
        draw: &Draw,
        device: &mut Device,
        glyphs: &mut GlyphBrush,
        flip_projection: bool,
    ) -> &[Commands] {
        self.renderer.clear();
        process_draw(self, draw, device, glyphs, flip_projection);
        self.renderer.commands()
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

fn paint_batch(
    device: &mut Device,
    manager: &mut DrawManager,
    glyphs: &mut GlyphBrush,
    b: &Batch,
    projection: &Mat4,
) {
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
            manager
                .text_painter
                .push(device, glyphs, &mut manager.renderer, b, projection)
        }
    }
}

fn process_glyphs(
    manager: &mut DrawManager,
    draw: &Draw,
    device: &mut Device,
    glyphs: &mut GlyphBrush,
) {
    if let Some(indices) = &draw.text_batch_indices {
        let batch_len = draw.batches.len();
        let mut last_index = std::usize::MAX;
        indices.iter().for_each(|i| {
            let n = *i;
            if n == last_index {
                return;
            }
            last_index = n;

            let batch = if n >= batch_len {
                draw.current_batch.as_ref()
            } else {
                draw.batches.get(n)
            };

            if let Some(b) = batch {
                if let BatchType::Text { texts } = &b.typ {
                    texts.iter().for_each(|data| {
                        glyphs.queue(&data.section);
                    });
                }
            }
        });

        glyphs.process_queued(device, &mut manager.text_painter);
    }
}

fn process_draw(
    manager: &mut DrawManager,
    draw: &Draw,
    device: &mut Device,
    glyphs: &mut GlyphBrush,
    flip_projection: bool,
) {
    process_glyphs(manager, draw, device, glyphs);

    manager.image_painter.clear();
    manager.shape_painter.clear();
    manager.pattern_painter.clear();
    manager.text_painter.clear();

    manager.renderer.begin(Some(&ClearOptions {
        color: draw.clear_color,
        ..Default::default()
    }));

    let mut projection = draw.projection();

    // used to flip axis-y for opengl framebuffers
    // check https://github.com/Nazariglez/notan/issues/151
    if flip_projection {
        projection.y_axis.y *= -1.0;
        projection.w_axis.y *= -1.0;
    }

    draw.batches
        .iter()
        .for_each(|b| paint_batch(device, manager, glyphs, b, &projection));
    if let Some(current) = &draw.current_batch {
        paint_batch(device, manager, glyphs, current, &projection);
    }

    manager.renderer.end();

    manager.image_painter.upload_buffers(device);
    manager.shape_painter.upload_buffers(device);
    manager.pattern_painter.upload_buffers(device);
    manager.text_painter.upload_buffers(device);
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
            let masked = masked_pip(pip, batch.is_mask, batch.masking);
            let pip_to_use = masked.as_ref().unwrap_or(pip);
            let blended = blended_pip(pip_to_use, batch.blend_mode);
            let final_pip = blended.as_ref().unwrap_or(pip_to_use);
            renderer.set_pipeline(final_pip);

            if let Some(buffers) = &batch.uniform_buffers {
                buffers.iter().for_each(|u| renderer.bind_buffer(u));
            }
        }
        _ => {
            let masked = masked_pip(default_pipeline, batch.is_mask, batch.masking);
            let pip_to_use = masked.as_ref().unwrap_or(default_pipeline);
            let blended = blended_pip(pip_to_use, batch.blend_mode);
            let final_pip = blended.as_ref().unwrap_or(pip_to_use);
            renderer.set_pipeline(final_pip);
        }
    }
}

fn masked_pip(pip: &Pipeline, is_mask: bool, masking: bool) -> Option<Pipeline> {
    override_pipeline_options(pip, is_mask, masking)
}

fn blended_pip(pip: &Pipeline, blend_mode: BlendMode) -> Option<Pipeline> {
    match pip.options.color_blend {
        Some(bm) => {
            if bm != blend_mode {
                let mut blend_pip = pip.clone();
                blend_pip.options.color_blend = Some(blend_mode);
                Some(blend_pip)
            } else {
                None
            }
        }
        _ => None,
    }
}
