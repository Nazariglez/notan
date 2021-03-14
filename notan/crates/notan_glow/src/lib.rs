use glow::*;
use hashbrown::HashMap;
use notan_graphics::prelude::*;
use notan_graphics::{Device, DeviceBackend};
use std::rc::Rc;

mod buffer;
mod pipeline;
mod render_target;
mod texture;
mod to_glow;
mod utils;

use buffer::InnerBuffer;
use pipeline::{InnerPipeline, VertexAttributes};
use render_target::InnerRenderTexture;
use texture::InnerTexture;

pub struct GlowBackend {
    gl: Context,
    buffer_count: i32,
    texture_count: i32,
    pipeline_count: i32,
    render_target_count: i32,
    size: (i32, i32),
    pipelines: HashMap<i32, InnerPipeline>,
    buffers: HashMap<i32, InnerBuffer>,
    textures: HashMap<i32, InnerTexture>,
    render_targets: HashMap<i32, InnerRenderTexture>,
    current_vertex_attrs: Option<VertexAttributes>,
    gl_index_type: u32,
    using_indices: bool,
    api_name: String,

    #[cfg(target_arch = "wasm32")]
    current_uniforms: Vec<UniformLocation>,
}

impl GlowBackend {
    #[cfg(target_arch = "wasm32")]
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<Self, String> {
        let (gl, api) = utils::create_gl_context(canvas)?;
        Self::from(gl, &api)
    }

    fn from(gl: Context, api: &str) -> Result<Self, String> {
        notan_log::info!("Using {} graphics api", api);

        let gl_index_type = match api {
            "webgl" => glow::UNSIGNED_SHORT,
            _ => glow::UNSIGNED_INT,
        };

        Ok(Self {
            pipeline_count: 0,
            buffer_count: 0,
            texture_count: 0,
            render_target_count: 0,
            gl,
            size: (0, 0),
            pipelines: HashMap::new(),
            current_vertex_attrs: None,
            buffers: HashMap::new(),
            textures: HashMap::new(),
            render_targets: HashMap::new(),
            gl_index_type,
            using_indices: false,
            api_name: api.to_string(),

            #[cfg(target_arch = "wasm32")]
            current_uniforms: vec![],
        })
    }
}

impl GlowBackend {
    #[inline(always)]
    fn clear(&self, color: &Option<Color>, depth: &Option<f32>, stencil: &Option<i32>) {
        let mut mask = 0;
        unsafe {
            if let Some(color) = color {
                mask |= glow::COLOR_BUFFER_BIT;
                self.gl.clear_color(color.r, color.g, color.b, color.a);
            }

            if let Some(depth) = *depth {
                mask |= glow::DEPTH_BUFFER_BIT;
                self.gl.enable(glow::DEPTH_TEST);
                self.gl.depth_mask(true);
                self.gl.clear_depth_f32(depth);
            }

            if let Some(stencil) = *stencil {
                mask |= glow::STENCIL_BUFFER_BIT;
                self.gl.enable(glow::STENCIL_TEST);
                self.gl.stencil_mask(0xff);
                self.gl.clear_stencil(stencil);
            }

            if mask != 0 {
                self.gl.clear(mask);
            }
        }
    }

    fn begin(
        &self,
        target: Option<i32>,
        color: &Option<Color>,
        depth: &Option<f32>,
        stencil: &Option<i32>,
    ) {
        let render_target = match target {
            Some(id) => self.render_targets.get(&id),
            _ => None,
        };

        unsafe {
            let (width, height) = match render_target {
                Some(rt) => {
                    rt.bind(&self.gl);
                    rt.size
                }
                None => {
                    self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
                    self.size
                }
            };

            self.gl.viewport(0, 0, width, height);
        }

        self.clear(color, depth, stencil);
    }

    fn end(&mut self) {
        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            self.gl.bind_vertex_array(None);
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }

        self.using_indices = false;
        self.current_vertex_attrs = None;
        //TODO pipeline clean and stats
    }

    fn clean_pipeline(&mut self, id: i32) {
        if let Some(pip) = self.pipelines.remove(&id) {
            pip.clean(&self.gl);
        }
    }

    fn set_pipeline(&mut self, id: i32, options: &PipelineOptions) {
        if let Some(pip) = self.pipelines.get(&id) {
            pip.bind(&self.gl, options);
            self.current_vertex_attrs = Some(pip.attrs.clone());
            self.using_indices = false;

            #[cfg(target_arch = "wasm32")]
            {
                self.current_uniforms = pip.uniform_locations.clone();
            }
        }
    }

    fn bind_buffer(
        &mut self,
        id: i32,
        data_wrapper: BufferDataWrapper,
        usage: &BufferUsage,
        draw: &DrawType,
    ) {
        if let Some(buffer) = self.buffers.get_mut(&id) {
            match usage {
                BufferUsage::Vertex => {
                    let inner_data = data_wrapper.unwrap_f32().unwrap();
                    let data = inner_data.read();
                    let ptr = bytemuck::cast_slice(&data);
                    buffer.bind_as_vbo_with_data(&self.gl, &self.current_vertex_attrs, draw, ptr)
                }
                BufferUsage::Index => {
                    self.using_indices = true;
                    let inner_data = data_wrapper.unwrap_u32().unwrap();
                    let data = inner_data.read();
                    let ptr = bytemuck::cast_slice(&data);
                    buffer.bind_as_ebo_with_data(&self.gl, draw, ptr)
                }
                BufferUsage::Uniform(slot) => {
                    let inner_data = data_wrapper.unwrap_f32().unwrap();
                    let data = inner_data.read();
                    let ptr = bytemuck::cast_slice(&data);
                    buffer.bind_as_ubo_with_data(&self.gl, *slot, draw, ptr);
                }
            }
        }
    }

    fn bind_texture(&mut self, id: i32, slot: u32, location: u32) {
        if let Some(texture) = self.textures.get(&id) {
            texture.bind(&self.gl, slot, self.get_uniform_loc(&location));
        }
    }

    #[inline(always)]
    fn get_uniform_loc(&self, location: &u32) -> &UniformLocation {
        #[cfg(target_arch = "wasm32")]
        {
            &self.current_uniforms[*location as usize]
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            location
        }
    }

    fn clean_buffer(&mut self, id: i32) {
        if let Some(buffer) = self.buffers.remove(&id) {
            buffer.clean(&self.gl);
        }
    }

    fn clean_texture(&mut self, id: i32) {
        if let Some(texture) = self.textures.remove(&id) {
            texture.clean(&self.gl);
        }
    }

    fn clean_render_target(&mut self, id: i32) {
        if let Some(rt) = self.render_targets.remove(&id) {
            rt.clean(&self.gl);
        }
    }

    fn draw(&mut self, offset: i32, count: i32) {
        unsafe {
            if self.using_indices {
                self.gl
                    .draw_elements(glow::TRIANGLES, count, self.gl_index_type, offset * 4);
            } else {
                self.gl.draw_arrays(glow::TRIANGLES, offset, count);
            }
        }
    }
}

