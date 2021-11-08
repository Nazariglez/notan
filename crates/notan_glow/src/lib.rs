use crate::to_glow::ToGlow;
use glow::*;
use hashbrown::HashMap;
use notan_graphics::prelude::*;
use notan_graphics::DeviceBackend;

mod buffer;
mod pipeline;
mod render_target;
mod texture;
mod to_glow;
mod utils;

use crate::pipeline::get_inner_attrs;
use crate::texture::texture_format;
use buffer::InnerBuffer;
use pipeline::{InnerPipeline, VertexAttributes};
use render_target::InnerRenderTexture;
use texture::InnerTexture;

pub struct GlowBackend {
    gl: Context,
    buffer_count: u64,
    texture_count: u64,
    pipeline_count: u64,
    render_target_count: u64,
    size: (i32, i32),
    dpi: f32,
    pipelines: HashMap<u64, InnerPipeline>,
    buffers: HashMap<u64, InnerBuffer>,
    textures: HashMap<u64, InnerTexture>,
    render_targets: HashMap<u64, InnerRenderTexture>,
    // current_vertex_attrs: Option<VertexAttributes>,
    using_indices: bool,
    api_name: String,
    current_pipeline: u64,
    limits: Limits,

    #[cfg(target_arch = "wasm32")]
    current_uniforms: Vec<UniformLocation>,
}

impl GlowBackend {
    #[cfg(target_arch = "wasm32")]
    pub fn new(canvas: &web_sys::HtmlCanvasElement, antialias: bool) -> Result<Self, String> {
        let (gl, api) = utils::create_gl_context(canvas, antialias)?;
        Self::from(gl, &api)
    }

    #[cfg(all(
        not(target_arch = "wasm32"),
        not(target_os = "ios"),
        not(target_os = "android")
    ))]
    pub fn new<F>(loader_function: F) -> Result<Self, String>
    where
        F: FnMut(&str) -> *const std::os::raw::c_void,
    {
        let gl = unsafe { Context::from_loader_function(loader_function) };

        Self::from(gl, "opengl")
    }

    #[cfg(any(target_os = "ios", target_os = "android"))]
    pub fn new<F>(mut loader_function: F) -> Result<Self, String>
    where
        F: FnMut(&str) -> *const std::os::raw::c_void,
    {
        let gl = unsafe { Context::from_loader_function(loader_function) };

        Self::from(gl, "opengl_es")
    }

    fn from(gl: Context, api: &str) -> Result<Self, String> {
        log::info!("Using {} graphics api", api);

        let limits = unsafe {
            Limits {
                max_texture_size: gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE) as _,
            }
        };

        Ok(Self {
            pipeline_count: 0,
            buffer_count: 0,
            texture_count: 0,
            render_target_count: 0,
            gl,
            size: (0, 0),
            dpi: 1.0,
            pipelines: HashMap::new(),
            // current_vertex_attrs: None,
            buffers: HashMap::new(),
            textures: HashMap::new(),
            render_targets: HashMap::new(),
            using_indices: false,
            api_name: api.to_string(),
            current_pipeline: 0,
            limits,

            #[cfg(target_arch = "wasm32")]
            current_uniforms: vec![],
        })
    }
}

impl GlowBackend {
    #[inline(always)]
    fn clear(&self, color: &Option<Color>, depth: &Option<f32>, stencil: &Option<i32>) {
        clear(&self.gl, color, depth, stencil);
    }

    fn begin(
        &self,
        target: Option<u64>,
        color: &Option<Color>,
        depth: &Option<f32>,
        stencil: &Option<i32>,
    ) {
        let render_target = match target {
            Some(id) => self.render_targets.get(&id),
            _ => None,
        };

        let (width, height, dpi) = match render_target {
            Some(rt) => {
                rt.bind(&self.gl);
                (rt.size.0, rt.size.1, 1.0)
            }
            None => {
                unsafe {
                    self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
                }
                (self.size.0, self.size.1, self.dpi)
            }
        };

        self.viewport(0.0, 0.0, width as _, height as _, dpi);

        self.clear(color, depth, stencil);
    }

    #[inline]
    fn viewport(&self, x: f32, y: f32, width: f32, height: f32, dpi: f32) {
        let ww = width * dpi;
        let hh = height * dpi;

        unsafe {
            self.gl.viewport(x as _, y as _, ww as _, hh as _);
        }
    }

    #[inline]
    fn scissors(&self, x: f32, y: f32, width: f32, height: f32, dpi: f32) {
        let canvas_height = ((self.size.1 - (height + y) as i32) as f32 * dpi) as i32;
        let x = x * dpi;
        let width = width * dpi;
        let height = height * dpi;

        unsafe {
            self.gl.enable(glow::SCISSOR_TEST);
            self.gl
                .scissor(x as _, canvas_height, width as _, height as _);
        }
    }

    fn end(&mut self) {
        unsafe {
            self.gl.disable(glow::SCISSOR_TEST);
            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            self.gl.bind_vertex_array(None);
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }

        self.using_indices = false;
        // self.current_vertex_attrs = None;
    }

