// use crate::context::Context2d;
// use crate::{BufferKey, GlContext, GlowValue};
use crate::{GlContext, Graphics};
use glow::HasContext;
use hashbrown::HashMap;
use nae_core::graphics::BaseShader;
use nae_core::math::Mat3;
use nae_core::{BaseApp, BaseContext2d, BaseSystem};
use spirv_cross::{glsl, spirv, ErrorCode};
use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;
use std::{io, slice};

type ShaderKey = <glow::Context as HasContext>::Shader;
type ProgramKey = <glow::Context as HasContext>::Program;
type UniformLocationKey = <glow::Context as HasContext>::UniformLocation;
pub type BufferKey = <glow::Context as HasContext>::Buffer;

#[derive(Clone, Copy)]
pub enum Driver {
    WebGl,
    WebGl2,
    OpenGl3_3,
    OpenGlEs2_0,
}

impl Driver {
    fn to_glsl_version(&self) -> Option<glsl::Version> {
        use glsl::Version::*;
        use Driver::*;

        Some(match self {
            WebGl => V1_00Es,
            WebGl2 => V3_00Es,
            OpenGl3_3 => V3_30,
            OpenGlEs2_0 => V1_00Es,
            _ => return None,
        })
    }
}

#[derive(Clone)]
pub struct Shader {
    pub(crate) program: ProgramKey,
    pub(crate) gl: GlContext,
    // inner: Rc<InnerShader>,
}

impl Shader {
    pub fn new(graphics: &Graphics, vertex: &[u8], fragment: &[u8]) -> Result<Self, String> {
        let vert_spv = read_spirv(Cursor::new(&vertex[..])).map_err(|e| e.to_string())?;
        let frag_spv = read_spirv(Cursor::new(&fragment[..])).map_err(|e| e.to_string())?;

        let vert = compile_spirv_to_glsl(&vert_spv, graphics.driver)?;
        let frag = compile_spirv_to_glsl(&frag_spv, graphics.driver)?;

        //FIXME layout(binding = 0) is not allowed for

        println!("vert: {}", vert);
        println!("frag: {}", frag);

        Self::from_source(graphics, &vert, &frag)
    }

    pub fn from_source(graphics: &Graphics, vertex: &str, fragment: &str) -> Result<Self, String> {
        let gl = graphics.gl.clone();
        let vertex = create_shader(&gl, glow::VERTEX_SHADER, vertex)?;
        let fragment = create_shader(&gl, glow::FRAGMENT_SHADER, fragment)?;

        let program = create_program(&gl, vertex, fragment)?;

        let vao = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            vao
        };

        Ok(Self { program, gl })
    }
}

fn compile_spirv_to_glsl(source: &[u32], driver: Driver) -> Result<String, String> {
    let module = spirv::Module::from_words(source);
    let mut ast = spirv::Ast::<glsl::Target>::parse(&module).map_err(error_code_to_string)?;
    let res = ast.get_shader_resources().map_err(|e| format!("{:?}", e))?;

    ast.set_compiler_options(&glsl::CompilerOptions {
        version: driver.to_glsl_version().ok_or("Invalid driver version")?,
        vertex: glsl::CompilerVertexOptions::default(),
    })
    .map_err(error_code_to_string)?;

    println!(
        "{:?} {:?} {:?}",
        res.sampled_images, res.uniform_buffers, res.storage_buffers
    );
    //TODO get spirv for vulkan as input and output glsl for opengl
    //https://community.arm.com/developer/tools-software/graphics/b/blog/posts/spirv-cross-working-with-spir-v-in-your-app
    //https://github.com/gfx-rs/gfx/blob/d6c68cb9a940a6639a42651304c6d49b5399aca7/src/backend/gl/src/device.rs#L238
    fix_ast_for_gl(&mut ast, &res.sampled_images);
    fix_ast_for_gl(&mut ast, &res.uniform_buffers);
    fix_ast_for_gl(&mut ast, &res.storage_buffers);

    ast.compile().map_err(error_code_to_string)
}

fn fix_ast_for_gl(ast: &mut spirv::Ast<glsl::Target>, resources: &[spirv::Resource]) {
    for r in resources {
        println!("{:?}", r);
        ast.unset_decoration(r.id, spirv::Decoration::Binding)
            .unwrap();
    }
}

