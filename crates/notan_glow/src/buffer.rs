use crate::pipeline::VertexAttributes;
use crate::pipeline::*;
use crate::to_glow::*;
use glow::*;
use notan_graphics::prelude::*;

//https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera
//https://wgld.org/d/webgl2/w009.html

pub(crate) enum Kind {
    Vertex(VertexAttributes),
    Index,
    Uniform(u32, String),
}

pub(crate) struct InnerBuffer {
    buffer: glow::Buffer,

    #[cfg(target_arch = "wasm32")]
    global_ubo: Option<Vec<u8>>, //Hack, wasm doesn't use the offset for std140

    pub block_binded: bool,

    gpu_buff_size: usize,
    draw_usage: u32,
    draw_target: u32,
    kind: Kind,
    last_pipeline: Option<u64>,
}

impl InnerBuffer {
    #[allow(unused_variables)] // ubo is used only on wasm32 builds
    pub fn new(gl: &Context, kind: Kind, dynamic: bool) -> Result<Self, String> {
        let buffer = unsafe { gl.create_buffer()? };

        #[cfg(target_arch = "wasm32")]
        let global_ubo = if matches!(kind, Kind::Uniform(_, _)) {
            let max = unsafe { gl.get_parameter_i32(glow::MAX_UNIFORM_BLOCK_SIZE) } as usize;

            Some(vec![0; max])
        } else {
            None
        };

        let draw_usage = if dynamic {
            glow::DYNAMIC_DRAW
        } else {
            glow::STATIC_DRAW
        };

        let draw_target = match kind {
            Kind::Vertex(_) => glow::ARRAY_BUFFER,
            Kind::Index => glow::ELEMENT_ARRAY_BUFFER,
            Kind::Uniform(_, _) => glow::UNIFORM_BUFFER,
        };

        Ok(InnerBuffer {
            buffer,

            #[cfg(target_arch = "wasm32")]
            global_ubo,

            block_binded: false,

            gpu_buff_size: 0,
            draw_usage,
            draw_target,
            kind,
            last_pipeline: None,
        })
    }

    #[inline]
    pub fn bind(&mut self, gl: &Context, pipeline_id: u64) {
        let pip = Some(pipeline_id);
        let pipeline_changed = pip != self.last_pipeline;
        if pipeline_changed {
            self.last_pipeline = pip;
        };

        unsafe {
            gl.bind_buffer(self.draw_target, Some(self.buffer));

            match &self.kind {
                Kind::Vertex(attrs) => {
                    if pipeline_changed {
                        attrs.enable(gl);
                    }
                }
                Kind::Uniform(slot, _) => {
                    gl.bind_buffer_base(glow::UNIFORM_BUFFER, *slot, Some(self.buffer));
                }
                _ => {}
            }
        }
    }

    #[inline]
    pub fn update(&mut self, gl: &Context, data: &[u8]) {
        let needs_alloc = self.gpu_buff_size != data.len();

        unsafe {
            // Hack to avoid layout(std140) offset problem on webgl2
            #[cfg(target_arch = "wasm32")]
            let data = if matches!(self.kind, Kind::Uniform(_, _)) {
                self.global_ubo
                    .as_mut()
                    .map(|ubo| {
                        ubo[..data.len()].copy_from_slice(data);
                        ubo.as_slice()
                    })
                    .unwrap_or_else(|| data)
            } else {
                data
            };

            if needs_alloc {
                gl.buffer_data_u8_slice(self.draw_target, data, self.draw_usage);
            } else {
                gl.buffer_sub_data_u8_slice(self.draw_target, 0, data);
            }
        }
    }

    pub fn bind_ubo_block(&mut self, gl: &Context, pipeline: &InnerPipeline) {
        self.block_binded = true;

        if let Kind::Uniform(slot, name) = &self.kind {
            unsafe {
                if let Some(index) = gl.get_uniform_block_index(pipeline.program, name) {
                    gl.uniform_block_binding(pipeline.program, index, *slot as _);
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
}
