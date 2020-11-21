use crate::buffer::*;
use crate::color::Color;
use crate::pipeline::*;

struct TextureId(i32);

#[allow(unused)]
#[derive(Debug)]
pub enum Commands<'a> {
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
        id: PipelineId,
        options: PipelineOptions,
    },
    BindBuffer {
        id: BufferId,
        ptr: &'a [u8],
        usage: BufferUsage,
        draw: DrawType,
    },
    Draw {
        offset: i32,
        count: i32,
    },
}

pub trait ToCommandBuffer<'a> {
    fn commands(&'a self) -> &'a [Commands<'a>];
}
