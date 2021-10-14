use crate::buffer::*;
use crate::color::Color;
use crate::pipeline::*;
use parking_lot::RwLock;
use std::sync::Arc;

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
        id: i32,
        options: PipelineOptions,
    },
    BindBuffer {
        id: i32,
        data: BufferDataWrapper,
        usage: BufferUsage,
        draw: DrawType,
    },
    BindTexture {
        id: i32,
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
        offset: i32,
        count: i32,
    },
}

impl From<&Buffer<u32>> for Commands {
    fn from(buffer: &Buffer<u32>) -> Commands {
        Commands::BindBuffer {
            id: buffer.id(),
            data: BufferDataWrapper::Uint32(buffer.data_ptr().clone()),
            usage: buffer.usage,
            draw: buffer.draw.unwrap_or(DrawType::Dynamic),
        }
    }
}

impl From<&Buffer<f32>> for Commands {
    fn from(buffer: &Buffer<f32>) -> Commands {
        Commands::BindBuffer {
            id: buffer.id(),
            data: BufferDataWrapper::Float32(buffer.data_ptr().clone()),
            usage: buffer.usage,
            draw: buffer.draw.unwrap_or(DrawType::Dynamic),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BufferDataWrapper {
    Float32(Arc<RwLock<Vec<f32>>>),
    Uint32(Arc<RwLock<Vec<u32>>>),
}

impl BufferDataWrapper {
    pub fn unwrap_f32(self) -> Result<Arc<RwLock<Vec<f32>>>, String> {
        match self {
            BufferDataWrapper::Float32(d) => Ok(d),
            _ => Err("Invalid data type".to_string()),
        }
    }

    pub fn unwrap_u32(self) -> Result<Arc<RwLock<Vec<u32>>>, String> {
        match self {
            BufferDataWrapper::Uint32(d) => Ok(d),
            _ => Err("Invalid data type".to_string()),
        }
    }
}
