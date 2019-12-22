use crate::context::DrawData;
use crate::font::{Font, FontManager, FontTextureData};
use crate::shader::{text_shader_from_gl_context, Shader};
use crate::texture::{texture_from_gl_context, Texture};
use crate::{BufferKey, GlContext, TextureKey};
use glow::HasContext;
use nae_core::graphics::BaseShader;
use nae_core::math::*;
use nae_core::resources::{
    BaseTexture, HorizontalAlign, Resource, TextureFilter, TextureFormat, VerticalAlign,
};

const VERTICES: usize = 3;
const VERTICE_SIZE: usize = 2;
const COLOR_VERTICE_SIZE: usize = 4;

const MAX_PER_BATCH: usize = 65000 / (VERTICES * COLOR_VERTICE_SIZE);

#[cfg(target_arch = "wasm32")]
type VaoKey = glow::WebVertexArrayKey;

#[cfg(not(target_arch = "wasm32"))]
type VaoKey = <glow::Context as HasContext>::VertexArray;

pub(crate) struct ColorBatcher {}

impl ColorBatcher {
    pub fn new(gl: &GlContext) -> Result<Self, String> {
        unimplemented!()
    }

    pub fn flush(&mut self, gl: &GlContext, data: &DrawData) {
        unimplemented!()
    }

    pub fn reset(&mut self) {
        unimplemented!()
    }

    pub fn draw(&mut self, gl: &GlContext, data: &DrawData, vertex: &[f32], color: Option<&[f32]>) {
        unimplemented!()
    }
}

pub(crate) struct SpriteBatcher {}

impl SpriteBatcher {
    pub fn new(gl: &GlContext) -> Result<Self, String> {
        unimplemented!()
    }

    pub fn flush(&mut self, gl: &GlContext, data: &DrawData) {
        unimplemented!()
    }

    pub fn reset(&mut self) {
        unimplemented!()
    }

    pub fn draw(&mut self, gl: &GlContext, data: &DrawData, vertex: &[f32], color: Option<&[f32]>) {
        unimplemented!()
    }
    pub fn draw_image(
        &mut self,
        gl: &GlContext,
        data: &DrawData,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        img: &Texture,
        source_x: f32,
        source_y: f32,
        source_width: f32,
        source_height: f32,
        color: Option<&[f32]>,
    ) {
        unimplemented!()
    }

    pub fn draw_pattern(
        &mut self,
        gl: &GlContext,
        data: &DrawData,
        x: f32,
        y: f32,
        img: &Texture,
        width: f32,
        height: f32,
        offset_x: f32,
        offset_y: f32,
        scale_x: f32,
        scale_y: f32,
        color: Option<&[f32]>,
    ) {
        unimplemented!()
    }
}

pub(crate) struct TextBatcher {
    pub font: Font,
    pub manager: FontManager<'static>,
    shader: Shader,
    vao: VaoKey,
    index: i32,
    vertex: Vec<f32>,
    vertex_color: Vec<f32>,
    vertex_tex: Vec<f32>,
    current_tex: TextureKey,
    data: Vec<FontTextureData>,
    texture: Texture,
    current_matrix: Mat3,
}

impl TextBatcher {
    pub fn new(gl: &GlContext) -> Result<Self, String> {
        let vao = create_vao(gl)?;
        let shader = text_shader_from_gl_context(gl, None)?;
        let font = Font::default();
        let manager = FontManager::new(gl)?;
        let (width, height) = manager.texture_dimensions();
        let texture = texture_from_gl_context(
            gl,
            width as _,
            height as _,
            TextureFormat::R8,
            TextureFormat::Red,
            TextureFilter::Linear,
            TextureFilter::Linear,
        )?;
        let current_tex = texture.tex().unwrap();

        Ok(Self {
            shader,
            vao,
            index: 0,
            vertex: vec![0.0; MAX_PER_BATCH * VERTICES * VERTICE_SIZE],
            vertex_color: vec![0.0; MAX_PER_BATCH * VERTICES * COLOR_VERTICE_SIZE],
            vertex_tex: vec![0.0; MAX_PER_BATCH * VERTICES * VERTICE_SIZE],
            current_tex,
            font,
            manager,
            data: vec![],
            texture,
            current_matrix: Mat3::identity(),
        })
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn set_font(&mut self, font: &Font) {
        self.font = font.clone();
    }

    fn use_shader(&self, data: &DrawData) {
        let shader = match &data.shader {
            Some(s) => s,
            _ => &self.shader,
        };
        shader.use_me();
        shader.set_uniform("u_matrix", data.projection);
        shader.set_uniform("u_texture", 0);
    }

    pub fn flush_gpu(&mut self, gl: &GlContext, data: &DrawData) {
        if self.index == 0 {
            return;
        }

        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            self.use_shader(data);

            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.current_tex));

