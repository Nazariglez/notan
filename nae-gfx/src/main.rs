use glow::{Context, HasContext};
use nae_core::{BlendFactor, BlendMode, Color};
use std::rc::Rc;

pub(crate) type GlContext = Rc<Context>;

pub trait BaseGraphics {
    fn viewport(&mut self, x: f32, y: f32, width: f32, height: f32);
    fn flush(&mut self);
    fn begin(&mut self);
    fn end(&mut self);
    fn clear(color: Option<Color>, depth: Option<f32>, stencil: Option<i32>);
    // etc...
}

struct Graphics {
    pub(crate) gl: GlContext,
}

#[macro_export]
macro_rules! include_glsl {
    ($path:expr) => {{
        let source = include_str!($path);
        source.as_bytes()
    }};
}

fn main() {
    let spv = read_spirv(Cursor::new(
        &include_bytes!("../resources/shaders/color.vert.spv")[..],
    ))
    .unwrap();
    let wgl = compile_spirv_to_glsl(&spv, Driver::WebGl).unwrap();
    let wgl2 = compile_spirv_to_glsl(&spv, Driver::WebGl2).unwrap();
    let gles2 = compile_spirv_to_glsl(&spv, Driver::OpenGlEs2_0).unwrap();
    let gl3 = compile_spirv_to_glsl(&spv, Driver::OpenGl3_3).unwrap();
    println!("wgl: {}\n", wgl);
    println!("wgl2: {}\n", wgl2);
    println!("gles2: {}\n", gles2);
    println!("gl3: {}\n", gl3);
    //    let n = include_glsl!("./color.vert.glsl");
    //    println!("{:?}", n);
}

use spirv_cross::{glsl, spirv, ErrorCode};

fn compile_spirv_to_glsl(source: &[u32], driver: Driver) -> Result<String, String> {
    let module = spirv::Module::from_words(source);
    let mut ast = spirv::Ast::<glsl::Target>::parse(&module).map_err(error_code_to_string)?;

    ast.set_compiler_options(&glsl::CompilerOptions {
        version: driver.to_glsl_version().ok_or("Invalid driver version")?,
        vertex: glsl::CompilerVertexOptions::default(),
    })
    .map_err(error_code_to_string)?;

    ast.compile().map_err(error_code_to_string)
}

fn error_code_to_string(err: ErrorCode) -> String {
    match err {
        ErrorCode::Unhandled => String::from("Unhandled"),
        ErrorCode::CompilationError(e) => e,
    }
}

#[derive(Clone, Copy)]
enum Driver {
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

use std::io::Cursor;
use std::{io, slice};

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
