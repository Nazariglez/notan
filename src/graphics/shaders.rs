use super::{Context, GlContext};
use glow::*;
use std::collections::HashMap;

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
        Self { name, size, data_type }
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

pub struct Shader {
    vertex: ShaderKey,
    fragment: ShaderKey,
    program: ProgramKey,
    gl: GlContext,
    attributes: HashMap<String, AttributeData>,
    uniforms: HashMap<String, glow::WebUniformLocationKey>
}

impl Shader {
    pub fn new(gl: &GlContext, vertex: &str, fragment: &str, mut attributes: Vec<Attribute>, uniforms: Vec<&str>) -> Result<Self, String> {
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
                    gl.vertex_attrib_pointer_f32(location, attr.size, attr.data_type, normalize, stride, offset);

                    attrs.insert(attr.name.clone(), AttributeData {
                        attr,
                        location,
                        buffer
                    });
            }

            for uniform in uniforms {
                let u = gl.get_uniform_location(program, uniform)
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
            uniforms: uniform_list
        })
    }

    pub fn useme(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }

    //TODO macro reflecting the shaders types?
//    pub fn set_uniform<T>(&self, name: &str, value: T) {
//        // gl.uniform2f(this.uniform("u_resolution"), gl.canvas.width, gl.canvas.height);
//        // gl.uniformMatrix3fv(this.uniform("u_matrix"), false, m3.projection(gl.canvas.width, gl.canvas.height));
//    }
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

pub struct ColorBatcher {
    shader: Shader,
    width: i32,
    height: i32
}

impl ColorBatcher {
    pub fn new(gl: &GlContext, width: i32, height: i32) -> Result<Self, String> {
        Ok(Self {
            shader: create_color_shader(gl)?,
            width,
            height
        })
    }

    pub fn flush(&self) {
        self.shader.useme();
        unsafe {
            if let Some(u) = self.shader.uniforms.get("u_resolution") {
                self.shader.gl.uniform_2_f32(Some(*u), self.width as f32, self.height as f32);
                //uniforMatrix3fv u_matrix
            }
        }
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
        Attribute::new("a_position", 2, glow::FLOAT),
        Attribute::new("a_color", 4, glow::FLOAT),
    ];

    let uniforms = vec!["u_matrix"];
    Ok(Shader::new(gl, COLOR_VERTEX, COLOR_FRAGMENT, attrs, uniforms)?)
}

pub struct SpriteBatcher {
    shader: Shader,
}