fn error_code_to_string(err: ErrorCode) -> String {
    match err {
        ErrorCode::Unhandled => String::from("Unhandled"),
        ErrorCode::CompilationError(e) => e,
    }
}

// FUNCTION TAKED FROM GFX
pub fn read_spirv<R: io::Read + io::Seek>(mut x: R) -> io::Result<Vec<u32>> {
    let size = x.seek(io::SeekFrom::End(0))?;
    if size % 4 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input length not divisible by 4",
        ));
    }
    if size > usize::max_value() as u64 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "input too long"));
    }
    let words = (size / 4) as usize;
    let mut result = Vec::<u32>::with_capacity(words);
    x.seek(io::SeekFrom::Start(0))?;
    unsafe {
        // Writing all bytes through a pointer with less strict alignment when our type has no
        // invalid bitpatterns is safe.
        x.read_exact(slice::from_raw_parts_mut(
            result.as_mut_ptr() as *mut u8,
            words * 4,
        ))?;
        result.set_len(words);
    }
    const MAGIC_NUMBER: u32 = 0x07230203;
    if result.len() > 0 && result[0] == MAGIC_NUMBER.swap_bytes() {
        for word in &mut result {
            *word = word.swap_bytes();
        }
    }
    if result.len() == 0 || result[0] != MAGIC_NUMBER {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input missing SPIR-V magic number",
        ));
    }
    Ok(result)
}

/*
impl BaseShader for Shader {
    type Graphics = Context2d;
    type Buffer = BufferKey;
    type Attr = Attr;
    type Kind = Self;

    fn new<T: BaseSystem<Context2d = Self::Graphics>>(
        app: &mut T,
        vertex: &str,
        fragment: &str,
        attributes: Vec<Self::Attr>,
    ) -> Result<Self, String> {
        shader_from_gl_context(&app.ctx2().gl, vertex, fragment, attributes)
    }

    fn buffer(&self, name: &str) -> Option<Self::Buffer> {
        if let Some(attr) = self.inner.attributes.get(name) {
            return Some(attr.buffer);
        }

        None
    }

    fn from_image_fragment<T, S>(app: &mut T, fragment: &str) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Graphics>,
    {
        sprite_shader_from_gl_context(&app.system().ctx2().gl, Some(fragment))
    }

    fn from_text_fragment<T, S>(app: &mut T, fragment: &str) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Graphics>,
    {
        text_shader_from_gl_context(&app.system().ctx2().gl, Some(fragment))
    }

    fn from_color_fragment<T, S>(app: &mut T, fragment: &str) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Graphics>,
    {
        color_shader_from_gl_context(&app.system().ctx2().gl, Some(fragment))
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
            let location = gl
                .get_attrib_location(program, &attr.name)
                .ok_or("Invalid location")? as u32;
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
*/
fn create_shader(gl: &GlContext, typ: u32, source: &str) -> Result<ShaderKey, String> {
    unsafe {
        let shader = gl.create_shader(typ)?;
        gl.shader_source(shader, &source);
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
// https://github.com/floooh/sokol/blob/d86d96625d6be0171c99909187e041ae699dbbf0/util/sokol_gfx_imgui.h#L978
// CHANGE NAME TO VertexFormat and use the formats from sokol https://github.com/floooh/sokol-tools/blob/master/docs/sokol-shdc.md#creating-shaders-and-pipeline-objects
pub enum VertexFormat {
    Float1,
    Float2,
    Float3,
    Float4,
}

impl VertexFormat {
    pub fn size(&self) -> i32 {
        use VertexFormat::*;
        match self {
            Float1 => 1,
            Float2 => 2,
            Float3 => 3,
            Float4 => 4,
        }
    }

    pub fn bytes(&self) -> i32 {
        self.size() * 4
    }

    pub fn normalized(&self) -> bool {
        use VertexFormat::*;
        match self {
            _ => false,
        }
    }
}

pub trait GlowValue {
    fn glow_value(&self) -> u32;
}

impl GlowValue for VertexFormat {
    fn glow_value(&self) -> u32 {
        glow::FLOAT
    }
}

#[derive(Clone)]
pub struct Attr {
    name: String,
    vertex_data: VertexFormat,
}

impl Attr {
    pub fn new(name: &str, data_type: VertexFormat) -> Self {
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

/*
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
*/