            self.bind_buffer(gl, "a_position", &self.vertex, 0);
            self.bind_buffer(gl, "a_texcoord", &self.vertex_tex, 0);
            self.bind_buffer(gl, "a_color", &self.vertex_color, 0);
            let count = self.index * VERTICES as i32;
            gl.draw_arrays(glow::TRIANGLES, 0, count);
        }

        self.index = 0;
    }

    pub fn draw_text(
        &mut self,
        gl: &GlContext,
        data: &DrawData,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        max_width: Option<f32>,
    ) {
        if !self.font.is_loaded() {
            return;
        }

        /*TODO avoid to flush because the matrix change, store it by section and
           process it in the same draw call
        */
        if data.transform.matrix() != &self.current_matrix {
            self.flush(gl, data);
            self.current_matrix = *data.transform.matrix();
        }

        let max_width = max_width.unwrap_or(std::f32::INFINITY);

        let mut color = data.color.to_rgba();
        color[3] *= data.alpha;
        self.manager.queue(
            &self.font, x, y, text, size, color, max_width, h_align, v_align,
        );
    }

    pub fn flush(&mut self, gl: &GlContext, data: &DrawData) {
        if let Some(tex_data) = self.manager.process_queue(gl, &mut self.texture) {
            self.data = tex_data;
        }

        self.current_tex = self.texture.tex().unwrap();
        for tex_data in self.data.clone() {
            //TODO borrow issue, don't clone the vector...
            self.draw_letter(gl, data, &tex_data);
        }

        self.flush_gpu(gl, data);
    }

    fn draw_letter(&mut self, gl: &GlContext, data: &DrawData, tex_data: &FontTextureData) {
        let x = tex_data.x;
        let y = tex_data.y;
        let img_ww = self.texture.width();
        let img_hh = self.texture.height();
        let ww = tex_data.source_width;
        let hh = tex_data.source_height;

        let vertex = [
            x,
            y,
            x,
            y + hh,
            x + ww,
            y,
            x + ww,
            y,
            x,
            y + hh,
            x + ww,
            y + hh,
        ];

        let count = (vertex.len() / 6) as i32;
        let next = self.index + count;

        if next >= (MAX_PER_BATCH as i32) {
            self.flush_gpu(gl, data);
        }

        let mut offset = self.index as usize * VERTICES * VERTICE_SIZE;
        for (i, _) in vertex.iter().enumerate().step_by(2) {
            if let (Some(v1), Some(v2)) = (vertex.get(i), vertex.get(i + 1)) {
                let v = self.current_matrix * vec3(*v1, *v2, 1.0);
                self.vertex[offset] = v.x;
                self.vertex[offset + 1] = v.y;
                offset += 2;
            }
        }

        let x1 = tex_data.source_x / img_ww;
        let y1 = tex_data.source_y / img_hh;
        let x2 = (tex_data.source_x + ww) / img_ww;
        let y2 = (tex_data.source_y + hh) / img_hh;

        let mut offset = self.index as usize * VERTICES * VERTICE_SIZE;
        let vertex_tex = [x1, y1, x1, y2, x2, y1, x2, y1, x1, y2, x2, y2];
        vertex_tex.iter().for_each(|v| {
            self.vertex_tex[offset] = *v;
            offset += 1;
        });

        let mut color = vec![];
        (0..VERTICES * count as usize).for_each(|_| {
            color.extend_from_slice(&tex_data.color);
        });

        let mut offset = self.index as usize * VERTICES * COLOR_VERTICE_SIZE;
        color.iter().for_each(|c| {
            self.vertex_color[offset] = *c;
            offset += 1;
        });

        self.index += count;
    }

    fn bind_buffer(&self, gl: &GlContext, name: &str, data: &[f32], offset: usize) {
        bind_buffer(gl, self.shader.buffer(name), data, offset);
    }
}

fn create_vao(gl: &GlContext) -> Result<VaoKey, String> {
    unsafe {
        let vao = gl.create_vertex_array()?;
        gl.bind_vertex_array(Some(vao));

        Ok(vao)
    }
}

fn bind_buffer(gl: &GlContext, buffer: Option<BufferKey>, data: &[f32], _offset: usize) {
    unsafe {
        gl.bind_buffer(glow::ARRAY_BUFFER, buffer);
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vf_to_u8(&data), glow::STATIC_DRAW);
    }
}

fn vf_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}
