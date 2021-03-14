use super::color_batcher::*;
use super::draw::{Draw, DrawCommands, GraphicCommands};
use glam::Mat4;
use notan_graphics::prelude::*;

pub struct DrawManager<'b> {
    pub(crate) commands: Vec<Commands<'b>>,
    color_batcher: ColorBatcher,
}

impl<'b> DrawManager<'b> {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        let color_batcher = ColorBatcher::new(device)?;
        Ok(Self {
            commands: vec![],
            color_batcher,
        })
    }

    // pub(crate) fn process_batch<'a: 'b>(&mut self, draw: &Draw<'b>)/* -> &[Commands<'b>] */{
    //     self.commands.clear();
    //
    //     draw.commands.iter().for_each(|graphics_cmd| {
    //         match graphics_cmd {
    //             GraphicCommands::Render(cmd) => self.commands.push(cmd.clone()),
    //             _ => {}
    //             // GraphicCommands::Draw(cmd) => match cmd {
    //             //     DrawCommands::Begin(opt_color) => {
    //             //         manager.commands.push(Commands::Begin {
    //             //             color: *opt_color,
    //             //             depth: None,
    //             //             stencil: None,
    //             //         })
    //             //     }
    //             // }
    //         }
    //     });
    // }

    pub fn create_draw<'a>(&self, width: i32, height: i32) -> Draw<'a> {
        Draw::new(width, height)
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

fn process_batch<'b>(manager: &mut DrawManager<'b>, draw: &Draw<'b>) {
    manager.commands.clear();

    draw.commands.iter().for_each(|graphics_cmd| {
        match graphics_cmd {
            GraphicCommands::Render(cmd) => manager.commands.push(cmd.clone()),
            _ => {} // GraphicCommands::Draw(cmd) => match cmd {
                    //     DrawCommands::Begin(opt_color) => {
                    //         manager.commands.push(Commands::Begin {
                    //             color: *opt_color,
                    //             depth: None,
                    //             stencil: None,
                    //         })
                    //     }
                    // }
        }
    });
}
