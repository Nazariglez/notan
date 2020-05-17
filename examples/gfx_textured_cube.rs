use nae::prelude::*;
use nalgebra_glm as glm;

struct State {
    texture: Texture,
    pipeline: Pipeline,
    vertex_buffer: VertexBuffer,
    clear: ClearOptions,
    vertices: [f32; 180],
    rx: f32,
    ry: f32,
    tex_location: Uniform,
    mvp_location: Uniform,
    mvp: glm::Mat4,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    let texture = Texture::from_bytes(app, include_bytes!("assets/cube.png")).unwrap();

    let mut gfx = app.gfx();
    let pipeline = Pipeline::new(
        &gfx,
        include_bytes!("assets/shaders/image_matrix.vert.spv"),
        include_bytes!("assets/shaders/image.frag.spv"),
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

    let mvp_location = pipeline.uniform_location("u_matrix").unwrap();
    let tex_location = pipeline.uniform_location("u_texture").unwrap();

    let vertex_buffer = VertexBuffer::new(&gfx, DrawUsage::Dynamic).unwrap();

    let clear = ClearOptions {
        color: Some(Color::new(0.1, 0.2, 0.3, 1.0)),
        depth: Some(1.0),
        stencil: None,
    };

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
        1.0,-1.0,1.0,       0.667979,0.335851
    ];

    let projection: glm::Mat4 = glm::perspective(4.0 / 3.0, 45.0, 0.1, 100.0);
    let view = glm::look_at(
        &glm::vec3(4.0, 3.0, 3.0),
        &glm::vec3(0.0, 0.0, 0.0),
        &glm::vec3(0.0, 1.0, 0.0),
    );

    let mut mvp: glm::Mat4 = glm::Mat4::identity();
    mvp = mvp * projection;
    mvp = mvp * view;

    State {
        texture,
        pipeline,
        vertex_buffer,
        mvp_location,
        vertices,
        clear,
        rx: 0.0,
        ry: 0.0,
        mvp,
        tex_location,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let mvp = rotate_matrix(state);
    let count = state.vertices.len() / state.pipeline.offset();

    let mut gfx = app.gfx();
    gfx.begin(&state.clear);
    gfx.set_pipeline(&state.pipeline);
    gfx.bind_uniform(&state.mvp_location, slice_to_matrix4(mvp.as_slice()));
    gfx.bind_texture(&state.tex_location, &state.texture);
    gfx.bind_vertex_buffer(&state.vertex_buffer, &state.pipeline, &state.vertices);
    gfx.draw(0, count as _);
    gfx.end();
}

fn rotate_matrix(state: &mut State) -> glm::Mat4 {
    state.rx += 0.01;
    state.ry += 0.01;
    let rmx = glm::rotate_x(&state.mvp, state.rx);
    let mvp = glm::rotate_y(&rmx, state.ry);

    mvp
}
