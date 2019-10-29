use super::{Context, GlContext};
use crate::graphics::DrawData;
use crate::{glm, log, Color};
use glow::*;
use std::collections::HashMap;
use std::rc::Rc;

type ShaderKey = glow::WebShaderKey;
type ProgramKey = glow::WebProgramKey;

pub struct Attribute {
    name: String,
    size: i32,
    data_type: u32,
    normalize: bool,
}

impl Attribute {
    pub fn new(name: &str, size: i32, data_type: u32, normalize: bool) -> Self {
        let name = name.to_string();
        Self {
            name,
            size,
            data_type,
            normalize,
        }
    }
}

struct AttributeData {
    attr: Attribute,
    location: u32,
    buffer: glow::WebBufferKey,
}

fn projection(w: f32, h: f32) -> glm::Mat3 {
    glm::mat3(2.0 / w, 0.0, -1.0, 0.0, -2.0 / h, 1.0, 0.0, 0.0, 1.0)
}

pub trait UniformType {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey);
}

impl UniformType for i32 {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_1_i32(Some(location), *self);
        }
    }
}

impl UniformType for (f32, f32) {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_2_f32(Some(location), self.0, self.1);
        }
    }
}

impl UniformType for glm::Mat3 {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_matrix_3_f32_slice(Some(location), false, &*m3_to_slice(self));
        }
    }
}

#[inline]
fn m3_to_slice(m: &glm::Mat3) -> *const [f32; 9] {
    m.as_slice().as_ptr() as *const [f32; 9]
}

pub struct Shader {
    vertex: ShaderKey,
    fragment: ShaderKey,
    program: ProgramKey,
    gl: GlContext,
    attributes: HashMap<String, AttributeData>,
    uniforms: HashMap<String, glow::WebUniformLocationKey>,
}

impl Shader {
    pub fn new(
        gl: &GlContext,
        vertex: &str,
        fragment: &str,
        mut attributes: Vec<Attribute>,
        uniforms: Vec<&str>,
    ) -> Result<Self, String> {
        let gl = gl.clone();
        let vertex = create_shader(&gl, glow::VERTEX_SHADER, vertex)?;
        let fragment = create_shader(&gl, glow::FRAGMENT_SHADER, fragment)?;

        let program = create_program(&gl, vertex, fragment)?;

        let mut attrs = HashMap::new();
        let mut uniform_list = HashMap::new();
        unsafe {
            while let Some(attr) = attributes.pop() {
                let location = gl.get_attrib_location(program, &attr.name) as u32;
                let buffer = gl.create_buffer()?;
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
                gl.enable_vertex_attrib_array(location);

                let stride = 0;
                let offset = 0;
                gl.vertex_attrib_pointer_f32(
                    location,
                    attr.size,
                    attr.data_type,
                    attr.normalize,
                    stride,
                    offset,
                );

                attrs.insert(
                    attr.name.clone(),
                    AttributeData {
                        attr,
                        location,
                        buffer,
                    },
                );
            }

            for uniform in uniforms {
                let u = gl
                    .get_uniform_location(program, uniform)
                    .ok_or(format!("Invalid uniform name: {}", uniform))?;

                uniform_list.insert(uniform.to_string(), u);
            }
        }

        Ok(Self {
            vertex: vertex,
            fragment: fragment,
            program: program,
            gl: gl,
            attributes: attrs,
            uniforms: uniform_list,
        })
    }

    pub fn useme(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }

    pub fn set_uniform<T: UniformType>(&self, name: &str, value: T) {
        if let Some(u) = self.uniforms.get(name) {
            value.set_uniform_value(&self.gl, *u);
        }
    }

    pub fn buffer(&self, name: &str) -> Option<WebBufferKey> {
        if let Some(attr) = self.attributes.get(name) {
            return Some(attr.buffer);
        }

        None
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_shader(self.vertex);
            self.gl.delete_shader(self.fragment);
            self.gl.delete_program(self.program);
        }
    }
}

fn create_shader(gl: &GlContext, typ: u32, source: &str) -> Result<ShaderKey, String> {
    unsafe {
        let shader = gl.create_shader(typ)?;
        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        let success = gl.get_shader_compile_status(shader);
        if success {
            return Ok(shader);
        }

        let err = gl.get_shader_info_log(shader);
        gl.delete_shader(shader);
        Err(err)
    }
}

