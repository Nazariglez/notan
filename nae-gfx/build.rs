use shaderc::{GlslProfile, OptimizationLevel, TargetEnv};
use std::error::Error;
use std::io::Read;

fn is_release() -> bool {
    if let Ok(profile) = std::env::var("PROFILE") {
        return profile == "release";
    }

    false
}

const SHADER_DIRECTORY: &'static str = "resources/shaders";

fn main() -> Result<(), Box<Error>> {
    println!("cargo:rerun-if-changed={}", SHADER_DIRECTORY);

    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.set_target_env(TargetEnv::OpenGL, 0);
    if is_release() {
        // options.set_optimization_level(OptimizationLevel::Performance);
    }
    for entry in std::fs::read_dir(SHADER_DIRECTORY)? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let in_path = entry.path();

            // Support only vertex and fragment shaders currently
            let shader_type =
                in_path
                    .extension()
                    .and_then(|ext| match ext.to_string_lossy().as_ref() {
                        "vert" => Some(shaderc::ShaderKind::Vertex),
                        "frag" => Some(shaderc::ShaderKind::Fragment),
                        "geom" => Some(shaderc::ShaderKind::Geometry),
                        "comp" => Some(shaderc::ShaderKind::Compute),
                        _ => None,
                    });

            if let Some(kind) = shader_type {
                let name = in_path.file_name().unwrap().to_string_lossy().to_string();
                let source = std::fs::read_to_string(&in_path)?;
                let binary =
                    compiler.compile_into_spirv(&source, kind, &name, "main", Some(&options))?;
                let bytes = binary.as_binary_u8();

                // Determine the output path based on the input name
                let out_path = format!("{}/{}.spv", SHADER_DIRECTORY, name,);

                std::fs::write(&out_path, &bytes)?;
            }
        }
    }

    Ok(())
}
