use super::GlContext;
use crate::app::App;

use crate::math::*;
use glow::*;
use hashbrown::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

type BufferKey = glow::WebBufferKey;
type ShaderKey = glow::WebShaderKey;
type ProgramKey = glow::WebProgramKey;

/// Vertex data types
#[derive(Debug, Clone)]
pub enum VertexData {
    Float1,
    Float2,
    Float3,
    Float4,
}

impl VertexData {
    pub fn size(&self) -> i32 {
        use VertexData::*;
        match self {
            Float1 => 1,
            Float2 => 2,
            Float3 => 3,
            Float4 => 4,
        }
    }

    pub fn typ(&self) -> u32 {
        glow::FLOAT
    }

    pub fn normalized(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct Attr {
    name: String,
    vertex_data: VertexData,
}

impl Attr {
    pub fn new(name: &str, data_type: VertexData) -> Self {
        Self {
            name: name.to_string(),
            vertex_data: data_type,
        }
    }
}

#[derive(Clone)]
struct AttributeData {
    attr: Attr,
    location: u32,
    buffer: glow::WebBufferKey,
}

/// Represent a shader uniform
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

impl UniformType for &[f32; 2] {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_2_f32(Some(location), self[0], self[1]);
        }
    }
}

impl UniformType for &[f32; 4] {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_4_f32(Some(location), self[0], self[1], self[2], self[3]);
        }
    }
}

impl UniformType for Mat3 {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_matrix_3_f32_slice(Some(location), false, &*m3_to_slice(self));
        }
    }
}

struct InnerShader {
    gl: GlContext,
    vertex: ShaderKey,
    fragment: ShaderKey,
    program: ProgramKey,
    attributes: HashMap<String, AttributeData>,
    uniforms: RefCell<HashMap<String, glow::WebUniformLocationKey>>,
}

impl Drop for InnerShader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_shader(self.vertex);
            self.gl.delete_shader(self.fragment);
            self.gl.delete_program(self.program);
            self.attributes.iter().for_each(|(_, attr)| {
                self.gl.delete_buffer(attr.buffer);
            });
        }
    }
}

/// A shader is a program that runs on thr GPU
#[derive(Clone)]
pub struct Shader {
    inner: Rc<InnerShader>,
}

impl Shader {
    pub const COLOR_VERTEX: &'static str = include_str!("./shaders/color.vert.glsl");
    pub const COLOR_FRAG: &'static str = include_str!("./shaders/color.frag.glsl");

    pub const IMAGE_VERTEX: &'static str = include_str!("./shaders/image.vert.glsl");
    pub const IMAGE_FRAG: &'static str = include_str!("./shaders/image.frag.glsl");

    pub const TEXT_VERTEX: &'static str = include_str!("./shaders/text.vert.glsl");
    pub const TEXT_FRAG: &'static str = include_str!("./shaders/text.frag.glsl");

    /// Create a new shader program from source
    pub fn new(
        app: &App,
        vertex: &str,
        fragment: &str,
        attributes: Vec<Attr>,
    ) -> Result<Self, String> {
        Self::new_from_context(&app.graphics.gl, vertex, fragment, attributes)
    }

    pub(crate) fn new_from_context(
        gl: &GlContext,
        vertex: &str,
        fragment: &str,
        mut attributes: Vec<Attr>,
    ) -> Result<Self, String> {
        let gl = gl.clone();
        let vertex = create_shader(&gl, glow::VERTEX_SHADER, vertex)?;
        let fragment = create_shader(&gl, glow::FRAGMENT_SHADER, fragment)?;

        let program = create_program(&gl, vertex, fragment)?;

        let mut attrs = HashMap::new();
        unsafe {
            while let Some(attr) = attributes.pop() {
                let location = gl.get_attrib_location(program, &attr.name) as u32;
                let buffer = gl.create_buffer()?;
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
                gl.enable_vertex_attrib_array(location);

                let stride = 0;
                let offset = 0;
                let size = attr.vertex_data.size();
                let data_type = attr.vertex_data.typ();
                let normalized = attr.vertex_data.normalized();
                gl.vertex_attrib_pointer_f32(location, size, data_type, normalized, stride, offset);

                attrs.insert(
                    attr.name.clone(),
                    AttributeData {
                        attr,
                        location,
                        buffer,
                    },
                );
            }
        }

        Ok(Self {
            inner: Rc::new(InnerShader {
                vertex,
                fragment,
                program,
                gl,
                attributes: attrs,
                uniforms: RefCell::new(HashMap::new()),
            }),
        })
    }

