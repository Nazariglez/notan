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
    uniform_buffer: Buffer<f32>,
    mvp: glam::Mat4,
    angle: f32,
    texture: Texture,
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    let clear_options = ClearOptions {
        color: Some(Color::new(0.1, 0.2, 0.3, 1.0)),
        depth: Some(1.0),
        stencil: None,
    };

    let pipeline = gfx
        .create_pipeline(
            &VERT,
            &FRAG,
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float2),
            ],
            PipelineOptions {
                depth_stencil: DepthStencil {
                    write: true,
                    compare: CompareMode::Less,
                },
                ..Default::default()
            },
        )
        .unwrap();

    let image = TextureInfo::from_image(include_bytes!("assets/cube.png")).unwrap();
    let texture = gfx.create_texture(image).unwrap();

    #[rustfmt::skip]
    let vertices = vec![
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
        1.0,-1.0,1.0,       0.667979,0.3358510
    ];

    let projection = glam::Mat4::perspective_rh_gl(45.0, 4.0 / 3.0, 0.1, 100.0);
    let view = glam::Mat4::look_at_rh(
        glam::Vec3::new(4.0, 3.0, 3.0),
        glam::Vec3::new(0.0, 0.0, 0.0),
        glam::Vec3::new(0.0, 1.0, 0.0),
    );
    let mvp = glam::Mat4::identity() * projection * view;

    let vertex_buffer = gfx.create_vertex_buffer(vertices).unwrap();
    let uniform_buffer = gfx
        .create_uniform_buffer(0, "Locals", mvp.to_cols_array().to_vec())
        .unwrap();

    let mut state = State {
        clear_options,
        pipeline,
        vertex_buffer,
        uniform_buffer,
        texture,
        mvp,
        angle: 0.0,
    };

    state
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut renderer = gfx.create_renderer();

    let count = state.vertex_buffer.data().len() / state.pipeline.offset();
    state
        .uniform_buffer
        .copy(&rotated_matrix(state.mvp, state.angle));

    renderer.begin(Some(&state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_uniform_buffer(&state.uniform_buffer);
    renderer.bind_vertex_buffer(&state.vertex_buffer);
    renderer.bind_texture(0, &state.texture);
    renderer.draw(0, count as _);
    renderer.end();

    gfx.render(&renderer);

    state.angle += 0.01
}

fn rotated_matrix(base: glam::Mat4, angle: f32) -> [f32; 16] {
    let rot_x = glam::Mat4::from_rotation_x(angle);
    let rot_y = glam::Mat4::from_rotation_y(angle);
    (base * rot_x * rot_y).to_cols_array()
}
