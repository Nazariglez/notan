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
        gl_Position = a_position;
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

#[derive(notan::AppState)]
struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    vertex_buffer: Buffer<f32>,
    index_buffer: Buffer<u32>,
    texture: Texture,
    render_texture: RenderTexture,
    render_texture2: RenderTexture,
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    let clear_options = ClearOptions::new(Color::new(0.1, 0.2, 0.3, 1.0));

    let pipeline = gfx
        .create_pipeline(
            &VERT,
            &FRAG,
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float2),
            ],
            PipelineOptions {
                color_blend: Some(BlendMode::NORMAL),
                ..Default::default()
            },
        )
        .unwrap();

    let image = TextureInfo::from_image(include_bytes!("assets/ferris.png")).unwrap();
    let texture = gfx.create_texture(image).unwrap();

    let render_texture = gfx
        .create_render_texture(TextureInfo::render_texture(
            false,
            texture.width() as _,
            texture.height() as _,
        ))
        .unwrap();
    let render_texture2 = gfx
        .create_render_texture(TextureInfo::render_texture(
            false,
            texture.width() as _,
            texture.height() as _,
        ))
        .unwrap();

    #[rustfmt::skip]
        let vertices = vec![
        //pos               //coords
        0.9,  0.9, 0.0,     1.0, 1.0,
        0.9, -0.9, 0.0,     1.0, 0.0,
        -0.9, -0.9, 0.0,    0.0, 0.0,
        -0.9,  0.9, 0.0,    0.0, 1.0
    ];

    #[rustfmt::skip]
        let indices = vec![
        0, 1, 3,
        1, 2, 3,
    ];

    let vertex_buffer = gfx.create_vertex_buffer(vertices).unwrap();
    let index_buffer = gfx.create_index_buffer(indices).unwrap();

    State {
        clear_options,
        pipeline,
        vertex_buffer,
        index_buffer,
        texture,
        render_texture,
        render_texture2,
    }
}

// create an effect of infinite loop
fn draw(gfx: &mut Graphics, state: &mut State) {
    // draw the texture and the first render_texture on the second render_texture
    let image_on_rt2 = render_texture(gfx, state, &state.texture, false);
    gfx.render_to(&state.render_texture2, &image_on_rt2);
    let rt1_on_rt2 = render_texture(gfx, state, &state.render_texture, false);
    gfx.render_to(&state.render_texture2, &rt1_on_rt2);

    let rt2_on_screen = render_texture(gfx, state, &state.render_texture2, false);
    gfx.render(&rt2_on_screen);

    // swap render target to draw on the next frame on a different rt
    std::mem::swap(&mut state.render_texture, &mut state.render_texture2);
}

fn render_texture(gfx: &mut Graphics, state: &State, texture: &Texture, clear: bool) -> Renderer {
    let clear_options = if clear {
        ClearOptions::new(Color::new(0.1, 0.2, 0.3, 1.0))
    } else {
        ClearOptions::none()
    };

    let mut renderer = gfx.create_renderer();

    renderer.begin(Some(&clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_texture(0, texture);
    renderer.bind_vertex_buffer(&state.vertex_buffer);
    renderer.bind_index_buffer(&state.index_buffer);
    renderer.draw(0, 6);
    renderer.end();

    renderer
}
