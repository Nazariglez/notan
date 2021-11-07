use crate::pipeline::VertexAttributes;
use crate::pipeline::*;
use crate::to_glow::*;
use glow::*;
use notan_graphics::prelude::*;

//https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera
//https://wgld.org/d/webgl2/w009.html

pub(crate) struct InnerBuffer {
    buffer: glow::Buffer,

    #[cfg(target_arch = "wasm32")]
    global_ubo: Option<Vec<u8>>, //Hack, wasm doesn't use the offset for std140

    uniform_block_name: Option<String>,

    pub block_binded: bool,

    gpu_buff_size: usize,
}

impl InnerBuffer {
    #[allow(unused_variables)] // ubo is used only on wasm32 builds
    pub fn new(gl: &Context, ubo: bool) -> Result<Self, String> {
        let buffer = unsafe { gl.create_buffer()? };

        #[cfg(target_arch = "wasm32")]
        let global_ubo = if ubo {
            let max = unsafe { gl.get_parameter_i32(glow::MAX_UNIFORM_BLOCK_SIZE) } as usize;

            Some(vec![0; max])
        } else {
            None
        };

        Ok(InnerBuffer {
            buffer,

            #[cfg(target_arch = "wasm32")]
            global_ubo,

            uniform_block_name: None,

            block_binded: false,

            gpu_buff_size: 0,
        })
    }

    pub fn bind_block(&mut self, gl: &Context, pipeline: &InnerPipeline, slot: u32) {
        self.block_binded = true;

        if let Some(name) = &self.uniform_block_name {
            unsafe {
                if let Some(index) = gl.get_uniform_block_index(pipeline.program, name) {
                    gl.uniform_block_binding(pipeline.program, index, slot as _);
                }
            }
        }
    }

    #[inline(always)]
    pub fn clean(self, gl: &Context) {
        unsafe {
            gl.delete_buffer(self.buffer);
        }
    }

    #[inline]
    pub fn setup_as_ubo(&mut self, gl: &Context, slot: u32, name: &str) {
        self.uniform_block_name = Some(name.to_string());
        self.bind_as_ubo(gl, slot);
    }

    #[inline(always)]
    pub fn bind_as_ubo(&mut self, gl: &Context, slot: u32) {
        unsafe {
            gl.bind_buffer(glow::UNIFORM_BUFFER, Some(self.buffer));
            gl.bind_buffer_base(glow::UNIFORM_BUFFER, slot, Some(self.buffer));
        }
    }

    #[inline(always)]
    pub fn bind_as_ubo_with_data(&mut self, gl: &Context, slot: u32, draw: &DrawType, data: &[u8]) {
        self.bind_as_ubo(gl, slot);

        #[cfg(target_arch = "wasm32")]
        unsafe {
            // Hack to avoid layout(std140) offset problem on webgl2
            let ubo = self
                .global_ubo
                .as_mut()
                .map(|ubo| {
                    ubo[..data.len()].copy_from_slice(data);
                    ubo.as_slice()
                })
                .unwrap_or_else(|| data);
            // gl.buffer_data_u8_slice(glow::UNIFORM_BUFFER, ubo, draw.to_glow());

            let len = data.len();
            if self.gpu_buff_size != len {
                self.gpu_buff_size = len;
                gl.buffer_data_u8_slice(glow::UNIFORM_BUFFER, ubo, draw.to_glow());
            } else {
                gl.buffer_sub_data_u8_slice(glow::UNIFORM_BUFFER, 0, ubo);
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            //https://webgl2fundamentals.org/webgl/lessons/webgl2-whats-new.html#:~:text=A%20Uniform%20Buffer%20Object%20is,blocks%20defined%20in%20a%20shader.
            //https://stackoverflow.com/questions/44629165/bind-multiple-uniform-buffer-objects
            let len = data.len();
            if self.gpu_buff_size != len {
                self.gpu_buff_size = len;
                gl.buffer_data_u8_slice(glow::UNIFORM_BUFFER, data, draw.to_glow());
            } else {
                gl.buffer_sub_data_u8_slice(glow::UNIFORM_BUFFER, 0, data);
            }
        }
    }

    #[inline(always)]
    pub fn bind_as_ebo(&mut self, gl: &Context) {
        unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.buffer));
        }
    }

    #[inline(always)]
    pub fn bind_as_ebo_with_data(&mut self, gl: &Context, draw: &DrawType, data: &[u8]) {
        self.bind_as_ebo(gl);

        unsafe {
            let len = data.len();
            if self.gpu_buff_size != len {
                self.gpu_buff_size = len;
                gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, data, draw.to_glow());
            } else {
                gl.buffer_sub_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, 0, data);
            }
        }
    }

    #[inline]
    pub fn setup_as_vbo(&mut self, gl: &Context, attrs: VertexAttributes) {
        self.bind_as_vbo(gl);
        unsafe {
            attrs.enable(gl);
        }
    }

    #[inline(always)]
    pub fn bind_as_vbo(&mut self, gl: &Context) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
        }
    }

    #[inline(always)]
    pub fn bind_as_vbo_with_data(&mut self, gl: &Context, draw: &DrawType, data: &[u8]) {
        self.bind_as_vbo(gl);

        unsafe {
            // TODO use buffer_sub_data_u8_slice
            // TODO https://community.khronos.org/t/bufferdata-or-buffersubdata-vbo/61674/2
            let len = data.len();
            if self.gpu_buff_size != len {
                self.gpu_buff_size = len;
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, draw.to_glow());
            } else {
                gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, data);
            }
        }
    }
}
