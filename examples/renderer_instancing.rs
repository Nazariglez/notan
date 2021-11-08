use notan::prelude::*;

const INSTANCES:usize = 30000;

//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;
    layout(location = 0) out vec3 v_color;

    void main() {

        float value = gl_InstanceIndex * 0.1;
        v_color = vec3(fract(value), 1.0 - fract(value), fract(value));

        // vec2 pos = a_pos - vec2(sin(value), cos(value)) * 0.5;
        gl_Position = vec4(a_pos, 0.0, 1.0);
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

#[derive(AppState)]
struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    pos_vbo: VertexBuffer,
    // color_vbo: VertexBuffer,
    index_buffer: IndexBuffer,
}

#[notan_main]
fn main() -> Result<(), String> {
    // We'll override to debug always
    let level = notan::log::LevelFilter::Debug;

    // We use the default config with a custom log level
    let log_config = notan::log::LogConfig::default().level(level);

    notan::init_with(setup)
        .set_config(log_config).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    let clear_options = ClearOptions::new(Color::new(0.1, 0.2, 0.3, 1.0));

    let pipeline = gfx
        .create_pipeline()
        .from(&VERT, &FRAG)
        .vertex_attr(0, VertexFormat::Float2) // a_pos
        .build()
        .unwrap();

    #[rustfmt::skip]
    let pos = vec![
       -0.2, -0.2,
        0.2, -0.2,
        0.0, 0.2
    ];

    #[rustfmt::skip]
    let colors = vec![
        0.5, 1.0, 0.0,
        0.0, 1.0, 0.0,
        1.0, 0.0, 1.0,
    ];

    let indices = vec![0, 1, 2];

    let pos_vbo = gfx
        .create_vertex_buffer()
        .with_data(pos)
        .attr(0, VertexFormat::Float2)
        .build()
        .unwrap();

    // let color_vbo = gfx
    //     .create_vertex_buffer()
    //     .attr(1, VertexFormat::Float3)
    //     .step_mode(VertexStepMode::Instance)
    //     .with_data(colors)
    //     .build()
    //     .unwrap();

    let index_buffer = gfx
        .create_index_buffer()
        .with_data(indices)
        .build()
        .unwrap();

    State {
        clear_options,
        pipeline,
        pos_vbo,
        // color_vbo,
        index_buffer,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    // notan::log::info!("{}", app.timer.fps());
    let mut renderer = gfx.create_renderer();

    renderer.begin(Some(&state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_vertex_buffer(&state.pos_vbo);
    // renderer.bind_vertex_buffer(&state.color_vbo);
    // renderer.bind_index_buffer(&state.index_buffer);
    renderer.draw_instanced(0, 3, INSTANCES as _);
    renderer.end();

    gfx.render(&renderer);

}
