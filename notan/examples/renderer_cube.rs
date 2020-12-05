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
    layout(location = 1) in vec4 a_color;

    layout(location = 0) out vec4 v_color;

    layout(set = 0, binding = 0) uniform Locals {
        mat4 u_matrix;
    };

    void main() {
        v_color = a_color;
        gl_Position = u_matrix * a_position;
    }
    "#
};

const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec4 v_color;
    layout(location = 0) out vec4 color;

    void main() {
        color = v_color;
    }
    "#
};

struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    vertices: [f32; 168],
    indices: [u32; 36],
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    uniform_buffer: Buffer,
    mvp: glam::Mat4,
    angle: f32,
}
impl AppState for State {}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build();

    Ok(())
}

fn setup(gfx: &mut Graphics) -> State {
    let clear_options = ClearOptions {
        color: Some(Color::new(0.1, 0.2, 0.3, 1.0)),
        depth: Some(1.0),
        stencil: None,
    };

    let depth_stencil = DepthStencil {
        write: true,
        compare: CompareMode::Less,
    };

    let pipeline = gfx
        .create_pipeline(
            &VERT,
            &FRAG,
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float4),
            ],
            PipelineOptions {
                depth_stencil,
                ..Default::default()
            },
        )
        .unwrap();

    let vertex_buffer = gfx.create_vertex_buffer().unwrap();
    let index_buffer = gfx.create_index_buffer().unwrap();
    let uniform_buffer = gfx.create_uniform_buffer(0).unwrap();

    #[rustfmt::skip]
    let vertices = [
        -1.0, -1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
        1.0, -1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
        1.0,  1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
        -1.0,  1.0, -1.0,   1.0, 0.0, 0.0, 1.0,

        -1.0, -1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
        1.0, -1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
        1.0,  1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
        -1.0,  1.0,  1.0,   0.0, 1.0, 0.0, 1.0,

        -1.0, -1.0, -1.0,   0.0, 0.0, 1.0, 1.0,
        -1.0,  1.0, -1.0,   0.0, 0.0, 1.0, 1.0,
        -1.0,  1.0,  1.0,   0.0, 0.0, 1.0, 1.0,
        -1.0, -1.0,  1.0,   0.0, 0.0, 1.0, 1.0,

        1.0, -1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
        1.0,  1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
        1.0,  1.0,  1.0,    1.0, 0.5, 0.0, 1.0,
        1.0, -1.0,  1.0,    1.0, 0.5, 0.0, 1.0,

        -1.0, -1.0, -1.0,   0.0, 0.5, 1.0, 1.0,
        -1.0, -1.0,  1.0,   0.0, 0.5, 1.0, 1.0,
        1.0, -1.0,  1.0,   0.0, 0.5, 1.0, 1.0,
        1.0, -1.0, -1.0,   0.0, 0.5, 1.0, 1.0,

        -1.0,  1.0, -1.0,   1.0, 0.0, 0.5, 1.0,
        -1.0,  1.0,  1.0,   1.0, 0.0, 0.5, 1.0,
        1.0,  1.0,  1.0,   1.0, 0.0, 0.5, 1.0,
        1.0,  1.0, -1.0,   1.0, 0.0, 0.5, 1.0,
    ];

    #[rustfmt::skip]
    let indices = [
        0, 1, 2,  0, 2, 3,
        6, 5, 4,  7, 6, 4,
        8, 9, 10,  8, 10, 11,
        14, 13, 12,  15, 14, 12,
        16, 17, 18,  16, 18, 19,
        22, 21, 20,  23, 22, 20
    ];

    let projection = glam::Mat4::perspective_rh_gl(45.0, 4.0 / 3.0, 0.1, 100.0);
    let view = glam::Mat4::look_at_rh(
        glam::Vec3::new(4.0, 3.0, 3.0),
        glam::Vec3::new(0.0, 0.0, 0.0),
        glam::Vec3::new(0.0, 1.0, 0.0),
    );
    let mvp = glam::Mat4::identity() * projection * view;

    let mut state = State {
        clear_options,
        pipeline,
        vertices,
        indices,
        vertex_buffer,
        index_buffer,
        uniform_buffer,
        mvp,
        angle: 0.0,
    };

    state
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut renderer = gfx.create_renderer();

    let mvp = rotated_matrix(state.mvp, state.angle);

    renderer.begin(&state.clear_options);
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_uniform_buffer(&state.uniform_buffer, &mvp);
    renderer.bind_vertex_buffer(&state.vertex_buffer, &state.vertices);
    renderer.bind_index_buffer(&state.index_buffer, &state.indices);
    renderer.draw(0, state.indices.len() as i32);
    renderer.end();

    gfx.render(&renderer);

    state.angle += 0.01;
}

fn rotated_matrix(base: glam::Mat4, angle: f32) -> [f32; 16] {
    let rot_x = glam::Mat4::from_rotation_x(angle);
    let rot_y = glam::Mat4::from_rotation_y(angle);
    (base * rot_x * rot_y).to_cols_array()
}