fn create_program(
    gl: &GlContext,
    vertex: ShaderKey,
    fragment: ShaderKey,
) -> Result<ProgramKey, String> {
    unsafe {
        let program = gl.create_program()?;
        gl.attach_shader(program, vertex);
        gl.attach_shader(program, fragment);
        gl.link_program(program);

        let success = gl.get_program_link_status(program);
        if success {
            return Ok(program);
        }

        let err = gl.get_program_info_log(program);
        gl.delete_program(program);
        Err(err)
    }
}

const VERTICES: usize = 3;
const VERTICE_SIZE: usize = 2;
const COLOR_VERTICE_SIZE: usize = 4;
const MAX_PER_BATCH: usize = 65000 / (VERTICES * VERTICE_SIZE);

fn vf_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

pub struct ColorBatcher {
    shader: Shader,
    vao: glow::WebVertexArrayKey,
    index: i32,
    vertex: Vec<f32>,
    vertex_color: Vec<f32>,
}

impl ColorBatcher {
    pub fn new(gl: &GlContext, data: &DrawData) -> Result<Self, String> {
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

    fn bind_buffer(&self, gl: &GlContext, name: &str, data: &[f32], _offset: usize) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, self.shader.buffer(name));
            let buff = vf_to_u8(&data);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buff, glow::STATIC_DRAW);
        }
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
                let v = data.transform.matrix() * glm::vec3(*v1, *v2, 1.0);
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

fn create_vao(gl: &GlContext) -> Result<WebVertexArrayKey, String> {
    unsafe {
        let vao = gl.create_vertex_array()?;
        gl.bind_vertex_array(Some(vao));

        Ok(vao)
    }
}

const COLOR_VERTEX: &str = r#"#version 300 es
in vec2 a_position;
in vec4 a_color;
out vec4 v_color;

uniform mat3 u_matrix;

void main() {
  v_color = a_color;
  gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
}
"#;

const COLOR_FRAGMENT: &str = r#"#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 outColor;

void main() {
    outColor = v_color;
}
"#;

fn create_color_shader(gl: &GlContext) -> Result<Shader, String> {
    let attrs = vec![
        Attribute::new("a_position", 2, glow::FLOAT, false),
        Attribute::new("a_color", 4, glow::FLOAT, false),
    ];

    let uniforms = vec!["u_matrix"];
    Ok(Shader::new(
        gl,
        COLOR_VERTEX,
        COLOR_FRAGMENT,
        attrs,
        uniforms,
    )?)
}

pub struct SpriteBatcher {
    shader: Shader,
    vao: glow::WebVertexArrayKey,
    index: i32,
    vertex: Vec<f32>,
    vertex_color: Vec<f32>,
}

impl SpriteBatcher {
    pub fn new(gl: &GlContext, data: &DrawData) -> Result<Self, String> {
        let vao = create_vao(gl)?;
        let shader = create_sprite_shader(gl)?;
        Ok(Self {
            shader,
            vao,
            index: 0,
            vertex: vec![0.0; MAX_PER_BATCH * VERTICES * VERTICE_SIZE],
            vertex_color: vec![0.0; MAX_PER_BATCH * VERTICES * COLOR_VERTICE_SIZE],
        })
    }

    fn use_shader(&self, data: &DrawData) {
        let shader = match &data.shader {
            Some(s) => s,
            _ => &self.shader
        };
        shader.useme();
        shader.set_uniform("u_matrix", data.projection);
        shader.set_uniform("u_texture", 0);
    }

    fn bind_buffer(&self, gl: &GlContext, name: &str, data: &[f32], _offset: usize) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, self.shader.buffer(name));
            let buff = vf_to_u8(&data);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buff, glow::STATIC_DRAW);
        }
    }

    pub fn draw_image(&mut self, gl: &GlContext, data: &DrawData, x:f32, y:f32, img: &mut Texture) {
        if !img.is_loaded() { return; }
        if img.tex.is_none() {
            let tex = create_gl_texture(gl, &img.inner.as_ref().unwrap()).unwrap();
            img.tex = Some(tex);
        }

        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            self.use_shader(data);

            gl.active_texture(glow::TEXTURE0);
            let tex_data = img.tex.as_ref().unwrap();
//            log(&format!("{:?}", tex_data));
            gl.bind_texture(glow::TEXTURE_2D, Some(tex_data.tex));

            let ww = tex_data.width as f32;
            let hh = tex_data.height as f32;

            self.bind_buffer(gl, "a_position", &[
                x, y,
                x, y+hh,
                x+ww, y,
                x+ww, y,
                x, y+hh,
                x+ww, y+hh
            ], 0);


            self.bind_buffer(gl, "a_texcoord", &[
                0.0, 0.0,
                0.0, 1.0,
                1.0, 0.0,
                1.0, 0.0,
                0.0, 1.0,
                1.0, 1.0
            ], 0);


            self.bind_buffer(gl, "a_color", &[
                1.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0,
            ], 0);

            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}

