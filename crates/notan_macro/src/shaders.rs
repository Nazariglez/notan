extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use spirv_cross::{glsl, spirv, ErrorCode};
use std::fs::read_to_string;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::{io, slice};

#[derive(Debug, Clone, Copy)]
pub(crate) enum ShaderType {
    Vertex,
    Fragment,
    //TODO more types
}

#[cfg(use_glsl_to_spirv)]
impl From<ShaderType> for glsl_to_spirv::ShaderType {
    fn from(value: ShaderType) -> Self {
        match value {
            ShaderType::Vertex => glsl_to_spirv::ShaderType::Vertex,
            ShaderType::Fragment => glsl_to_spirv::ShaderType::Fragment,
        }
    }
}

#[cfg(use_shaderc)]
impl From<ShaderType> for shaderc::ShaderKind {
    fn from(value: ShaderType) -> Self {
        match value {
            ShaderType::Vertex => shaderc::ShaderKind::Vertex,
            ShaderType::Fragment => shaderc::ShaderKind::Fragment,
        }
    }
}

fn get_root_path() -> PathBuf {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    Path::new(&root).to_path_buf()
}

fn read_file(full_path: &Path) -> Result<String, String> {
    if !full_path.is_file() {
        return Err(format!("File {} was not found.", full_path.display()));
    }

    read_to_string(full_path).map_err(|e| e.to_string())
}

pub(crate) fn spirv_from_file(relative_path: &str, typ: ShaderType) -> Result<Vec<u8>, String> {
    let root_path = get_root_path();
    let full_path = root_path.join(Path::new(relative_path));
    spirv_from(&read_file(&full_path)?, typ, Some(full_path))
}

#[cfg(use_glsl_to_spirv)]
pub(crate) fn spirv_from(
    source: &str,
    typ: ShaderType,
    _file_path: Option<PathBuf>,
) -> Result<Vec<u8>, String> {
    let source = source.trim();
    let mut spirv_output = glsl_to_spirv::compile(source, typ.into())
        .unwrap_or_else(|e| panic!("Invalid {typ:#?} shader: \n{e}"));

    let mut spirv = vec![];
    spirv_output
        .read_to_end(&mut spirv)
        .map_err(|e| e.to_string())?;
    Ok(spirv)
}

#[cfg(use_shaderc)]
pub(crate) fn spirv_from(
    source: &str,
    typ: ShaderType,
    file_path: Option<PathBuf>,
) -> Result<Vec<u8>, String> {
    use shaderc::IncludeType;

    let source = source.trim();
    let compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();

    // Resolve `#include` directives
    if let Some(file_path) = file_path.as_ref() {
        let file_dir = file_path.parent().unwrap();

        options.set_include_callback(|name, type_, _filename, _include_depth| {
            let include_path = match type_ {
                IncludeType::Relative => file_dir.join(name),
                IncludeType::Standard => get_root_path().join(name),
            };
            let include_path_string = include_path.to_string_lossy().into_owned();

            if let Ok(file_content) = read_file(include_path.as_path()) {
                Ok(shaderc::ResolvedInclude {
                    content: file_content,
                    resolved_name: include_path_string,
                })
            } else {
                Err(format!(
                    "Failed to include file: \"{}\" (from \"{}\")",
                    name, include_path_string
                ))
            }
        });
    }

    let input_file_name = file_path
        .as_ref()
        .and_then(|f| f.file_name())
        .and_then(|f| f.to_str())
        .unwrap_or("shader.glsl");

    let spirv_output = compiler
        .compile_into_spirv(source, typ.into(), input_file_name, "main", Some(&options))
        .unwrap_or_else(|e| panic!("Invalid {typ:#?} shader: \n{e}"));

    let mut spirv = vec![];
    spirv_output
        .as_binary_u8()
        .read_to_end(&mut spirv)
        .map_err(|e| e.to_string())?;
    Ok(spirv)
}

struct ShaderBytes(Vec<u8>);

impl quote::ToTokens for ShaderBytes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tree = self.0.iter().enumerate().flat_map(|(i, v)| {
            let byte = proc_macro2::TokenTree::Literal(proc_macro2::Literal::u8_suffixed(*v));

            let mut buff = vec![byte];
            if i < self.0.len() {
                buff.push(proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                    ',',
                    proc_macro2::Spacing::Alone,
                )));
            }

            buff
        });

        proc_macro2::Group::new(proc_macro2::Delimiter::Bracket, tree.collect()).to_tokens(tokens)
    }
}

