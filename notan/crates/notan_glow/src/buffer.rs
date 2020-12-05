use crate::pipeline::VertexAttributes;
use crate::to_glow::*;
use glow::*;
use notan_graphics::prelude::*;
use std::rc::Rc;

//https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera
//https://wgld.org/d/webgl2/w009.html

pub(crate) struct InnerBuffer {
    buffer: glow::Buffer,
}

impl InnerBuffer {
    pub fn new(gl: &Context) -> Result<Self, String> {
        let buffer = unsafe { gl.create_buffer()? };

        Ok(InnerBuffer { buffer })
    }

    #[inline(always)]
    pub fn clean(self, gl: &Context) {
        unsafe {
            gl.delete_buffer(self.buffer);
        }
    }

    #[inline(always)]
    pub fn bind_as_ubo(&self, gl: &Context, slot: u32) {
        unsafe {
            gl.bind_buffer(glow::UNIFORM_BUFFER, Some(self.buffer));
            gl.bind_buffer_base(glow::UNIFORM_BUFFER, slot, Some(self.buffer));
        }
    }

    #[inline(always)]
    pub fn bind_as_ubo_with_data(&self, gl: &Context, slot: u32, draw: &DrawType, data: &[u8]) {
        self.bind_as_ubo(gl, slot);
        unsafe {
            //https://webgl2fundamentals.org/webgl/lessons/webgl2-whats-new.html#:~:text=A%20Uniform%20Buffer%20Object%20is,blocks%20defined%20in%20a%20shader.
            //https://stackoverflow.com/questions/44629165/bind-multiple-uniform-buffer-objects
            gl.buffer_data_u8_slice(glow::UNIFORM_BUFFER, data, draw.to_glow());
        }
    }

    #[inline(always)]
    pub fn bind_as_ebo(&self, gl: &Context) {
        unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.buffer));
        }
    }

    #[inline(always)]
    pub fn bind_as_ebo_with_data(&self, gl: &Context, draw: &DrawType, data: &[u8]) {
        self.bind_as_ebo(gl);

        unsafe {
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, data, draw.to_glow());
        }
    }

    #[inline(always)]
    pub fn bind_as_vbo(&self, gl: &Context, attrs: &Option<VertexAttributes>) {
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
        gl: &Context,
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
