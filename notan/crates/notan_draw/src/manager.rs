use super::color_batcher::*;
use super::draw::{Draw, DrawCommands, GraphicCommands};
use super::images::ImagePainter;
use super::patterns::PatternPainter;
use super::shapes::ShapePainter;
use crate::draw2::{Draw2, DrawBatch};
use glam::Mat4;
use notan_graphics::prelude::*;

pub struct DrawManager {
    pub(crate) commands: Vec<Commands>,
    color_batcher: ColorBatcher,
    current_projection: [f32; 16],
    shape_painter: ShapePainter,
    image_painter: ImagePainter,
    pattern_painter: PatternPainter,
    renderer: Renderer,
}

impl DrawManager {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let color_batcher = ColorBatcher::new(device)?;
        let shape_painter = ShapePainter::new(device)?;
        let image_painter = ImagePainter::new(device)?;
        let pattern_painter = PatternPainter::new(device)?;
        let renderer = device.create_renderer();
        Ok(Self {
            commands: vec![],
            color_batcher,
            current_projection: [0.0; 16],
            shape_painter,
            image_painter,
            pattern_painter,
            renderer,
        })
    }

    pub(crate) fn process_batch(&mut self, draw: &Draw) -> &[Commands] {
        process_batch(self, draw);
        &self.commands
    }

    pub(crate) fn process_draw2(&mut self, draw: &Draw2) -> &[Commands] {
        self.renderer.clear();
        process_draw2(self, draw);
        &self.renderer.commands()
    }

    pub fn create_draw(&self, width: i32, height: i32) -> Draw {
        Draw::new(width, height)
    }

    pub fn create_draw2(&self, width: i32, height: i32) -> Draw2 {
        Draw2::new(width, height)
    }

    #[inline]
    pub fn create_pipeline(
        &self,
        device: &mut Device,
        mode: DrawMode,
        fragment: Option<&ShaderSource>,
    ) -> Result<Pipeline, String> {
        create_draw_pipeline(device, mode, fragment)
    }

    #[inline]
    pub fn create_pipeline_from_raw(
        &self,
        device: &mut Device,
        mode: DrawMode,
        fragment: Option<&[u8]>,
    ) -> Result<Pipeline, String> {
        create_draw_pipeline_from_raw(device, mode, fragment)
    }
}

#[inline]
pub fn create_draw_pipeline(
    device: &mut Device,
    mode: DrawMode,
    fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    match mode {
        DrawMode::Color => create_color_pipeline(device, fragment),
    }
}

#[inline]
pub fn create_draw_pipeline_from_raw(
    device: &mut Device,
    mode: DrawMode,
    fragment: Option<&[u8]>,
) -> Result<Pipeline, String> {
    match mode {
        DrawMode::Color => create_color_pipeline_from_raw(device, fragment),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DrawMode {
    Color,
    //Image,
    //Text,
}

fn process_draw2(manager: &mut DrawManager, draw: &Draw2) {
    manager.image_painter.clear();
    manager.shape_painter.clear();
    manager.pattern_painter.clear();

    manager.renderer.begin(Some(&ClearOptions {
        color: draw.background.clone(),
        ..Default::default()
    }));
    //TODO, if this last batch type was different we should end and begin the pass again
    let projection = draw.projection();
    draw.batches.iter().for_each(|b| match b {
        DrawBatch::Shape { .. } => {
            manager
                .shape_painter
                .push(&mut manager.renderer, b, &projection);
        }
        DrawBatch::Image { .. } => {
            manager
                .image_painter
                .push(&mut manager.renderer, b, &projection);
        }
        DrawBatch::Pattern { .. } => {
            manager
                .pattern_painter
                .push(&mut manager.renderer, b, &projection);
        }
        _ => {}
    });
    match &draw.current_batch {
        DrawBatch::Shape { .. } => {
            manager
                .shape_painter
                .push(&mut manager.renderer, &draw.current_batch, &projection);
        }
        DrawBatch::Image { .. } => {
            manager
                .image_painter
                .push(&mut manager.renderer, &draw.current_batch, &projection);
        }
        DrawBatch::Pattern { .. } => {
            manager
                .pattern_painter
                .push(&mut manager.renderer, &draw.current_batch, &projection);
        }
        _ => {}
    }
    manager.renderer.end();
}

fn process_batch(manager: &mut DrawManager, draw: &Draw) {
    manager.commands.clear();

    draw.commands
        .iter()
        .for_each(|graphics_cmd| match graphics_cmd {
            GraphicCommands::Render(cmd) => process_render_commands(manager, cmd.clone()),
            GraphicCommands::Draw(cmd) => process_draw_commands(manager, cmd.clone()),
        });
}

fn process_render_commands(manager: &mut DrawManager, cmd: Commands) {
    use Commands::*;

    match cmd {
        End => {
            manager.color_batcher.flush(
                None,
                &mut manager.current_projection,
                &mut manager.commands,
            );
        }
        _ => {}
    }

    manager.commands.push(cmd);
}

fn process_draw_commands(manager: &mut DrawManager, cmd: DrawCommands) {
    use DrawCommands::*;

    match cmd {
        Begin(opt_color) => manager.commands.push(Commands::Begin {
            color: opt_color,
            depth: None,
            stencil: None,
        }),
        Triangle {
            vertices,
            indices,
            color,
        } => manager.color_batcher.push(
            ColorData {
                vertices: &vertices,
                indices: &indices,
                pipeline: None,
                color: Some(&color),
                projection: &manager.current_projection,
            },
            &mut manager.commands,
        ),
        Rect {
            vertices,
            indices,
            color,
        } => manager.color_batcher.push(
            ColorData {
                vertices: &vertices,
                indices: &indices,
                pipeline: None,
                color: Some(&color),
                projection: &manager.current_projection,
            },
            &mut manager.commands,
        ),
        Projection(projecton) => manager.current_projection = projecton.to_cols_array(),
        RawColor { vertices, indices } => manager.color_batcher.push(
            ColorData {
                vertices: &vertices,
                indices: &indices,
                pipeline: None,
                color: None,
                projection: &manager.current_projection,
            },
            &mut manager.commands,
        ),
        _ => {}
    };
}
