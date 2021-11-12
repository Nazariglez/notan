use notan::prelude::*;
use notan_log::LevelFilter;

// Number of triangles to draw
const INSTANCES: usize = 50000;

//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;
    layout(location = 0) out vec3 v_color;

    layout(set = 0, binding = 0) uniform Locals {
        float count;
    };

    void main() {
        // Values to change position and color
        float n = gl_InstanceIndex * 0.1;
        float j = gl_VertexIndex * 0.2;
        vec2 pos = a_pos - vec2(sin(n + count), cos(n + count)) * fract(n) * 0.9;

        v_color = vec3(fract(n - j), 1.0 - fract(n), fract(n + j));
        gl_Position = vec4(pos, 0.0, 1.0);
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
    pipeline: Pipeline,
    vbo: VertexBuffer,
    ubo: UniformBuffer,
    count: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    let lc = notan::log::LogConfig::new(LevelFilter::Debug);
    notan::init_with(setup).set_config(lc).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
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

    let vbo = gfx
        .create_vertex_buffer()
        .with_data(pos)
        .attr(0, VertexFormat::Float2)
        .build()
        .unwrap();

    let ubo = gfx
        .create_uniform_buffer(0, "Locals")
        .with_data(vec![0.0])
        .build()
        .unwrap();

    State {
        pipeline,
        vbo,
        ubo,
        count: 0.0,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    notan::log::info!("fps {}", app.timer.fps());
    // Renderer pass as usual but instead of .draw uses .draw_instanced
    let mut renderer = gfx.create_renderer();
    renderer.begin(Some(&ClearOptions::new(Color::BLACK)));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_vertex_buffer(&state.vbo);
    renderer.bind_uniform_buffer(&state.ubo);
    renderer.draw_instanced(0, 3, INSTANCES as _);
    renderer.end();

    // Render to the screen
    gfx.render(&renderer);

    // Update the uniform to animate the triangles
    state.count += 0.05 * app.timer.delta_f32();
    state.ubo.set(&[state.count]);
}
