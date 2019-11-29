use glow::*;

use crate::graphics::DrawData;
use crate::log;
use crate::math::*;
use crate::res::*;

use super::shader::*;
use super::GlContext;
use wasm_bindgen::__rt::std::alloc::handle_alloc_error;

/*TODO masking: https://stackoverflow.com/questions/46806063/how-stencil-buffer-and-masking-work
    https://jsfiddle.net/z11zhf01/1
    https://jsfiddle.net/gpkdrs93/
*/

const VERTICES: usize = 3;
const VERTICE_SIZE: usize = 2;
const COLOR_VERTICE_SIZE: usize = 4;

/*TODO check this: drawElements use u16 as indices, 65553 is the max on webgl1
    but drawArrays doesn't have this limit.
    To use drawElements without limit on webgl1 also exists the this extension: OES_element_index_uint https://developer.mozilla.org/en-US/docs/Web/API/OES_element_index_uint
    On webgl2 the limit doesn't exists, but you need to use UNSIGNED_INT as index https://webgl2fundamentals.org/webgl/lessons/webgl2-whats-new.html (i32)
    A way to do this on webgl1 is:
        - try to get the extension
        - If it fails use drawElements by default
        - fallback if indices > 65553 to drawArrays
    On webgl2 we should probably use just drawElements with i32 indices
    --
    # help https://computergraphics.stackexchange.com/questions/3637/how-to-use-32-bit-integers-for-element-indices-in-webgl-1-0
    # var canvas = document.createElement("canvas");
    var gl = canvas.getContext("webgl");
    console.log(gl.getExtension("OES_element_index_uint"));
*/
const MAX_PER_BATCH: usize = 65000 / (VERTICES * COLOR_VERTICE_SIZE);

/* TODO for work with vaos on webgl1:
    https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object
    https://www.khronos.org/registry/webgl/extensions/OES_vertex_array_object/
    https://medium.com/@david.komer/dude-wheres-my-data-vao-s-in-webgl-896631783895
    https://stackoverflow.com/a/46143967
*/

fn vf_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

pub(super) struct ColorBatcher {
    shader: Shader,
    vao: glow::WebVertexArrayKey,
    index: i32,
    vertex: Vec<f32>,
    vertex_color: Vec<f32>,
}

impl ColorBatcher {
    pub fn new(gl: &GlContext, _data: &DrawData) -> Result<Self, String> {
        let vao = create_vao(gl)?;
        let shader = create_color_shader(gl)?;
        Ok(Self {
            shader,
            vao,
            index: 0,
            vertex: vec![0.0; MAX_PER_BATCH * VERTICES * VERTICE_SIZE],
            vertex_color: vec![0.0; MAX_PER_BATCH * VERTICES * COLOR_VERTICE_SIZE],
        })
    }

    pub fn begin(&mut self) {
        self.index = 0;
    }

    pub fn flush(&mut self, gl: &GlContext, data: &DrawData) {
        if self.index == 0 {
            return;
        }

        self.use_shader(data);
        unsafe {
            gl.bind_vertex_array(Some(self.vao));

            //TODO pass the whole slice or just pass what we need to save bandwidth? (is this really that worth it?)
            let v_max = self.index as usize * VERTICES * VERTICE_SIZE;
            let vc_max = self.index as usize * VERTICES * COLOR_VERTICE_SIZE;
            self.bind_buffer(gl, "a_position", &self.vertex[0..v_max], 0);
            self.bind_buffer(gl, "a_color", &self.vertex_color[0..vc_max], 0);

            let primitives = glow::TRIANGLES;
            let offset = 0;
            let count = self.index * VERTICES as i32;
            gl.draw_arrays(primitives, offset, count);
        }

        self.index = 0;
    }

    fn use_shader(&self, data: &DrawData) {
        let shader = match &data.shader {
            Some(s) => s,
            _ => &self.shader,
        };
        shader.useme();
        shader.set_uniform("u_matrix", data.projection);
    }