pub(crate) fn source_from_spirv(spirv: Vec<u8>) -> Result<TokenStream, String> {
    let webgl2_bytes = spirv_to(&spirv, Output::Webgl2)?;
    let opengl_3_3_bytes = spirv_to(&spirv, Output::OpenGl3_3)?;

    Ok((quote! {
        ShaderSource {
            sources: &[
                #[cfg(target_arch = "wasm32")]
                ("webgl2", &#webgl2_bytes),

                #[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))]
                ("opengl", &#opengl_3_3_bytes),
            ]
        }
    })
    .into())
}

#[allow(non_camel_case_types, unused)]
#[derive(Debug, Clone, Copy)]
enum Output {
    Webgl2,
    OpenGl3_3,
    OpenGl_ES,
    Wgpu,
}

impl From<Output> for Option<glsl::Version> {
    fn from(value: Output) -> Self {
        use glsl::Version::*;

        Some(match value {
            Output::Webgl2 => V3_00Es,
            Output::OpenGl3_3 => V3_30,
            Output::OpenGl_ES => V3_00Es,
            _ => return None,
        })
    }
}

fn spirv_to(spirv: &[u8], output: Output) -> Result<ShaderBytes, String> {
    match output {
        Output::Wgpu => Ok(ShaderBytes(spirv.to_vec())),
        _ => spirv_to_glsl(spirv, output),
    }
}

fn spirv_to_glsl(spirv: &[u8], output: Output) -> Result<ShaderBytes, String> {
    let spv = read_spirv(Cursor::new(spirv)).map_err(|e| e.to_string())?;
    let glsl = compile_spirv_to_glsl(&spv, output)?;
    // println!("{:?} \n{}", output, glsl);
    Ok(ShaderBytes(glsl.as_bytes().to_vec()))
}

//- Most of this code is based on https://github.com/gfx-rs/gfx/blob/master/src/backend/gl/src/device.rs
fn compile_spirv_to_glsl(source: &[u32], api: Output) -> Result<String, String> {
    let module = spirv::Module::from_words(source);
    let mut ast = spirv::Ast::<glsl::Target>::parse(&module).map_err(error_code_to_string)?;
    let res = ast.get_shader_resources().map_err(|e| format!("{e:?}"))?;

    let version: Option<glsl::Version> = api.into();
    let version = version.ok_or("Invalid GLSL version")?;
    let vertex = glsl::CompilerVertexOptions::default();

    let mut options = glsl::CompilerOptions::default();
    options.version = version;
    options.vertex = vertex;
    options.force_zero_initialized_variables = true;
    ast.set_compiler_options(&options)
        .map_err(error_code_to_string)?;

    //TODO get spirv for vulkan as input and output glsl for opengl
    //https://community.arm.com/developer/tools-software/graphics/b/blog/posts/spirv-cross-working-with-spir-v-in-your-app
    fix_ast_for_gl(&mut ast, &res.sampled_images);
    fix_ast_for_gl(&mut ast, &res.uniform_buffers);
    fix_ast_for_gl(&mut ast, &res.storage_buffers);

    ast.compile().map_err(error_code_to_string)
}

fn fix_ast_for_gl(ast: &mut spirv::Ast<glsl::Target>, resources: &[spirv::Resource]) {
    resources.iter().for_each(|res| {
        ast.unset_decoration(res.id, spirv::Decoration::Binding)
            .unwrap();
        ast.unset_decoration(res.id, spirv::Decoration::DescriptorSet)
            .unwrap();
    });
}

fn error_code_to_string(err: ErrorCode) -> String {
    match err {
        ErrorCode::Unhandled => String::from("Unhandled"),
        ErrorCode::CompilationError(e) => {
            println!("e-> {e}");
            e
        }
    }
}

pub fn read_spirv<R: io::Read + io::Seek>(mut x: R) -> io::Result<Vec<u32>> {
    let size = x.seek(io::SeekFrom::End(0))?;
    if size % 4 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input length not divisible by 4",
        ));
    }
    if size > usize::MAX as u64 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "input too long"));
    }
    let words = (size / 4) as usize;
    let mut result = Vec::<u32>::with_capacity(words);
    x.rewind()?;
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
    if !result.is_empty() && result[0] == MAGIC_NUMBER.swap_bytes() {
        for word in &mut result {
            *word = word.swap_bytes();
        }
    }
    if result.is_empty() || result[0] != MAGIC_NUMBER {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input missing SPIR-V magic number",
        ));
    }
    Ok(result)
}
