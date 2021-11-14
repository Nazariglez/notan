use notan::prelude::*;

//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450

    layout(location = 0) in vec4 a_position;
    layout(location = 1) in vec2 a_texcoord;

    layout(location = 0) out vec2 v_texcoord;

    void main() {
        v_texcoord = a_texcoord;
        gl_Position = vec4(a_position.x, a_position.y * -1.0, a_position.z, 1.0);
    }
    "#
};

//language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_texcoord;

    layout(location = 0) out vec4 outColor;

    layout(set = 0, binding = 0) uniform sampler2D u_texture;
    void main() {
        outColor = texture(u_texture, v_texcoord);
    }
    "#
};

#[derive(AppState)]
struct State {
    pipeline: Pipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    texture: Texture,
    render_texture: RenderTexture,
    render_texture2: RenderTexture,
    texture_initiated: bool,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    let vertex_info = VertexInfo::new()
        .attr(0, VertexFormat::Float3)
        .attr(1, VertexFormat::Float2);

    let pipeline = gfx
        .create_pipeline()
        .from(&VERT, &FRAG)
        .vertex_info(&vertex_info)
        .with_color_blend(BlendMode::NORMAL)
        .build()
        .unwrap();

    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/ferris.png"))
        .build()
        .unwrap();

    let (width, height) = (texture.width() as i32, texture.height() as i32);
    let render_texture = gfx.create_render_texture(width, height).build().unwrap();
    let render_texture2 = gfx.create_render_texture(width, height).build().unwrap();

    #[rustfmt::skip]
    let vertices = [
        //pos               //coords
        0.9,  0.9, 0.0,     1.0, 1.0,
        0.9, -0.9, 0.0,     1.0, 0.0,
        -0.9, -0.9, 0.0,    0.0, 0.0,
        -0.9,  0.9, 0.0,    0.0, 1.0
    ];

    #[rustfmt::skip]
    let indices = [
        0, 1, 3,
        1, 2, 3,
    ];

    let vertex_buffer = gfx
        .create_vertex_buffer()
        .with_info(&vertex_info)
        .with_data(&vertices)
        .build()
        .unwrap();

    let index_buffer = gfx
        .create_index_buffer()
        .with_data(&indices)
        .build()
        .unwrap();

    State {
        pipeline,
        vertex_buffer,
        index_buffer,
        texture,
        render_texture,
        render_texture2,
        texture_initiated: false,
    }
}

// create an effect of infinite loop
fn draw(gfx: &mut Graphics, state: &mut State) {
    // drawing the render textures
    if !state.texture_initiated {
        for i in 0..30 {
            // the first pass will draw the texture to the rt1
            let tex = if i == 0 {
                &state.texture
            } else {
                &state.render_texture
            };

            // draw rt1 to rt2
            let image_on_rt2 = render_texture(gfx, state, tex, None);
            gfx.render_to(&state.render_texture2, &image_on_rt2);

            // draw rt2 to rt1
            let rt1_on_rt2 = render_texture(gfx, state, &state.render_texture2, None);
            gfx.render_to(&state.render_texture, &rt1_on_rt2);

            // swap render target to draw on the next frame on a different rt
            std::mem::swap(&mut state.render_texture, &mut state.render_texture2);
        }

        // avoid to do this on each frame
        state.texture_initiated = true;
    }

    // draw to screen the rt1
    let rt_to_screen = render_texture(gfx, state, &state.render_texture, Some(Color::ORANGE));
    gfx.render(&rt_to_screen);
}

fn render_texture(
    gfx: &mut Graphics,
    state: &State,
    texture: &Texture,
    clear_color: Option<Color>,
) -> Renderer {
    let mut renderer = gfx.create_renderer();

    renderer.begin(Some(&ClearOptions {
        color: clear_color,
        ..Default::default()
    }));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_texture(0, texture);
    renderer.bind_vertex_buffer(&state.vertex_buffer);
    renderer.bind_index_buffer(&state.index_buffer);
    renderer.draw(0, 6);
    renderer.end();

    renderer
}