const SPRITE_VERTEX: &str = r#"#version 300 es
in vec2 a_position;
in vec4 a_color;
out vec4 v_color;

in vec2 a_texcoord;
out vec2 v_texcoord;

uniform mat3 u_matrix;

void main() {
  v_color = a_color;
  v_texcoord = a_texcoord;
  gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
}
"#;

const SPRITE_FRAGMENT: &str = r#"#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 outColor;

in vec2 v_texcoord;
uniform sampler2D u_texture;

void main() {
    outColor = texture(u_texture, v_texcoord) * v_color;
}
"#;

fn create_sprite_shader(gl: &GlContext) -> Result<Shader, String> {
    //https://webgl2fundamentals.org/webgl/lessons/webgl-2d-drawimage.html
    let attrs = vec![
        Attribute::new("a_position", 2, glow::FLOAT, false),
        Attribute::new("a_color", 4, glow::FLOAT, false),
        Attribute::new("a_texcoord", 2, glow::FLOAT, true)
    ];

    let uniforms = vec!["u_matrix", "u_texture"];
    Ok(Shader::new(
        gl,
        SPRITE_VERTEX,
        SPRITE_FRAGMENT,
        attrs,
        uniforms,
    )?)
}

#[derive(Debug)]
struct TextureData {
    tex: glow::WebTextureKey,
    width: i32,
    height: i32,
    raw: Vec<u8>,
}

fn create_gl_texture(gl: &GlContext, data: &[u8]) -> Result<TextureData, String> {
    unsafe {
        let tex = gl.create_texture()?;
        gl.bind_texture(glow::TEXTURE_2D, Some(tex));

        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);

        let img_data = image::load_from_memory(data)
            .map_err(|e| e.to_string())?
            .to_rgba();

        let width = img_data.width() as i32;
        let height = img_data.height() as i32;
        let vec_data = img_data.to_vec();

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width,
            height,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(&vec_data)
        );
