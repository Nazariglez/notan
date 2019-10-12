use super::{Context, GlContext};
use glow::*;

type ShaderKey = glow::WebShaderKey;
type ProgramKey = glow::WebProgramKey;

pub struct Shader {
    vertex: ShaderKey,
    fragment: ShaderKey,
    program: ProgramKey,
    gl: GlContext,
}

impl Shader {
    pub fn new(ctx: &Context, vertex: &str, fragment: &str) -> Result<Self, String> {
        let gl = ctx.gl.clone();
        let vertex = create_shader(&gl, glow::VERTEX_SHADER, vertex)?;
        let fragment = create_shader(&gl, glow::FRAGMENT_SHADER, fragment)?;
        let program = create_program(&gl, vertex, fragment)?;

        Ok(Self {
            vertex: vertex,
            fragment: fragment,
            program: program,
            gl: gl,
        })
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

fn create_program(gl: &GlContext, vertex: ShaderKey, fragment: ShaderKey) -> Result<ProgramKey, String> {
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
}

impl ColorBatcher {
    pub fn new(ctx: &Context) -> Result<Self, String> {
        Ok(Self {
            shader: create_color_shader(ctx)?
        })
    }
}

const COLOR_VERTEX:&str = r#"#version 300 es
in vec2 a_position;
in vec4 a_color;
out vec4 v_color;

uniform mat3 u_matrix;

void main() {
  v_color = a_color;
  gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
}
"#;

const COLOR_FRAGMENT:&str = r#"#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 outColor;

void main() {
    outColor = v_color;
}
"#;

fn create_color_shader(ctx: &Context) -> Result<Shader, String> {
    Ok(Shader::new(ctx, COLOR_VERTEX, COLOR_FRAGMENT)?)
}

pub struct SpriteBatcher {
    shader: Shader,
}
