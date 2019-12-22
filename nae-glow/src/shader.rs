use crate::context::Context2d;
use crate::{BufferKey, GlContext, GlowValue};
use glow::HasContext;
use hashbrown::HashMap;
use nae_core::graphics::BaseShader;
use nae_core::math::Mat3;
use nae_core::BaseApp;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
type ShaderKey = glow::WebShaderKey;

#[cfg(not(target_arch = "wasm32"))]
type ShaderKey = <glow::Context as HasContext>::Shader;

#[cfg(target_arch = "wasm32")]
type ProgramKey = glow::WebProgramKey;

#[cfg(not(target_arch = "wasm32"))]
type ProgramKey = <glow::Context as HasContext>::Program;

#[cfg(target_arch = "wasm32")]
type UniformLocationKey = glow::WebUniformLocationKey;

#[cfg(not(target_arch = "wasm32"))]
type UniformLocationKey = <glow::Context as HasContext>::UniformLocation;

#[derive(Clone)]
pub struct Shader {
    inner: Rc<InnerShader>,
}

impl Shader {
    pub const COLOR_VERTEX: &'static str = include_str!("../resources/shaders/color.vert.glsl");
    pub const COLOR_FRAG: &'static str = include_str!("../resources/shaders/color.frag.glsl");

    pub const IMAGE_VERTEX: &'static str = include_str!("../resources/shaders/image.vert.glsl");
    pub const IMAGE_FRAG: &'static str = include_str!("../resources/shaders/image.frag.glsl");

    pub const TEXT_VERTEX: &'static str = include_str!("../resources/shaders/text.vert.glsl");
    pub const TEXT_FRAG: &'static str = include_str!("../resources/shaders/text.frag.glsl");

    /// Send to the GPU an uniform value
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

    /// Tell to the GPU to use this shader
    pub(crate) fn use_me(&self) {
        unsafe {
            self.inner.gl.use_program(Some(self.inner.program));
        }
    }
}

impl BaseShader for Shader {
    type Graphics = Context2d;
    type Buffer = BufferKey;
    type Attr = Attr;
    type Kind = Self;

    fn new<T: BaseApp<Graphics = Self::Graphics>>(
        app: &mut T,
        vertex: &str,
        fragment: &str,
        attributes: Vec<Self::Attr>,
    ) -> Result<Self, String> {
        shader_from_gl_context(&app.graphics().gl, vertex, fragment, attributes)
    }

    fn buffer(&self, name: &str) -> Option<Self::Buffer> {
        if let Some(attr) = self.inner.attributes.get(name) {
            return Some(attr.buffer);
        }

        None
    }

    fn from_image_fragment<T: BaseApp<Graphics = Self::Graphics>>(
        app: &mut T,
        fragment: &str,
    ) -> Result<Self, String> {
        sprite_shader_from_gl_context(&app.graphics().gl, Some(fragment))
    }

    fn from_text_fragment<T: BaseApp<Graphics = Self::Graphics>>(
        app: &mut T,
        fragment: &str,
    ) -> Result<Self, String> {
        text_shader_from_gl_context(&app.graphics().gl, Some(fragment))
    }

    fn from_color_fragment<T: BaseApp<Graphics = Self::Graphics>>(
        app: &mut T,
        fragment: &str,
    ) -> Result<Self, String> {
        color_shader_from_gl_context(&app.graphics().gl, Some(fragment))
    }

    fn is_equal(&self, shader: &Shader) -> bool {
        self.inner.program == shader.inner.program
    }
}

fn shader_from_gl_context(
    gl: &GlContext,
    vertex: &str,
    fragment: &str,
    mut attributes: Vec<Attr>,
) -> Result<Shader, String> {
    let vertex = create_shader(gl, glow::VERTEX_SHADER, vertex)?;
    let fragment = create_shader(gl, glow::FRAGMENT_SHADER, fragment)?;

    let program = create_program(gl, vertex, fragment)?;

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
            let data_type = attr.vertex_data.glow_value();
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

    Ok(Shader {
        inner: Rc::new(InnerShader {
            vertex,
            fragment,
            program,
            gl: gl.clone(),
            attributes: attrs,
            uniforms: RefCell::new(HashMap::new()),
        }),
    })
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

/// Vertex data types
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

    pub fn normalized(&self) -> bool {
        false
    }
}

impl GlowValue for VertexData {
    fn glow_value(&self) -> u32 {
        glow::FLOAT
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
    buffer: BufferKey,
}

/// Represent a shader uniform
pub trait UniformType {
    fn set_uniform_value(&self, gl: &GlContext, location: UniformLocationKey);
}

impl UniformType for i32 {
    fn set_uniform_value(&self, gl: &GlContext, location: UniformLocationKey) {
        unsafe {
            gl.uniform_1_i32(Some(location), *self);
        }
    }
}

impl UniformType for &[f32; 2] {
    fn set_uniform_value(&self, gl: &GlContext, location: UniformLocationKey) {
        unsafe {
            gl.uniform_2_f32(Some(location), self[0], self[1]);
        }
    }
}

impl UniformType for &[f32; 4] {
    fn set_uniform_value(&self, gl: &GlContext, location: UniformLocationKey) {
        unsafe {
            gl.uniform_4_f32(Some(location), self[0], self[1], self[2], self[3]);
        }
    }
}

impl UniformType for Mat3 {
    fn set_uniform_value(&self, gl: &GlContext, location: UniformLocationKey) {
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
    uniforms: RefCell<HashMap<String, UniformLocationKey>>,
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

fn m3_to_slice(m: &Mat3) -> *const [f32; 9] {
    m.as_slice().as_ptr() as *const [f32; 9]
}

pub(crate) fn sprite_shader_from_gl_context(
    gl: &GlContext,
    frag: Option<&str>,
) -> Result<Shader, String> {
    let attrs = vec![
        Attr::new("a_position", VertexData::Float2),
        Attr::new("a_color", VertexData::Float4),
        Attr::new("a_texcoord", VertexData::Float2),
    ];

    Ok(shader_from_gl_context(
        gl,
        Shader::IMAGE_VERTEX,
        frag.unwrap_or(Shader::IMAGE_FRAG),
        attrs,
    )?)
}

pub(crate) fn text_shader_from_gl_context(
    gl: &GlContext,
    frag: Option<&str>,
) -> Result<Shader, String> {
    let attrs = vec![
        Attr::new("a_position", VertexData::Float2),
        Attr::new("a_color", VertexData::Float4),
        Attr::new("a_texcoord", VertexData::Float2),
    ];
    Ok(shader_from_gl_context(
        gl,
        Shader::TEXT_VERTEX,
        frag.unwrap_or(Shader::TEXT_FRAG),
        attrs,
    )?)
}

pub(crate) fn color_shader_from_gl_context(
    gl: &GlContext,
    frag: Option<&str>,
) -> Result<Shader, String> {
    let attrs = vec![
        Attr::new("a_position", VertexData::Float2),
        Attr::new("a_color", VertexData::Float4),
    ];

    Ok(shader_from_gl_context(
        gl,
        Shader::COLOR_VERTEX,
        frag.unwrap_or(Shader::COLOR_FRAG),
        attrs,
    )?)
}
