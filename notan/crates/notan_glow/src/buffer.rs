use crate::pipeline::VertexAttributes;
use crate::to_glow::*;
use glow::*;
use notan_graphics::prelude::*;
use std::rc::Rc;

pub(crate) struct InnerBuffer {
    buffer: glow::Buffer,
}

impl InnerBuffer {
    pub fn new(gl: &Rc<Context>, usage: BufferUsage, draw: DrawType) -> Result<Self, String> {
        let buffer = unsafe { gl.create_buffer()? };

        Ok(InnerBuffer { buffer })
    }

    pub fn clean(self, gl: &Rc<Context>) {
        unsafe {
            gl.delete_buffer(self.buffer);
        }
    }

    pub fn bind_as_ebo(&self, gl: &Rc<Context>, draw: &DrawType, data: &[u8]) {
        unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.buffer));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, data, draw.to_glow());
        }
    }

    pub fn bind_as_vbo(
        &self,
        gl: &Rc<Context>,
        draw: &DrawType,
        attrs: &VertexAttributes,
        data: &[u8],
    ) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
            attrs.enable(gl);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, draw.to_glow());
        }
    }
}
