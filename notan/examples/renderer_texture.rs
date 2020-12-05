use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

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

struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    vertices: [f32; 20],
    indices: [u32; 6],
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    uniform_buffer: Buffer,
    texture: Texture,
}
impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build();

    Ok(())
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

    let vertex_buffer = gfx.create_vertex_buffer().unwrap();
    let index_buffer = gfx.create_index_buffer().unwrap();
    let uniform_buffer = gfx.create_uniform_buffer(0).unwrap();

    let texture = gfx.create_texture(Default::default()).unwrap();

    #[rustfmt::skip]
    let vertices = [
        //pos               //coords
        0.5,  0.5, 0.0,     1.0, 1.0,
        0.5, -0.5, 0.0,     1.0, 0.0,
        -0.5, -0.5, 0.0,    0.0, 0.0,
        -0.5,  0.5, 0.0,    0.0, 1.0
    ];

    #[rustfmt::skip]
    let indices = [
        0, 1, 3,
        1, 2, 3,
    ];

    let mut state = State {
        clear_options,
        pipeline,
        vertices,
        indices,
        vertex_buffer,
        index_buffer,
        uniform_buffer,
        texture,
    };

    state
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut renderer = gfx.create_renderer();

    renderer.begin(&state.clear_options);
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_texture(0, &state.texture);
    renderer.bind_vertex_buffer(&state.vertex_buffer, &state.vertices);
    renderer.bind_index_buffer(&state.index_buffer, &state.indices);
    renderer.draw(0, state.indices.len() as _);
    renderer.end();

    gfx.render(&renderer);
}
