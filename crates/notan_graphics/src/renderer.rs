use crate::buffer::*;
use crate::commands::*;

use crate::pipeline::*;
use crate::texture::*;

#[derive(Default, Clone)]
pub struct Renderer {
    commands: Vec<Commands>,
    size: (i32, i32),
    primitive: DrawPrimitive,
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            size: (width, height),
            commands: vec![Commands::Size { width, height }],
            primitive: DrawPrimitive::Triangles,
        }
    }

    pub fn begin(&mut self, options: Option<&ClearOptions>) {
        let (color, stencil, depth) = match options {
            Some(opts) => (opts.color, opts.stencil, opts.depth),
            _ => (None, None, None),
        };

        self.commands.push(Commands::Begin {
            color,
            stencil,
            depth,
        });
    }

    pub fn set_primitive(&mut self, primitive: DrawPrimitive) {
        self.primitive = primitive;
    }

    pub fn end(&mut self) {
        self.commands.push(Commands::End);
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
        self.commands.push(Commands::Size { width, height });
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

    pub fn set_viewport(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.commands.push(Commands::Viewport {
            x,
            y,
            width,
            height,
        });
    }

    pub fn set_scissors(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.commands.push(Commands::Scissors {
            x,
            y,
            width,
            height,
        });
    }

    pub fn set_pipeline(&mut self, pipeline: &Pipeline) {
        self.commands.push(Commands::Pipeline {
            id: pipeline.id(),
            options: pipeline.options.clone(),
        });
    }

    pub fn bind_buffer(&mut self, buffer: &Buffer) {
        self.commands.push(Commands::BindBuffer { id: buffer.id() });
    }

    pub fn bind_buffers(&mut self, buffers: &[&Buffer]) {
        self.commands
            .extend(buffers.iter().map(|b| Commands::BindBuffer { id: b.id() }));
    }

    pub fn draw(&mut self, offset: i32, count: i32) {
        self.commands.push(Commands::Draw {
            primitive: self.primitive,
            offset,
            count,
        })
    }

    pub fn draw_instanced(&mut self, offset: i32, count: i32, length: i32) {
        self.commands.push(Commands::DrawInstanced {
            primitive: self.primitive,
            offset,
            count,
            length,
        })
    }

    pub fn bind_texture(&mut self, location: u32, texture: &Texture) {
        self.bind_texture_slot(0, location, texture);
    }

    pub fn bind_texture_slot(&mut self, slot: u32, location: u32, texture: &Texture) {
        self.commands.push(Commands::BindTexture {
            slot,
            location,
            id: texture.id(),
        })
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn commands(&self) -> &[Commands] {
        &self.commands
    }
}
