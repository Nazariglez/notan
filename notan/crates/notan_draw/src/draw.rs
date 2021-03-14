use super::color_batcher::*;
use super::manager::DrawMode;
use crate::manager::DrawManager;
use glam::Mat4;
use notan_graphics::prelude::*;
use std::cell::{Ref, RefCell};

#[derive(Clone)]
pub(crate) enum GraphicCommands<'a> {
    Draw(DrawCommands),
    Render(Commands<'a>),
}

#[derive(Clone)]
pub(crate) enum DrawCommands {
    Begin(Option<Color>),
    SetColor(Color),
    SetAlpha(f32),
    Triangle {
        vertices: [f32; 6],
        indices: [u32; 3],
        color: [f32; 4],
    },
    Rect {
        vertices: [f32; 8],
        indices: [u32; 4],
        color: [f32; 4],
    },
}

#[derive(Clone)]
pub struct Draw<'a> {
    size: (i32, i32),
    pub(crate) commands: Vec<GraphicCommands<'a>>,

    color: Color,
    alpha: f32,
}

impl<'a> Draw<'a> {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            size: (width, height),
            commands: vec![],
            color: Color::WHITE,
            alpha: 1.0,
        }
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
        self.commands
            .push(GraphicCommands::Render(Commands::Size { width, height }));
    }

    pub fn size(&self) -> (i32, i32) {
        self.size
    }

    pub fn width(&self) -> i32 {
        self.size.0
    }

    pub fn height(&self) -> i32 {
        self.size.1
    }

    pub fn set_pipeline(&mut self, pipeline: &Pipeline) {
        self.commands
            .push(GraphicCommands::Render(Commands::Pipeline {
                id: pipeline.id(),
                options: pipeline.options.clone(),
            }));
    }

    pub fn set_color(&mut self, color: &Color) {
        self.color = *color;

        self.commands
            .push(GraphicCommands::Draw(DrawCommands::SetColor(*color)));
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;

        self.commands
            .push(GraphicCommands::Draw(DrawCommands::SetAlpha(alpha)));
    }

    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    pub fn begin(&mut self, color: Option<&Color>) {
        self.commands
            .push(GraphicCommands::Draw(DrawCommands::Begin(
                color.map(|c| *c),
            )));
    }

    pub fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let [r, g, b, a] = self.color.to_rgba();

        #[rustfmt::skip]
        let triangle = DrawCommands::Triangle {
            vertices: [
                x1, y1,
                x2, y2,
                x3, y3,
            ],
            indices: [0, 1, 2],
            color: [r, g, b, a * self.alpha]
        };

        self.commands.push(GraphicCommands::Draw(triangle));
    }

    // pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
    //     #[rustfmt::skip]
    //     let vertices = [
    //         x, y,
    //         x + width, y,
    //         x, y + height,
    //         x + width, y + width
    //     ];
    //
    //     self.commands
    //         .push(GraphicCommands::Draw(DrawCommands::Rect(vertices)))
    // }

    pub fn end(&mut self) {
        self.commands.push(GraphicCommands::Render(Commands::End));
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

// // TODO cargo make

pub trait DrawRenderer<'a> {
    fn commands(
        &self,
        device: &mut Device,
        draw_manager: &'a mut DrawManager,
    ) -> &'a [Commands<'a>];
}

impl<'a> DrawRenderer<'a> for Draw<'a> {
    fn commands(&self, _: &mut Device, draw_manager: &'a mut DrawManager) -> &'a [Commands<'a>] {
        // draw_manager.process_batch(self);
        &draw_manager.commands
    }
}