//
//        gl.tex_image_2d(
//            glow::TEXTURE_2D,
//            0,
//            glow::RGBA as i32,
//            32,
//            32,
//            0,
//            glow::RGBA,
//            glow::UNSIGNED_BYTE,
//            Some(&[
////                255, 255, 255, 255,
////                255, 82, 15, 255,
////                158, 255, 15, 255,
////                158, 82, 255, 255,
////                158, 82, 255, 255,
////                158, 255, 15, 255,
////                255, 82, 15, 255,
////                158, 255, 15, 255,
////                158, 82, 15, 255,
////                255, 82, 15, 255,
////                158, 255, 15, 255,
////                158, 82, 255, 255,
////                158, 82, 255, 255,
////                158, 255, 15, 255,
////                255, 82, 15, 255,
////                158, 255, 15, 255,
//                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 82, 15, 255, 158, 82, 15, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 82, 15, 255, 158, 82, 15, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 82, 15, 255, 158, 82, 15, 255, 255, 171, 18, 255, 255, 208, 17, 255, 158, 82, 15, 255, 158, 82, 15, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 82, 15, 255, 158, 82, 15, 255, 255, 208, 17, 255, 255, 171, 18, 255, 158, 82, 15, 255, 158, 82, 15, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 82, 15, 255, 255, 171, 18, 255, 255, 208, 17, 255, 255, 247, 255, 255, 255, 247, 255, 255, 255, 171, 18, 255, 255, 252, 91, 255, 158, 82, 15, 255, 0, 0, 0, 0, 158, 82, 15, 255, 255, 252, 91, 255, 255, 171, 18, 255, 255, 247, 255, 255, 255, 247, 255, 255, 255, 208, 17, 255, 255, 171, 18, 255, 158, 82, 15, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 82, 15, 255, 255, 208, 17, 255, 255, 247, 255, 255, 255, 171, 18, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 247, 255, 255, 255, 247, 255, 255, 255, 252, 91, 255, 158, 82, 15, 255, 255, 252, 91, 255, 255, 247, 255, 255, 255, 247, 255, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 171, 18, 255, 255, 247, 255, 255, 255, 208, 17, 255, 158, 82, 15, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 171, 18, 255, 190, 16, 8, 255, 255, 171, 18, 255, 255, 36, 36, 255, 255, 171, 18, 255, 255, 208, 17, 255, 255, 36, 36, 255, 255, 247, 255, 255, 255, 252, 91, 255, 255, 247, 255, 255, 255, 36, 36, 255, 255, 252, 91, 255, 255, 171, 18, 255, 255, 36, 36, 255, 255, 171, 18, 255, 190, 16, 8, 255, 255, 171, 18, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 82, 15, 255, 255, 208, 17, 255, 190, 16, 8, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 171, 18, 255, 190, 16, 8, 255, 190, 16, 8, 255, 255, 171, 18, 255, 255, 252, 91, 255, 255, 36, 36, 255, 255, 252, 91, 255, 255, 208, 17, 255, 190, 16, 8, 255, 190, 16, 8, 255, 255, 171, 18, 255, 255, 36, 36, 255, 255, 36, 36, 255, 190, 16, 8, 255, 255, 208, 17, 255, 158, 82, 15, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 171, 18, 255, 190, 16, 8, 255, 255, 171, 18, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 171, 18, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 171, 18, 255, 190, 16, 8, 255, 255, 171, 18, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 171, 18, 255, 190, 16, 8, 255, 190, 16, 8, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 208, 17, 255, 255, 252, 91, 255, 255, 36, 36, 255, 255, 247, 255, 255, 255, 36, 36, 255, 255, 252, 91, 255, 255, 208, 17, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 36, 36, 255, 190, 16, 8, 255, 190, 16, 8, 255, 255, 171, 18, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 82, 15, 255, 255, 208, 17, 255, 190, 16, 8, 255, 190, 16, 8, 255, 255, 171, 18, 255, 255, 208, 17, 255, 190, 16, 8, 255, 255, 36, 36, 255, 255, 252, 91, 255, 255, 36, 36, 255, 255, 252, 91, 255, 255, 36, 36, 255, 190, 16, 8, 255, 255, 208, 17, 255, 255, 208, 17, 255, 190, 16, 8, 255, 190, 16, 8, 255, 255, 208, 17, 255, 158, 82, 15, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 171, 18, 255, 190, 16, 8, 255, 255, 171, 18, 255, 190, 16, 8, 255, 255, 171, 18, 255, 190, 16, 8, 255, 190, 16, 8, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 36, 36, 255, 190, 16, 8, 255, 190, 16, 8, 255, 255, 171, 18, 255, 190, 16, 8, 255, 255, 171, 18, 255, 190, 16, 8, 255, 255, 171, 18, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 171, 18, 255, 190, 16, 8, 255, 255, 36, 36, 255, 255, 36, 36, 255, 255, 171, 18, 255, 255, 36, 36, 255, 190, 16, 8, 255, 255, 36, 36, 255, 190, 16, 8, 255, 255, 36, 36, 255, 255, 171, 18, 255, 255, 36, 36, 255, 255, 36, 36, 255, 190, 16, 8, 255, 255, 171, 18, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 171, 18, 255, 190, 16, 8, 255, 190, 16, 8, 255, 255, 36, 36, 255, 190, 16, 8, 255, 255, 252, 91, 255, 190, 16, 8, 255, 255, 252, 91, 255, 190, 16, 8, 255, 255, 36, 36, 255, 190, 16, 8, 255, 190, 16, 8, 255, 255, 171, 18, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 208, 17, 255, 150, 41, 21, 255, 190, 16, 8, 255, 255, 171, 18, 255, 255, 208, 17, 255, 255, 252, 91, 255, 255, 208, 17, 255, 255, 171, 18, 255, 190, 16, 8, 255, 150, 41, 21, 255, 255, 208, 17, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 208, 17, 255, 150, 41, 21, 255, 190, 16, 8, 255, 255, 171, 18, 255, 255, 208, 17, 255, 255, 171, 18, 255, 190, 16, 8, 255, 150, 41, 21, 255, 255, 208, 17, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 171, 18, 255, 150, 41, 21, 255, 190, 16, 8, 255, 255, 171, 18, 255, 190, 16, 8, 255, 150, 41, 21, 255, 255, 171, 18, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 252, 91, 255, 150, 41, 21, 255, 190, 16, 8, 255, 150, 41, 21, 255, 255, 252, 91, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 252, 91, 255, 150, 41, 21, 255, 255, 252, 91, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 255, 252, 91, 255, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 73, 29, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
//            ])
//        );

        log(&format!("{} wh:{},{}", vec_data.len(), width, height));
        log(&format!("{:?}", vec_data));
        //TODO mipmaps? gl.generate_mipmap(glow::TEXTURE_2D);
        gl.bind_texture(glow::TEXTURE_2D, None);
        Ok(TextureData {
            tex: tex,
            width: width,
            height: height,
            raw: vec_data,
        })
    }
}