    fn clean_pipeline(&mut self, id: u64) {
        if let Some(pip) = self.pipelines.remove(&id) {
            pip.clean(&self.gl);
        }
    }

    fn set_pipeline(&mut self, id: u64, options: &PipelineOptions) {
        if let Some(pip) = self.pipelines.get(&id) {
            pip.bind(&self.gl, options);
            // self.current_vertex_attrs = Some(pip.attrs.clone());
            self.using_indices = false;
            self.current_pipeline = id;

            #[cfg(target_arch = "wasm32")]
            {
                self.current_uniforms = pip.uniform_locations.clone();
            }
        }
    }

    fn bind_buffer(
        &mut self,
        id: u64,
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
                    buffer.bind_as_vbo_with_data(&self.gl, draw, ptr)
                }
                BufferUsage::Index => {
                    self.using_indices = true;
                    let inner_data = data_wrapper.unwrap_u32().unwrap();
                    let data = inner_data.read();
                    let ptr = bytemuck::cast_slice(&data);
                    buffer.bind_as_ebo_with_data(&self.gl, draw, ptr)
                }
                BufferUsage::Uniform(slot) => {
                    if !buffer.block_binded {
                        buffer.bind_block(
                            &self.gl,
                            self.pipelines.get(&self.current_pipeline).as_ref().unwrap(),
                            *slot,
                        );
                    }

                    let inner_data = data_wrapper.unwrap_f32().unwrap();
                    let data = inner_data.read();
                    let ptr = bytemuck::cast_slice(&data);
                    buffer.bind_as_ubo_with_data(&self.gl, *slot, draw, ptr);
                }
            }
        }
    }

    fn bind_texture(&mut self, id: u64, slot: u32, location: u32) {
        if let Some(texture) = self.textures.get(&id) {
            texture.bind(&self.gl, slot, self.get_uniform_loc(&location));
        }
    }

    #[inline(always)]
    fn get_uniform_loc<'a>(&'a self, location: &'a u32) -> &'a UniformLocation {
        #[cfg(target_arch = "wasm32")]
        {
            &self.current_uniforms[*location as usize]
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            location
        }
    }

    fn clean_buffer(&mut self, id: u64) {
        if let Some(buffer) = self.buffers.remove(&id) {
            buffer.clean(&self.gl);
        }
    }

    fn clean_texture(&mut self, id: u64) {
        if let Some(texture) = self.textures.remove(&id) {
            texture.clean(&self.gl);
        }
    }

    fn clean_render_target(&mut self, id: u64) {
        if let Some(rt) = self.render_targets.remove(&id) {
            rt.clean(&self.gl);
        }
    }

    fn draw(&mut self, offset: i32, count: i32) {
        unsafe {
            if self.using_indices {
                self.gl
                    .draw_elements(glow::TRIANGLES, count, glow::UNSIGNED_INT, offset * 4);
            } else {
                self.gl.draw_arrays(glow::TRIANGLES, offset, count);
            }
        }
    }
    fn draw_instanced(&mut self, offset: i32, count: i32, length: i32) {
        unsafe {
            if self.using_indices {
                self.gl.draw_elements_instanced(
                    glow::TRIANGLES,
                    count,
                    glow::UNSIGNED_INT,
                    offset,
                    length,
                );
            } else {
                self.gl.draw_arrays_instanced(glow::TRIANGLES, offset, count, length);
            }
        }
    }
}

impl DeviceBackend for GlowBackend {
    fn api_name(&self) -> &str {
        &self.api_name
    }

    fn limits(&self) -> Limits {
        self.limits
    }

