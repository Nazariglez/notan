use super::{Context, GlContext};
use crate::graphics::DrawData;
use crate::{glm, log};
use glow::*;
use std::collections::HashMap;
use std::rc::Rc;

type ShaderKey = glow::WebShaderKey;
type ProgramKey = glow::WebProgramKey;

pub struct Attribute {
    name: String,
    size: i32,
    data_type: u32,
}

impl Attribute {
    pub fn new(name: &str, size: i32, data_type: u32) -> Self {
        let name = name.to_string();
        Self {
            name,
            size,
            data_type,
        }
    }
}

struct AttributeData {
    attr: Attribute,
    location: u32,
    buffer: glow::WebBufferKey,
}

//#[shader_data]
//struct ShaderData {
//    u_color: [f32; 4],
//}
//
//pub struct Shader<ShaderData> {}
//
//let s = Shader<ShaderData>::new();
//s.u_color = [1.0, 2.0, 3.0, 4.0];

//https://github.com/17cupsofcoffee/tetra/pull/75/files

fn mult_vec(m: &glm::Mat3, x: f32, y: f32) -> glm::Vec2 {
    /*  0, 1, 2,
        3, 4, 5,
        6, 7, 8 */

//    let w = m[6] * x + m[7] * y + m[8] * 1.0;
//    let xx = (m[0] * x + m[1] * y + m[2] * 1.0) / w;
//    let yy = (m[3] * x + m[4] * y + m[5] * 1.0) / w;


    /*  0, 3, 6,
        1, 4, 7,
        2, 5, 8 */

    let w = m[2] * x + m[5] * y + m[8] * 1.0;
    let xx = (m[0] * x + m[3] * y + m[6] * 1.0) / w;
    let yy = (m[1] * x + m[4] * y + m[7] * 1.0) / w;


    glm::vec2(xx, yy)
}

fn projection(w: f32, h: f32) -> glm::Mat3 {
    glm::mat3(2.0 / w, 0.0, -1.0, 0.0, -2.0 / h, 1.0, 0.0, 0.0, 1.0)
}

trait UniformType {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey);
}

impl UniformType for i32 {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_1_i32(Some(location), *self);
        }
    }
}

impl UniformType for (f32, f32) /*[f32; 2]*/ {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_2_f32(Some(location), self.0, self.1);
        }
    }
}

impl UniformType for crate::glm::Mat3 {
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

                let normalize = false;
                let stride = 0;
                let offset = 0;
                gl.vertex_attrib_pointer_f32(
                    location,
                    attr.size,
                    attr.data_type,
                    normalize,
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
const MAX_PER_BATCH: usize = 2;

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
        log("pre flush");
        if self.index == 0 {
            return;
        }

        log(&format!("Flush, {} {}", self.index, self.vertex.len()));

        self.use_shader(data);
        unsafe {
            gl.bind_vertex_array(Some(self.vao));

            self.bind_buffer(gl, "a_position", &self.vertex, 0);
            self.bind_buffer(gl, "a_color", &self.vertex_color, 0);
            //            gl.bind_buffer(glow::ARRAY_BUFFER, self.shader.buffer("a_position"));
            //            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vf_to_u8(&self.vertex), glow::STATIC_DRAW);
            //
            //            gl.bind_buffer(glow::ARRAY_BUFFER, self.shader.buffer("a_color"));
            //            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vf_to_u8(&self.vertex_color), glow::STATIC_DRAW);

            let primitives = glow::TRIANGLES;
            let offset = 0;
            let count = self.index * 3;
            //            log(&format!("{} pos: {:?}", count, self.vertex));
            //            log(&format!("color: {:?}", self.vertex_color));
            gl.draw_arrays(primitives, offset, count);
        }

        self.index = 0;
    }

    fn use_shader(&self, data: &DrawData) {
        let shader = match &data.shader {
            Some(s) => s,
            _ => &self.shader
        };
        shader.useme();
        shader.set_uniform("u_matrix", projection(data.width as f32, data.height as f32));
    }

    fn bind_buffer(&self, gl: &GlContext, name: &str, data: &[f32], offset: usize) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, self.shader.buffer(name));
            let buff = vf_to_u8(&data);
            log(&format!("{:?}", buff));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buff, glow::STATIC_DRAW);
//            let len = std::mem::size_of_val(data) / std::mem::size_of::<u8>();
//            let data = std::slice::from_raw_parts(data.as_ptr() as *const u8, len);
//            let offset = offset * std::mem::size_of::<f32>();
//            log(&format!("{} {} {:?} ", len, offset, data));
//            gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, offset as i32, data);
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
                let vec = mult_vec(data.transform.matrix(), *v1, *v2);
                self.vertex[offset] = vec.x;
                offset += 1;
                self.vertex[offset] = vec.y;
                offset += 1;
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
//  gl_Position = vec4((vec3(a_position, 1)).xy, 0, 1);
//  gl_Position = vec4(a_position.x, a_position.y, 0, 1);
}
"#;

const COLOR_FRAGMENT: &str = r#"#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 outColor;

void main() {
    outColor = v_color;
//    outColor = vec4(1, 0, 0, 1);
}
"#;

fn create_color_shader(gl: &GlContext) -> Result<Shader, String> {
    let attrs = vec![
        Attribute::new("a_position", 2, glow::FLOAT),
        Attribute::new("a_color", 4, glow::FLOAT),
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
}
