use notan::prelude::*;

//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450

    layout(location = 0) in vec4 a_position;
    layout(location = 1) in vec2 a_texcoord;

    layout(location = 0) out vec2 v_texcoord;
    layout(set = 0, binding = 0) uniform Locals {
        mat4 u_matrix;
    };

    void main() {
        v_texcoord = a_texcoord;
        gl_Position = u_matrix * a_position;
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

    layout(binding = 0) uniform sampler2D u_texture;

    void main() {
        outColor = texture(u_texture, v_texcoord);
    }
    "#
};

#[derive(AppState)]
struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    vertex_buffer: Buffer,
    uniform_buffer: Buffer,
    mvp: notan::math::Mat4,
    angle: f32,
    texture: Texture,
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

    let vertex_info = VertexInfo::new()
        .attr(0, VertexFormat::Float32x3)
        .attr(1, VertexFormat::Float32x2);

    let pipeline = gfx
        .create_pipeline()
        .from(&VERT, &FRAG)
        .with_vertex_info(&vertex_info)
        .with_texture_location(0, "u_texture")
        .with_depth_stencil(DepthStencil {
            write: true,
            compare: CompareMode::Less,
        })
        .build()
        .unwrap();

    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/cube.png"))
        .build()
        .unwrap();

    #[rustfmt::skip]
    let vertices = [
        -1.0,-1.0,-1.0,     0.000059,0.000004,
        -1.0,-1.0,1.0,      0.000103,0.336048,
        -1.0,1.0,1.0,       0.335973,0.335903,
        1.0,1.0,-1.0,       1.000023,0.000013,
        -1.0,-1.0,-1.0,     0.667979,0.335851,
        -1.0,1.0,-1.0,      0.999958,0.336064,
        1.0,-1.0,1.0,       0.667979,0.335851,
        -1.0,-1.0,-1.0,     0.336024,0.671877,
        1.0,-1.0,-1.0,      0.667969,0.671889,
        1.0,1.0,-1.0,       1.000023,0.000013,
        1.0,-1.0,-1.0,      0.668104,0.000013,
        -1.0,-1.0,-1.0,     0.667979,0.335851,
        -1.0,-1.0,-1.0,     0.000059,0.000004,
        -1.0,1.0,1.0,       0.335973,0.335903,
        -1.0,1.0,-1.0,      0.336098,0.000071,
        1.0,-1.0,1.0,       0.667979,0.335851,
        -1.0,-1.0,1.0,      0.335973,0.335903,
        -1.0,-1.0,-1.0,     0.336024,0.671877,
        -1.0,1.0,1.0,       1.000004,0.671847,
        -1.0,-1.0,1.0,      0.999958,0.336064,
        1.0,-1.0,1.0,       0.667979,0.335851,
        1.0,1.0,1.0,        0.668104,0.000013,
        1.0,-1.0,-1.0,      0.335973,0.335903,
        1.0,1.0,-1.0,       0.667979,0.335851,
        1.0,-1.0,-1.0,      0.335973,0.335903,
        1.0,1.0,1.0,        0.668104,0.000013,
        1.0,-1.0,1.0,       0.336098,0.000071,
        1.0,1.0,1.0,        0.000103,0.336048,
        1.0,1.0,-1.0,       0.000004,0.671870,
        -1.0,1.0,-1.0,      0.336024,0.671877,
        1.0,1.0,1.0,        0.000103,0.336048,
        -1.0,1.0,-1.0,      0.336024,0.671877,
        -1.0,1.0,1.0,       0.335973,0.335903,
        1.0,1.0,1.0,        0.667969,0.671889,
        -1.0,1.0,1.0,       1.000004,0.671847,
        1.0,-1.0,1.0,       0.667979,0.335_851
    ];

    let projection = notan::math::Mat4::perspective_rh_gl(45.0, 4.0 / 3.0, 0.1, 100.0);
    let view = notan::math::Mat4::look_at_rh(
        notan::math::Vec3::new(4.0, 3.0, 3.0),
        notan::math::Vec3::new(0.0, 0.0, 0.0),
        notan::math::Vec3::new(0.0, 1.0, 0.0),
    );
    let mvp = notan::math::Mat4::IDENTITY * projection * view;

    let vertex_buffer = gfx
        .create_vertex_buffer()
        .with_info(&vertex_info)
        .with_data(&vertices)
        .build()
        .unwrap();

    let uniform_buffer = gfx
        .create_uniform_buffer(0, "Locals")
        .with_data(&[mvp])
        .build()
        .unwrap();

    State {
        clear_options,
        pipeline,
        vertex_buffer,
        uniform_buffer,
        texture,
        mvp,
        angle: 0.0,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    gfx.set_buffer_data(
        &state.uniform_buffer,
        &rotated_matrix(state.mvp, state.angle),
    );

    let mut renderer = gfx.create_renderer();

    let vertices_count = 180;
    let count = vertices_count / state.pipeline.offset();

    renderer.begin(Some(&state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_buffers(&[&state.vertex_buffer, &state.uniform_buffer]);
    renderer.bind_texture(0, &state.texture);
    renderer.draw(0, count as _);
    renderer.end();

    gfx.render(&renderer);

    state.angle += 0.6 * app.timer.delta_f32();
}

fn rotated_matrix(base: notan::math::Mat4, angle: f32) -> [f32; 16] {
    let rot_x = notan::math::Mat4::from_rotation_x(angle);
    let rot_y = notan::math::Mat4::from_rotation_y(angle);
    (base * rot_x * rot_y).to_cols_array()
}