    fn create_pipeline(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<u64, String> {
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

    fn create_vertex_buffer(
        &mut self,
        attrs: &[VertexAttr],
        step_mode: VertexStepMode,
    ) -> Result<u64, String> {
        let mut inner_buffer = InnerBuffer::new(&self.gl, false)?;
        let (stride, inner_attrs) = get_inner_attrs(attrs);
        inner_buffer.setup_as_vbo(
            &self.gl,
            VertexAttributes::new(stride, inner_attrs, step_mode),
        );
        self.buffer_count += 1;
        self.buffers.insert(self.buffer_count, inner_buffer);
        Ok(self.buffer_count)
    }

    fn create_index_buffer(&mut self) -> Result<u64, String> {
        let mut inner_buffer = InnerBuffer::new(&self.gl, false)?;
        inner_buffer.bind_as_ebo(&self.gl);
        self.buffer_count += 1;
        self.buffers.insert(self.buffer_count, inner_buffer);
        Ok(self.buffer_count)
    }

    fn create_uniform_buffer(&mut self, slot: u32, name: &str) -> Result<u64, String> {
        let mut inner_buffer = InnerBuffer::new(&self.gl, true)?;
        inner_buffer.setup_as_ubo(&self.gl, slot, name);
        self.buffer_count += 1;
        self.buffers.insert(self.buffer_count, inner_buffer);
        Ok(self.buffer_count)
    }

    fn render(&mut self, commands: &[Commands], target: Option<u64>) {
        commands.iter().for_each(|cmd| {
            use Commands::*;
            // log::trace!("Render cmd: {:?}", cmd);

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
                DrawInstanced {
                    offset,
                    count,
                    length,
                } => self.draw_instanced(*offset, *count, *length),
                BindTexture { id, slot, location } => self.bind_texture(*id, *slot, *location),
                Size { width, height } => self.set_size(*width, *height),
                Viewport {
                    x,
                    y,
                    width,
                    height,
                } => self.viewport(*x, *y, *width, *height, self.dpi),
                Scissors {
                    x,
                    y,
                    width,
                    height,
                } => self.scissors(*x, *y, *width, *height, self.dpi),
            }
        });
    }

    fn clean(&mut self, to_clean: &[ResourceId]) {
        log::debug!("gpu resources to_clean {:?}", to_clean);
        to_clean.iter().for_each(|res| match &res {
            ResourceId::Pipeline(id) => self.clean_pipeline(*id),
            ResourceId::Buffer(id) => self.clean_buffer(*id),
            ResourceId::Texture(id) => self.clean_texture(*id),
            ResourceId::RenderTexture(id) => self.clean_render_target(*id),
        })
    }

    fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
    }

    fn set_dpi(&mut self, scale_factor: f64) {
        self.dpi = scale_factor as _;
    }

    fn create_texture(&mut self, info: &TextureInfo) -> Result<u64, String> {
        let inner_texture = InnerTexture::new(&self.gl, info)?;
        self.texture_count += 1;
        self.textures.insert(self.texture_count, inner_texture);
        Ok(self.texture_count)
    }

    fn create_render_texture(
        &mut self,
        texture_id: u64,
        info: &TextureInfo,
    ) -> Result<u64, String> {
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

    fn update_texture(&mut self, texture: u64, opts: &TextureUpdate) -> Result<(), String> {
        match self.textures.get(&texture) {
            Some(texture) => {
                unsafe {
                    self.gl
                        .bind_texture(glow::TEXTURE_2D, Some(texture.texture));
                    self.gl.tex_sub_image_2d(
                        glow::TEXTURE_2D,
                        0,
                        opts.x_offset,
                        opts.y_offset,
                        opts.width,
                        opts.height,
                        texture_format(&opts.format), // 3d texture needs another value?
                        glow::UNSIGNED_BYTE,          // todo UNSIGNED SHORT FOR DEPTH (3d) TEXTURES
                        PixelUnpackData::Slice(opts.bytes),
                    );
                    // todo unbind texture?
                    Ok(())
                }
            }
            _ => Err("Invalid texture id".to_string()),
        }
    }

    fn read_pixels(
        &mut self,
        texture: u64,
        bytes: &mut [u8],
        opts: &TextureRead,
    ) -> Result<(), String> {
        match self.textures.get(&texture) {
            Some(texture) => unsafe {
                let fbo = self.gl.create_framebuffer()?;
                self.gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
                self.gl.framebuffer_texture_2d(
                    glow::FRAMEBUFFER,
                    glow::COLOR_ATTACHMENT0,
                    glow::TEXTURE_2D,
                    Some(texture.texture),
                    0,
                );

                let status = self.gl.check_framebuffer_status(glow::FRAMEBUFFER);
                let can_read = status == glow::FRAMEBUFFER_COMPLETE;

                let clean = || {
                    self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
                    self.gl.delete_framebuffer(fbo);
                };

                if can_read {
                    self.gl.read_pixels(
                        opts.x_offset,
                        opts.y_offset,
                        opts.width,
                        opts.height,
                        texture_format(&opts.format),
                        glow::UNSIGNED_BYTE,
                        glow::PixelPackData::Slice(bytes),
                    );
                    clean();
                    Ok(())
                } else {
                    clean();
                    Err("Framebuffer incomplete...".to_string())
                }
            },
            None => Err("Invalid texture id".to_string()),
        }
    }
}

#[inline]
pub(crate) fn clear(
    gl: &Context,
    color: &Option<Color>,
    depth: &Option<f32>,
    stencil: &Option<i32>,
) {
    let mut mask = 0;
    unsafe {
        if let Some(color) = color {
            mask |= glow::COLOR_BUFFER_BIT;
            gl.clear_color(color.r, color.g, color.b, color.a);
        }

        if let Some(depth) = *depth {
            mask |= glow::DEPTH_BUFFER_BIT;
            gl.enable(glow::DEPTH_TEST);
            gl.depth_mask(true);
            gl.clear_depth_f32(depth);
        }

        if let Some(stencil) = *stencil {
            mask |= glow::STENCIL_BUFFER_BIT;
            gl.enable(glow::STENCIL_TEST);
            gl.stencil_mask(0xff);
            gl.clear_stencil(stencil);
        }

        if mask != 0 {
            gl.clear(mask);
        }
    }
}
