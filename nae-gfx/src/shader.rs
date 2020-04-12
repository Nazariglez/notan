use crate::{GlContext, GlowValue, Graphics};
use glow::HasContext;
use hashbrown::HashMap;
use nae_core::math::Mat3;
use nae_core::BaseShader;
use nae_core::{BaseApp, BaseSystem};
use nae_core::{BaseContext2d, GraphicsAPI};
use spirv_cross::{glsl, spirv, ErrorCode};
use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;
use std::{io, slice};

type ShaderKey = <glow::Context as HasContext>::Shader;
type ProgramKey = <glow::Context as HasContext>::Program;
type UniformLocationKey = <glow::Context as HasContext>::UniformLocation;
pub type BufferKey = <glow::Context as HasContext>::Buffer;

fn to_glsl_version(api: &GraphicsAPI) -> Option<glsl::Version> {
    use glsl::Version::*;
    use GraphicsAPI::*;

    Some(match api {
        WebGl => V1_00Es,
        WebGl2 => V3_00Es,
        OpenGl3_3 => V3_30,
        OpenGlEs2_0 => V1_00Es,
        _ => return None,
    })
}

pub(crate) struct InnerShader {
    pub gl: GlContext,
    pub raw: ProgramKey,
    vertex: ShaderKey,
    fragment: ShaderKey,
}

impl Drop for InnerShader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_shader(self.vertex);
            self.gl.delete_shader(self.fragment);
            self.gl.delete_program(self.raw);
        }
    }
}

#[derive(Clone)]
pub struct Shader {
    pub(crate) inner: Rc<InnerShader>,
}

impl Shader {
    pub fn new(graphics: &Graphics, vertex: &[u8], fragment: &[u8]) -> Result<Self, String> {
        let vert_spv = read_spirv(Cursor::new(&vertex[..])).map_err(|e| e.to_string())?;
        let frag_spv = read_spirv(Cursor::new(&fragment[..])).map_err(|e| e.to_string())?;

        let vert = compile_spirv_to_glsl(&vert_spv, &graphics.gfx_api)?;
        let frag = compile_spirv_to_glsl(&frag_spv, &graphics.gfx_api)?;

        //FIXME layout(binding = 0) is not allowed for

        // nae_core::log::info!("vert: {}", vert);
        // nae_core::log::info!("frag: {}", frag);

        Self::from_source(graphics, &vert, &frag)
    }

    pub fn from_source(graphics: &Graphics, vertex: &str, fragment: &str) -> Result<Self, String> {
        let gl = graphics.gl.clone();
        let vertex = create_shader(&gl, glow::VERTEX_SHADER, vertex)?;
        let fragment = create_shader(&gl, glow::FRAGMENT_SHADER, fragment)?;

        let program = create_program(&gl, vertex, fragment)?;
        let inner = InnerShader {
            gl,
            raw: program,
            vertex,
            fragment,
        };

        Ok(Self {
            inner: Rc::new(inner),
        })
    }
}

fn compile_spirv_to_glsl(source: &[u32], api: &GraphicsAPI) -> Result<String, String> {
    let module = spirv::Module::from_words(source);
    let mut ast = spirv::Ast::<glsl::Target>::parse(&module).map_err(error_code_to_string)?;
    let res = ast.get_shader_resources().map_err(|e| format!("{:?}", e))?;

    ast.set_compiler_options(&glsl::CompilerOptions {
        version: to_glsl_version(&api).ok_or("Invalid driver version")?,
        vertex: glsl::CompilerVertexOptions::default(),
    })
    .map_err(error_code_to_string)?;

    // println!(
    //     "{:?} {:?} {:?}",
    //     res.sampled_images, res.uniform_buffers, res.storage_buffers
    // );
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
        // println!("{:?}", r);
        ast.unset_decoration(r.id, spirv::Decoration::Binding)
            .unwrap();
    }
}

fn error_code_to_string(err: ErrorCode) -> String {
    match err {
        ErrorCode::Unhandled => String::from("Unhandled"),
        ErrorCode::CompilationError(e) => {
            nae_core::log::error!("e-> {}", e);
            e
        }
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

impl GlowValue for VertexFormat {
    type VALUE = u32;

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