impl DeviceBackend for GlowBackend {
    fn api_name(&self) -> &str {
        &self.api_name
    }

    fn create_pipeline(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<i32, String> {
        let vertex_source = std::str::from_utf8(vertex_source).map_err(|e| e.to_string())?;
        let fragment_source = std::str::from_utf8(fragment_source).map_err(|e| e.to_string())?;

        let inner_pipeline =
            InnerPipeline::new(&self.gl, vertex_source, fragment_source, vertex_attrs)?;
        inner_pipeline.bind(&self.gl, &options);

        self.pipeline_count += 1;
        self.pipelines.insert(self.pipeline_count, inner_pipeline);

        self.set_pipeline(self.pipeline_count, &options);
        Ok(self.pipeline_count)
    }

    fn create_vertex_buffer(&mut self) -> Result<i32, String> {
        let inner_buffer = InnerBuffer::new(&self.gl, false)?;
        inner_buffer.bind_as_vbo(&self.gl, &self.current_vertex_attrs);
        self.buffer_count += 1;
        self.buffers.insert(self.buffer_count, inner_buffer);
        Ok(self.buffer_count)
    }

    fn create_index_buffer(&mut self) -> Result<i32, String> {
        let inner_buffer = InnerBuffer::new(&self.gl, false)?;
        inner_buffer.bind_as_ebo(&self.gl);
        self.buffer_count += 1;
        self.buffers.insert(self.buffer_count, inner_buffer);
        Ok(self.buffer_count)
    }

    fn create_uniform_buffer(&mut self, slot: u32) -> Result<i32, String> {
        let inner_buffer = InnerBuffer::new(&self.gl, true)?;
        inner_buffer.bind_as_ubo(&self.gl, slot);
        self.buffer_count += 1;
        self.buffers.insert(self.buffer_count, inner_buffer);
        Ok(self.buffer_count)
    }

    fn create_render_texture(
        &mut self,
        texture_id: i32,
        info: &TextureInfo,
    ) -> Result<i32, String> {
        let texture = self.textures.get(&texture_id).ok_or(format!(
            "Error creating render target: texture id '{}' not found.",
            texture_id
        ))?;

        let inner_rt = InnerRenderTexture::new(&self.gl, texture, info)?;
        self.render_target_count += 1;
        self.render_targets
            .insert(self.render_target_count, inner_rt);
        Ok(self.render_target_count)
    }

    fn create_texture(&mut self, info: &TextureInfo) -> Result<i32, String> {
        let inner_texture = InnerTexture::new(&self.gl, info)?;
        //TODO bind?
        self.texture_count += 1;
        self.textures.insert(self.texture_count, inner_texture);
        Ok(self.texture_count)
    }

    fn render(&mut self, commands: &[Commands], target: Option<i32>) {
        commands.iter().for_each(|cmd| {
            use Commands::*;
            // notan_log::info!("{:?}", cmd);

            match cmd {
                Begin {
                    color,
                    depth,
                    stencil,
                } => self.begin(target, color, depth, stencil),
                End => self.end(),
                Pipeline { id, options } => self.set_pipeline(*id, options),
                BindBuffer {
                    id,
                    data,
                    usage,
                    draw,
                } => self.bind_buffer(*id, data.clone(), usage, draw),
                Draw { offset, count } => self.draw(*offset, *count),
                BindTexture { id, slot, location } => self.bind_texture(*id, *slot, *location),
                _ => {}
            }
        });
    }

    fn clean(&mut self, to_clean: &[ResourceId]) {
        notan_log::info!("{:?}", to_clean);
        to_clean.iter().for_each(|res| match &res {
            ResourceId::Pipeline(id) => self.clean_pipeline(*id),
            ResourceId::Buffer(id) => self.clean_buffer(*id),
            ResourceId::Texture(id) => self.clean_texture(*id),
            ResourceId::RenderTexture(id) => self.clean_render_target(*id),
            _ => {}
        })
    }

    fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
    }
}
