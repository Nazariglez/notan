use crate::to_glow::*;
use glow::*;
use notan_graphics::prelude::*;
use std::rc::Rc;

pub(crate) struct InnerBuffer {
    draw: u32,
    usage: u32,
    buffer: glow::Buffer,
}

impl InnerBuffer {
    pub fn new(gl: &Rc<Context>, usage: BufferUsage, draw: DrawType) -> Result<Self, String> {
        let buffer = unsafe { gl.create_buffer()? };
        let draw = draw.to_glow();
        let usage = usage.to_glow();

        Ok(InnerBuffer {
            draw,
            usage,
            buffer,
        })
    }

    pub fn clean(self, gl: &Rc<Context>) {
        unsafe {
            gl.delete_buffer(self.buffer);
        }
    }

    pub fn bind(&mut self, gl: &Rc<Context>, data: &[u8]) {
        unsafe {
            match self.usage {
                glow::ELEMENT_ARRAY_BUFFER => bind_index_buffer(gl, &self, data),
                glow::ARRAY_BUFFER => {} //TODO
                _ => {}
            }
        }
    }
}

/*unsafe fn bind_vertex_buffer(gl: &Rc<Context>, data: &[u8], attrs....) {

}*/

#[inline(always)]
unsafe fn bind_index_buffer(gl: &Rc<Context>, buffer: &InnerBuffer, data: &[u8]) {
    gl.bind_buffer(buffer.usage, Some(buffer.buffer));
    gl.buffer_data_u8_slice(buffer.usage, data, buffer.draw);
}