    fn bind_buffer(&self, gl: &GlContext, name: &str, data: &[f32], offset: usize) {
        bind_buffer(gl, self.shader.buffer(name), data, offset);
    }

    pub fn draw(&mut self, gl: &GlContext, data: &DrawData, vertex: &[f32], color: Option<&[f32]>) {
        let count = (vertex.len() / 6) as i32; //vertex.len() / (vertices*size)
        let next = self.index + count;

        if next >= (MAX_PER_BATCH as i32) {
            self.flush(gl, data);
        }

        let mut offset = self.index as usize * VERTICES * VERTICE_SIZE;
        for (i, _) in vertex.iter().enumerate().step_by(2) {
            if let (Some(v1), Some(v2)) = (vertex.get(i), vertex.get(i + 1)) {
                let v = data.transform.matrix() * vec3(*v1, *v2, 1.0);
                self.vertex[offset] = v.x;
                self.vertex[offset + 1] = v.y;
                offset += 2;
            }
        }

        let color = match color {
            Some(c) => c.to_vec(),
            None => {
                let (r, g, b, a) = data.color.to_rgba();
                let mut color = vec![];
                (0..VERTICES * count as usize).for_each(|_| {
                    color.push(r);
                    color.push(g);
                    color.push(b);
                    color.push(a);
                });
                color
            }
        };

        let mut offset = self.index as usize * VERTICES * COLOR_VERTICE_SIZE;
        color.iter().enumerate().for_each(|(i, c)| {
            let is_alpha = (i + 1) % 4 == 0;
            self.vertex_color[offset] = if is_alpha { *c * data.alpha } else { *c };
            offset += 1;
        });

        self.index += count;
    }
}

pub(super) struct SpriteBatcher {
    shader: Shader,
    vao: glow::WebVertexArrayKey,
    index: i32,
    vertex: Vec<f32>,
    vertex_color: Vec<f32>,
    vertex_tex: Vec<f32>,
    current_tex: Option<glow::WebTextureKey>,
    texture_matrix: Mat3,
}

impl SpriteBatcher {
    pub fn new(gl: &GlContext, _data: &DrawData) -> Result<Self, String> {
        let vao = create_vao(gl)?;
        let shader = create_sprite_shader(gl)?;
        Ok(Self {
            shader,
            vao,
            index: 0,
            vertex: vec![0.0; MAX_PER_BATCH * VERTICES * VERTICE_SIZE],
            vertex_color: vec![0.0; MAX_PER_BATCH * VERTICES * COLOR_VERTICE_SIZE],
            vertex_tex: vec![0.0; MAX_PER_BATCH * VERTICES * VERTICE_SIZE],
            current_tex: None,
            texture_matrix: Mat3::identity(),
        })
    }

    fn use_shader(&self, data: &DrawData) {
        let shader = match &data.shader {
            Some(s) => s,
            _ => &self.shader,
        };
        shader.useme();
        shader.set_uniform("u_matrix", data.projection);
        shader.set_uniform("u_tex_matrix", self.texture_matrix);
        shader.set_uniform("u_texture", 0);
    }

    fn bind_buffer(&self, gl: &GlContext, name: &str, data: &[f32], offset: usize) {
        bind_buffer(gl, self.shader.buffer(name), data, offset);
    }

    pub fn flush(&mut self, gl: &GlContext, data: &DrawData) {
        if self.index == 0 {
            return;
        }

        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            self.use_shader(data);

            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, self.current_tex);

