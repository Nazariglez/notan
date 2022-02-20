use crate::color::Color;
use crate::pipeline::*;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Commands {
    Size {
        width: i32,
        height: i32,
    },
    Viewport {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    Begin {
        color: Option<Color>,
        depth: Option<f32>,
        stencil: Option<i32>,
    },
    End,
    Pipeline {
        id: u64,
        options: PipelineOptions,
    },
    BindBuffer {
        id: u64,
    },
    BindTexture {
        id: u64,
        slot: u32,
        location: u32,
    },
    Scissors {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    Draw {
        primitive: DrawPrimitive,
        offset: i32,
        count: i32,
    },
    DrawInstanced {
        primitive: DrawPrimitive,
        offset: i32,
        count: i32,
        length: i32,
    },
}
