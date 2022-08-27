use notan::math::{Mat4, Vec3};
use notan::prelude::*;

//language=glsl
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

//language=glsl
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

#[derive(AppState)]
struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    vbo: Buffer,
    ebo: Buffer,
    ubo: Buffer,
    mvp: notan::math::Mat4,
    angle: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
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

    let vertex_info = VertexInfo::new()
        .attr(0, VertexFormat::Float32x3)
        .attr(1, VertexFormat::Float32x4);

    let pipeline = gfx
        .create_pipeline()
        .from(&VERT, &FRAG)
        .with_vertex_info(&vertex_info)
        .with_depth_stencil(depth_stencil)
        .build()
        .unwrap();

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

    let projection = Mat4::perspective_rh_gl(45.0, 4.0 / 3.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(
        Vec3::new(4.0, 3.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let mvp = Mat4::IDENTITY * projection * view;

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

    let uniform_buffer = gfx
        .create_uniform_buffer(0, "Locals")
        .with_data(&mvp)
        .build()
        .unwrap();

    State {
        clear_options,
        pipeline,
        vbo: vertex_buffer,
        ebo: index_buffer,
        ubo: uniform_buffer,
        mvp,
        angle: 0.0,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut renderer = gfx.create_renderer();

    gfx.set_buffer_data(&state.ubo, &rotated_matrix(state.mvp, state.angle));

    renderer.begin(Some(&state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_buffers(&[&state.vbo, &state.ebo, &state.ubo]);
    renderer.draw(0, 36);
    renderer.end();

    gfx.render(&renderer);

    state.angle += 0.6 * app.timer.delta_f32();
}

fn rotated_matrix(base: Mat4, angle: f32) -> Mat4 {
    let rot_x = Mat4::from_rotation_x(angle);
    let rot_y = Mat4::from_rotation_y(angle);
    base * rot_x * rot_y
}