            self.bind_buffer(gl, "a_position", &self.vertex, 0);
            self.bind_buffer(gl, "a_texcoord", &self.vertex_tex, 0);
            self.bind_buffer(gl, "a_color", &self.vertex_color, 0);
            let count = self.index * VERTICES as i32;
            gl.draw_arrays(glow::TRIANGLES, 0, count);
        }

        self.index = 0;
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
        if !img.is_loaded() {
            return;
        }

        let tex = img.tex().unwrap();

        //        let tex = match img.tex() {
        //            Some(t) => t,
        //            _ => init_graphic_texture(gl, img).unwrap(),
        //        };

        let img_ww = img.width();
        let img_hh = img.height();

        let ww = if width == 0.0 { img_ww } else { width };
        let hh = if height == 0.0 { img_hh } else { height };

        let sw = if source_width == 0.0 {
            img_ww
        } else {
            source_width
        };
        let sh = if source_height == 0.0 {
            img_hh
        } else {
            source_height
        };

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

        if self.current_tex.is_none() {
            self.current_tex = Some(tex);
        }

        if let Some(t) = self.current_tex {
            if t != tex {
                self.flush(gl, data);
            } else {
                self.current_tex = Some(tex);
            }
        }

        let count = (vertex.len() / 6) as i32;
        let next = self.index + count;

        if next >= (MAX_PER_BATCH as i32) {
            self.flush(gl, data);
        }

        let mut offset = self.index as usize * VERTICES * VERTICE_SIZE;
        for (i, _) in vertex.iter().enumerate().step_by(2) {
            if let (Some(v1), Some(v2)) = (vertex.get(i), vertex.get(i + 1)) {
                let v = data.transform.matrix() * vec3(*v1, *v2, 1.0);
                self.vertex[offset] = v.x;
                self.vertex[offset + 1] = v.y;
                offset += 2;
            }
        }

        let x1 = source_x / img_ww;
        let y1 = source_y / img_hh;
        let x2 = (source_x + sw) / img_ww;
        let y2 = (source_y + sh) / img_hh;

        let mut offset = self.index as usize * VERTICES * VERTICE_SIZE;
        let vertex_tex = [x1, y1, x1, y2, x2, y1, x2, y1, x1, y2, x2, y2];
        vertex_tex.iter().for_each(|v| {
            self.vertex_tex[offset] = *v;
            offset += 1;
        });

        let color = match color {
            Some(c) => c.to_vec(),
            None => {
                let (r, g, b, a) = data.color.to_rgba();
                let mut color = vec![];
                (0..VERTICES * count as usize).for_each(|_| {
                    color.push(r);
                    color.push(g);
                    color.push(b);
                    color.push(a);
                });
                color
            }
        };

        let mut offset = self.index as usize * VERTICES * COLOR_VERTICE_SIZE;
        color.iter().enumerate().for_each(|(i, c)| {
            let is_alpha = (i + 1) % 4 == 0;
            self.vertex_color[offset] = if is_alpha { *c * data.alpha } else { *c };
            offset += 1;
        });

        self.index += count;
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
        if !img.is_loaded() {
            return;
        }

        let tex = img.tex().unwrap();

        //        let tex = match img.tex() {
        //            Some(t) => t,
        //            _ => init_graphic_texture(gl, img).unwrap(),
        //        };

        let offset_x = offset_x * scale_x;
        let offset_y = offset_y * scale_y;

        let ww = img.width() * scale_x;
        let hh = img.height() * scale_y;
        let quad_scale_x = width / ww;
        let quad_scale_y = height / hh;

        let sw = width;
        let sh = height;

        let vertex = [
            x,
            y,
            x,
            y + sh,
            x + sw,
            y,
            x + sw,
            y,
            x,
            y + sh,
            x + sw,
            y + sh,
        ];

        if self.current_tex.is_none() {
            self.current_tex = Some(tex);
        }

        if let Some(t) = self.current_tex {
            if t != tex {
                self.flush(gl, data);
            } else {
                self.current_tex = Some(tex);
            }
        }

        let count = (vertex.len() / 6) as i32;
        let next = self.index + count;

        if next >= (MAX_PER_BATCH as i32) {
            self.flush(gl, data);
        }

        let mut offset = self.index as usize * VERTICES * VERTICE_SIZE;
        for (i, _) in vertex.iter().enumerate().step_by(2) {
            if let (Some(v1), Some(v2)) = (vertex.get(i), vertex.get(i + 1)) {
                let v = data.transform.matrix() * vec3(*v1, *v2, 1.0);
                self.vertex[offset] = v.x;
                self.vertex[offset + 1] = v.y;
                offset += 2;
            }
        }

        let fract_x = quad_scale_x.fract();
        let fract_y = quad_scale_y.fract();
        let tex_offset_x = ((ww - offset_x) / ww).fract();
        let tex_offset_y = ((hh - offset_y) / hh).fract();

        let x1 = (quad_scale_x.floor() + tex_offset_x);
        let y1 = (quad_scale_y.floor() + tex_offset_y);
        let x2 = ((width + sw) / ww - fract_x + tex_offset_x);
        let y2 = ((height + sh) / hh - fract_y + tex_offset_y);

        let mut offset = self.index as usize * VERTICES * VERTICE_SIZE;
        let vertex_tex = [x1, y1, x1, y2, x2, y1, x2, y1, x1, y2, x2, y2];
        vertex_tex.iter().for_each(|v| {
            self.vertex_tex[offset] = *v;
            offset += 1;
        });

        let color = match color {
            Some(c) => c.to_vec(),
            None => {
                let (r, g, b, a) = data.color.to_rgba();
                let mut color = vec![];
                (0..VERTICES * count as usize).for_each(|_| {
                    color.push(r);
                    color.push(g);
                    color.push(b);
                    color.push(a);
                });
                color
            }
        };

        let mut offset = self.index as usize * VERTICES * COLOR_VERTICE_SIZE;
        color.iter().enumerate().for_each(|(i, c)| {
            let is_alpha = (i + 1) % 4 == 0;
            self.vertex_color[offset] = if is_alpha { *c * data.alpha } else { *c };
            offset += 1;
        });

        self.index += count;
    }
}

