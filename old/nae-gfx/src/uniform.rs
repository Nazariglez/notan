use crate::Graphics;
use glow::HasContext;
use nae_core::BaseGfx;

#[cfg(target_arch = "wasm32")]
pub type Uniform = web_sys::WebGlUniformLocation;

#[cfg(not(target_arch = "wasm32"))]
pub type Uniform = <glow::Context as HasContext>::UniformLocation;

pub trait UniformValue {
    type Graphics: BaseGfx;
    fn bind_uniform(&self, gfx: &Self::Graphics, location: Uniform);
}

impl UniformValue for i32 {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: Uniform) {
        unsafe {
            graphics.gl.uniform_1_i32(Some(location), *self);
        }
    }
}

impl UniformValue for f32 {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: Uniform) {
        unsafe {
            graphics.gl.uniform_1_f32(Some(location), *self);
        }
    }
}

impl UniformValue for [f32; 2] {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: Uniform) {
        unsafe {
            graphics.gl.uniform_2_f32(Some(location), self[0], self[1]);
        }
    }
}

impl UniformValue for [f32; 3] {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: Uniform) {
        unsafe {
            graphics
                .gl
                .uniform_3_f32(Some(location), self[0], self[1], self[2]);
        }
    }
}

impl UniformValue for [f32; 4] {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: Uniform) {
        unsafe {
            graphics
                .gl
                .uniform_4_f32(Some(location), self[0], self[1], self[2], self[3]);
        }
    }
}

impl UniformValue for [f32; 16] {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: Uniform) {
        let matrix = self.as_ptr() as *const [f32; 16];
        unsafe {
            graphics
                .gl
                .uniform_matrix_4_f32_slice(Some(location), false, &*matrix);
        }
    }
}
