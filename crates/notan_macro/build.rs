use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        use_shaderc: { feature = "shaderc" },
        use_glsl_to_spirv: { all(feature = "glsl-to-spirv", not(feature = "shaderc")) },
        shader_compilation: { any(use_shaderc, use_glsl_to_spirv) }
    }
}