fn create_sprite_shader(gl: &GlContext) -> Result<Shader, String> {
    let attrs = vec![
        Attribute::new("a_position", 2, glow::FLOAT, false),
        Attribute::new("a_color", 4, glow::FLOAT, false),
        Attribute::new("a_texcoord", 2, glow::FLOAT, true),
    ];

    let uniforms = vec!["u_matrix", "u_texture", "u_tex_matrix"];
    Ok(Shader::new(
        gl,
        include_str!("./shaders/image.vert.glsl"),
        include_str!("./shaders/image.frag.glsl"),
        attrs,
        uniforms,
    )?)
}

#[derive(Debug, Clone)]
pub(crate) struct GraphicTexture {
    pub gl: GlContext,
    pub tex: glow::WebTextureKey,
}

fn create_vao(gl: &GlContext) -> Result<WebVertexArrayKey, String> {
    unsafe {
        let vao = gl.create_vertex_array()?;
        gl.bind_vertex_array(Some(vao));

        Ok(vao)
    }
}

fn create_color_shader(gl: &GlContext) -> Result<Shader, String> {
    let attrs = vec![
        Attribute::new("a_position", 2, glow::FLOAT, false),
        Attribute::new("a_color", 4, glow::FLOAT, false),
    ];

    let uniforms = vec!["u_matrix"];
    Ok(Shader::new(
        gl,
        include_str!("./shaders/color.vert.glsl"),
        include_str!("./shaders/color.frag.glsl"),
        attrs,
        uniforms,
    )?)
}

fn bind_buffer(gl: &GlContext, buffer: Option<WebBufferKey>, data: &[f32], _offset: usize) {
    unsafe {
        gl.bind_buffer(glow::ARRAY_BUFFER, buffer);
        let buff = vf_to_u8(&data);
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buff, glow::STATIC_DRAW);
    }
}

