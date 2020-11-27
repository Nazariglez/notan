use crate::pipeline::VertexAttributes;
use crate::to_glow::*;
use glow::*;
use notan_graphics::prelude::*;
use std::rc::Rc;

pub(crate) struct InnerBuffer {
    buffer: glow::Buffer,
}

impl InnerBuffer {
    pub fn new(gl: &Rc<Context>) -> Result<Self, String> {
        let buffer = unsafe { gl.create_buffer()? };

        Ok(InnerBuffer { buffer })
    }

    #[inline(always)]
    pub fn clean(self, gl: &Rc<Context>) {
        unsafe {
            gl.delete_buffer(self.buffer);
        }
    }

    #[inline(always)]
    pub fn bind_as_ebo(&self, gl: &Rc<Context>) {
        unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.buffer));
        }
    }

    #[inline(always)]
    pub fn bind_as_ebo_with_data(&self, gl: &Rc<Context>, draw: &DrawType, data: &[u8]) {
        self.bind_as_ebo(gl);

        unsafe {
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, data, draw.to_glow());
        }
    }

    #[inline(always)]
    pub fn bind_as_vbo(&self, gl: &Rc<Context>, attrs: &Option<VertexAttributes>) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));

            if let Some(vertex_attrs) = attrs {
                vertex_attrs.enable(gl);
            }
        }
    }

    #[inline(always)]
    pub fn bind_as_vbo_with_data(
        &self,
        gl: &Rc<Context>,
        attrs: &Option<VertexAttributes>,
        draw: &DrawType,
        data: &[u8],
    ) {
        self.bind_as_vbo(gl, attrs);

        unsafe {
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, draw.to_glow());
        }
    }
}
