use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec3 a_color;

    layout(location = 0) out vec3 v_color;

    void main() {
        v_color = a_color;
        gl_Position = vec4(a_pos - 0.5, 0.0, 1.0);
    }
    "#
};

//language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec3 v_color;
    layout(location = 0) out vec4 color;

    void main() {
        color = vec4(v_color, 1.0);
    }
    "#
};

struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    vertices: [f32; 15],
    vertex_buffer: Buffer,
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
                VertexAttr::new(0, VertexFormat::Float2),
                VertexAttr::new(1, VertexFormat::Float3),
            ],
            PipelineOptions::default(),
        )
        .unwrap();

    #[rustfmt::skip]
    let vertices = [
        0.5, 1.0,   1.0, 0.2, 0.3,
        0.0, 0.0,   0.1, 1.0, 0.3,
        1.0, 0.0,   0.1, 0.2, 1.0,
    ];

    let vertex_buffer = gfx.create_vertex_buffer(DrawType::Static).unwrap();

    let mut state = State {
        clear_options,
        pipeline,
        vertices,
        vertex_buffer,
    };

    state
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut renderer = gfx.create_renderer();

    renderer.begin(&state.clear_options);
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_vertex_buffer(&state.vertex_buffer, &state.vertices);
    renderer.draw(0, 3);
    renderer.end();

    gfx.render(&renderer);
}