use futures::{future, Future, Async};
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Promise, ArrayBuffer, Uint8Array};
use web_sys::{XmlHttpRequest, XmlHttpRequestEventTarget, XmlHttpRequestResponseType};

pub fn load_file(path: &str) -> impl Future<Item = Vec<u8>, Error = String> {
    future::result(xhr_req(path))
        .and_then(|xhr| {
            // Code ported from quicksilver https://github.com/ryanisaacg/quicksilver/blob/master/src/file.rs#L30
            future::poll_fn(move || {
                let status = xhr.status().unwrap() / 100;
                let done = xhr.ready_state() == 4;
                match (status, done) {
                    (2, true) => Ok(Async::Ready(xhr.response().unwrap())),
                    (2, _) => Ok(Async::NotReady),
                    (0, _) => Ok(Async::NotReady),
                    _ => Err(format!("Error loading file.")) //todo add path to know which file is failing. (borrow error here?)
                }
            }).and_then(|data| {
                log(&format!("DATA: {:?}", data));
                let js_arr: Uint8Array = Uint8Array::new(&data);
                let mut arr = vec![];
                let mut cb = |a, b, c| {
                    arr.push(a);
                };
                js_arr.for_each(&mut cb);
                Ok(arr)
//                let mut arr = vec![];
//                js_arr.copy_to(arr.as_mut_slice());
//                Ok(arr)
            })
        })
}

fn xhr_req(url: &str) -> Result<XmlHttpRequest, String> {
    let mut xhr = XmlHttpRequest::new()
        .map_err(|e| e.as_string().unwrap())?;

    xhr.set_response_type(XmlHttpRequestResponseType::Arraybuffer);
    xhr.open("GET", url)
        .map_err(|e| e.as_string().unwrap())?;
    xhr.send()
        .map_err(|e| e.as_string().unwrap())?;

    Ok(xhr)
}

/// Represent an Asset
pub trait Asset {
    /// Create an instance from a file path
    fn new(path: &str) -> Self;

    /// Get the future stored on charge of load the file
    fn future(&mut self) -> &mut Box<Future<Item = Vec<u8>, Error = String>>;

    /// Dispatched when the asset buffer is loaded, used to process and store the data ready to be consumed
    fn set_asset(&mut self, asset: Vec<u8>);

    /// Check if the asset is loaded on memory
    fn is_loaded(&self) -> bool;

    /// Execute the future in charge of loading the file
    fn try_load(&mut self) -> Result<(), String> {
        if self.is_loaded() { return Ok(()) }

        self.future()
            .poll()
            .map(|s|{
                Ok(match s {
                    Async::Ready(buff) => {
                        self.set_asset(buff);
                    },
                    _ => {}
                })
            })?
    }
}

pub struct Texture {
    pub inner: Option<Vec<u8>>,
    fut: Box<Future<Item = Vec<u8>, Error = String>>,
    tex: Option<TextureData>,
}

impl Asset for Texture {
    fn new(file: &str) -> Self {
        Self {
            inner: None,
            fut: Box::new(load_file(file)),
            tex: None,
        }
    }

    fn future(&mut self) -> &mut Box<Future<Item = Vec<u8>, Error = String>> {
        &mut self.fut
    }

    fn set_asset(&mut self, asset: Vec<u8>) {
        self.inner = Some(asset)
    }

    fn is_loaded(&self) -> bool {
        self.inner.is_some()
    }
}
