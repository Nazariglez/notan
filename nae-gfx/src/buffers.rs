use crate::shader::BufferKey;
use crate::{GlContext, GlowValue, Graphics, Pipeline, VertexAttr};
use glow::HasContext;
use nae_core::{BaseGfx, BaseIndexBuffer, BasePipeline, BaseVertexBuffer, DrawUsage};
use std::rc::Rc;

pub struct IndexBuffer {
    inner: Rc<InnerBuffer>,
    usage: DrawUsage,
}

impl IndexBuffer {
    pub fn new(gfx: &Graphics, usage: DrawUsage) -> Result<Self, String> {
        unsafe {
            let gl = gfx.gl.clone();
            let buffer = gl.create_buffer()?;
            // gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer));

            let inner = Rc::new(InnerBuffer { buffer, gl });

            Ok(Self { inner, usage })
        }
    }
}

impl BaseIndexBuffer for IndexBuffer {
    type Graphics = Graphics;

    fn bind(&self, gfx: &mut Graphics, indices: &[u32]) {
        let gl = &gfx.gl;

        unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.inner.buffer));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                vfi_to_u8(&indices),
                self.usage.glow_value(),
            );
        }
    }
}

struct InnerBuffer {
    gl: GlContext,
    buffer: BufferKey,
}

impl Drop for InnerBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.buffer);
        }
    }
}

pub struct VertexBuffer {
    inner: Rc<InnerBuffer>,
    usage: DrawUsage,
}

impl VertexBuffer {
    pub fn new(graphics: &Graphics, usage: DrawUsage) -> Result<Self, String> {
        unsafe {
            let gl = graphics.gl.clone();
            let buffer = gl.create_buffer()?;
            let inner = Rc::new(InnerBuffer { buffer, gl });

            Ok(VertexBuffer { inner, usage })
        }
    }
}

impl BaseVertexBuffer for VertexBuffer {
    type Graphics = Graphics;

    fn bind(
        &self,
        gfx: &mut Graphics,
        pipeline: &<Self::Graphics as BaseGfx>::Pipeline,
        data: &[f32],
    ) {
        unsafe {
            gfx.gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.inner.buffer));
            let stride = pipeline.stride() as i32;

            pipeline.attrs.iter().for_each(|attr| {
                gfx.gl.enable_vertex_attrib_array(attr.location);
                gfx.gl.vertex_attrib_pointer_f32(
                    attr.location,
                    attr.size,
                    attr.data_type,
                    attr.normalized,
                    stride,
                    attr.offset,
                );
            });

            gfx.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                vf_to_u8(data),
                self.usage.glow_value(),
            );
        }
    }
}

fn vf_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

fn vfi_to_u8(v: &[u32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}