    /// Tell to the GPU to use this shader
    pub(crate) fn use_me(&self) {
        unsafe {
            self.inner.gl.use_program(Some(self.inner.program));
        }
    }

    /// Send to the GPU a uniform value
    pub fn set_uniform<T: UniformType>(&self, name: &str, value: T) -> Result<(), String> {
        let mut uniforms = self.inner.uniforms.borrow_mut();
        if let Some(location) = uniforms.get(name) {
            value.set_uniform_value(&self.inner.gl, *location);
        } else {
            let location = unsafe {
                self.inner
                    .gl
                    .get_uniform_location(self.inner.program, name)
                    .ok_or(format!("Invalid uniform name: {}", name))?
            };
            value.set_uniform_value(&self.inner.gl, location);
            uniforms.insert(name.to_string(), location);
        }
        Ok(())
    }

    /// Returns an attribute buffer
    pub fn buffer(&self, name: &str) -> Option<WebBufferKey> {
        if let Some(attr) = self.inner.attributes.get(name) {
            return Some(attr.buffer);
        }

        None
    }

    /// Default image vertex shader with custom fragment
    pub fn from_image_fragment(app: &App, fragment: &str) -> Result<Self, String> {
        create_sprite_shader(&app.graphics.gl, Some(fragment))
    }

    /// Default text vertex shader with custom fragment
    pub fn from_text_fragment(app: &App, fragment: &str) -> Result<Self, String> {
        create_text_shader(&app.graphics.gl, Some(fragment))
    }

    /// Default color vertex shader with custom fragment
    pub fn from_color_fragment(app: &App, fragment: &str) -> Result<Self, String> {
        create_color_shader(&app.graphics.gl, Some(fragment))
    }

    /// Check if the shader program is equal to another shader (clone)
    pub fn is_equal(&self, shader: &Shader) -> bool {
        self.inner.program == shader.inner.program
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

fn m3_to_slice(m: &glm::Mat3) -> *const [f32; 9] {
    m.as_slice().as_ptr() as *const [f32; 9]
}

pub(crate) fn create_text_shader(gl: &GlContext, frag: Option<&str>) -> Result<Shader, String> {
    let attrs = vec![
        Attr::new("a_position", VertexData::Float2),
        Attr::new("a_color", VertexData::Float4),
        Attr::new("a_texcoord", VertexData::Float2),
    ];
    Ok(Shader::new_from_context(
        gl,
        Shader::TEXT_VERTEX,
        frag.unwrap_or(Shader::TEXT_FRAG),
        attrs,
    )?)
}

pub(crate) fn create_color_shader(gl: &GlContext, frag: Option<&str>) -> Result<Shader, String> {
    let attrs = vec![
        Attr::new("a_position", VertexData::Float2),
        Attr::new("a_color", VertexData::Float4),
    ];

    Ok(Shader::new_from_context(
        gl,
        Shader::COLOR_VERTEX,
        frag.unwrap_or(Shader::COLOR_FRAG),
        attrs,
    )?)
}

pub(crate) fn create_sprite_shader(gl: &GlContext, frag: Option<&str>) -> Result<Shader, String> {
    let attrs = vec![
        Attr::new("a_position", VertexData::Float2),
        Attr::new("a_color", VertexData::Float4),
        Attr::new("a_texcoord", VertexData::Float2),
    ];

    Ok(Shader::new_from_context(
        gl,
        Shader::IMAGE_VERTEX,
        frag.unwrap_or(Shader::IMAGE_FRAG),
        attrs,
    )?)
}
