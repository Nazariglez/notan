use cfg_aliases::cfg_aliases;

fn main() {
    // We're defining features here to make it easy to swap between
    // naga, glsl-to-spirv and shaderc keeping a priority order
    // without emit compile errors that can mess with compilations
    // wit the --all-features flag enabled.

    // TODO: add naga once the PR lands
    cfg_aliases! {
        use_naga: { all(feature = "naga", not(use_glsl_to_spirv), not(use_shaderc)) },
        use_glsl_to_spirv: { all(feature = "glsl-to-spirv", not(feature = "shaderc")) },
        use_shaderc: { feature = "shaderc" },
        shader_compilation: { any(use_naga, use_glsl_to_spirv, use_shaderc) }
    }
}