pub(super) struct TextBatcher {
    shader: Shader,
    vao: glow::WebVertexArrayKey,
    index: i32,
    vertex: Vec<f32>,
    vertex_color: Vec<f32>,
    vertex_tex: Vec<f32>,
    current_tex: glow::WebTextureKey,
    texture_matrix: Mat3,
    font: Font,
    pub(crate) manager: FontManager<'static>,
    data: Vec<FontTextureData>,
    texture: Texture,
    text_count: usize,
    current_matrix: Mat3,
    //dirty: bool
}

impl TextBatcher {
    pub fn new(gl: &GlContext, _data: &DrawData) -> Result<Self, String> {
        let vao = create_vao(gl)?;
        let shader = create_text_shader(gl)?;
        let font = Font::default();
        let manager = FontManager::new(gl)?;
        let (width, height) = manager.texture_dimensions();
        let texture = Texture::from(gl, width as _, height as _, TextureFormat::Rgba, TextureFilter::Linear, TextureFilter::Linear)?;
        let current_tex = texture.tex().unwrap();

        Ok(Self {
            shader,
            vao,
            index: 0,
            vertex: vec![0.0; MAX_PER_BATCH * VERTICES * VERTICE_SIZE],
            vertex_color: vec![0.0; MAX_PER_BATCH * VERTICES * COLOR_VERTICE_SIZE],
            vertex_tex: vec![0.0; MAX_PER_BATCH * VERTICES * VERTICE_SIZE],
            current_tex,
            texture_matrix: Mat3::identity(),
            font,
            manager,
            data: vec![],
            texture,
            text_count: 0,
            current_matrix: Mat3::identity(),
            //dirty: true,
        })
    }

    pub fn set_font(&mut self, font: &Font) {
        self.font = font.clone();
    }

    pub fn set_font_valign(&mut self, a: ()) {}

    pub fn set_font_halign(&mut self, a: ()) {}

    //pub fn draw_text_ext to use breaklines?

    fn use_shader(&self, data: &DrawData) {
        let shader = match &data.shader {
            Some(s) => s,
            _ => &self.shader,
        };
        shader.useme();
        shader.set_uniform("u_matrix", data.projection);
        shader.set_uniform("u_tex_matrix", self.texture_matrix);
        shader.set_uniform("u_texture", 0);
    }

    fn bind_buffer(&self, gl: &GlContext, name: &str, data: &[f32], offset: usize) {
        bind_buffer(gl, self.shader.buffer(name), data, offset);
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

        let color = data.color.to_rgba();
        self.manager.queue(
            &self.font,
            x,
            y,
            text,
            size,
            [color.0, color.1, color.2, color.3 * data.alpha],
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
        let tex = self.texture.tex().unwrap();
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

        //        let (r, g, b, a) = data.color.to_rgba();
        let mut color = vec![];
        (0..VERTICES * count as usize).for_each(|_| {
            color.extend_from_slice(&tex_data.color);
        });

        let mut offset = self.index as usize * VERTICES * COLOR_VERTICE_SIZE;
        color.iter().enumerate().for_each(|(i, c)| {
            let is_alpha = (i + 1) % 4 == 0;
            self.vertex_color[offset] = if is_alpha { *c * data.alpha } else { *c };
            offset += 1;
        });

        self.index += count;
    }
}

fn create_text_shader(gl: &GlContext) -> Result<Shader, String> {
    let attrs = vec![
        Attribute::new("a_position", 2, glow::FLOAT, false),
        Attribute::new("a_color", 4, glow::FLOAT, false),
        Attribute::new("a_texcoord", 2, glow::FLOAT, true),
    ];

    let uniforms = vec!["u_matrix", "u_texture", "u_tex_matrix"];
    Ok(Shader::new(
        gl,
        include_str!("./shaders/image.vert.glsl"),
        include_str!("./shaders/image.frag.glsl"),
        attrs,
        uniforms,
    )?)
}
