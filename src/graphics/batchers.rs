

fn create_sprite_shader(gl: &GlContext) -> Result<Shader, String> {
    let attrs = vec![
        Attribute::new("a_position", 2, glow::FLOAT, false),
        Attribute::new("a_color", 4, glow::FLOAT, false),
        Attribute::new("a_texcoord", 2, glow::FLOAT, true),
    ];

    let uniforms = vec!["u_matrix", "u_texture"];
    Ok(Shader::new(
        gl,
        include_str!("./shaders/image.vert.glsl"),
        include_str!("./shaders/image.frag.glsl"),
        attrs,
        uniforms,
    )?)
}

