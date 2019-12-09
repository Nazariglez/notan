use super::GlContext;
use crate::app::App;
use crate::math::*;
use glow::*;
use hashbrown::HashMap;

//TODO cross compile https://crates.io/crates/shaderc - https://crates.io/crates/spirv_cross

type BufferKey = glow::WebBufferKey;
type ShaderKey = glow::WebShaderKey;
type ProgramKey = glow::WebProgramKey;

//https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer
pub enum VertexData {
    Float1,
    Float2,
    Float3,
    Float4,
}

impl VertexData {
    pub fn size(&self) -> usize {
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
}

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

/// Represent a shader attribute
pub struct Attribute {
    name: String,
    size: i32,
    data_type: u32,
    normalize: bool,
}

impl Attribute {
    /// Create a new attribute
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

impl UniformType for (f32, f32) {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_2_f32(Some(location), self.0, self.1);
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

/// A shader is a program that runs on thr GPU
pub struct Shader {
    vertex: ShaderKey,
    fragment: ShaderKey,
    program: ProgramKey,
    gl: GlContext,
    attributes: HashMap<String, AttributeData>,
    uniforms: HashMap<String, glow::WebUniformLocationKey>,
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
        unimplemented!()
        //        Self::new_from_context(&app.graphics.gl, vertex, fragment, attributes, uniforms)
    }

    pub(crate) fn new_from_context(
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
            vertex,
            fragment,
            program,
            gl,
            attributes: attrs,
            uniforms: uniform_list,
        })
    }

    /// Tell to the GPU to use this shader
    pub fn useme(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }

    /// Send to the GPU a uniform value
    pub fn set_uniform<T: UniformType>(&self, name: &str, value: T) {
        if let Some(u) = self.uniforms.get(name) {
            value.set_uniform_value(&self.gl, *u);
        }
    }

    /// Returns an attribute buffer
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

fn m3_to_slice(m: &glm::Mat3) -> *const [f32; 9] {
    m.as_slice().as_ptr() as *const [f32; 9]
}
