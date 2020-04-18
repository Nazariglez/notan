use nae::prelude::*;
use nalgebra_glm as glm;

struct State {
    pipeline: Pipeline,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    clear: ClearOptions,
    vertices: [f32; 168],
    indices: [u32; 36],
    rotation: (f32, f32),
    mvp: glm::Mat4,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    let mut gfx = app.gfx();
    let shader = nae_gfx::Shader::new(
        &gfx,
        include_bytes!("assets/shaders/color_matrix.vert.spv"),
        include_bytes!("assets/shaders/color.frag.spv"),
    )
    .unwrap();

    let pipeline = Pipeline::new(
        &gfx,
        &shader,
        PipelineOptions {
            depth_stencil: DepthStencil::Less,
            ..Default::default()
        },
    );
    let vertex_buffer = VertexBuffer::new(
        &gfx,
        &[
            VertexAttr::new(0, VertexFormat::Float3),
            VertexAttr::new(1, VertexFormat::Float4),
        ],
        DrawUsage::Dynamic,
    )
    .unwrap();

    let index_buffer = IndexBuffer::new(&gfx, DrawUsage::Dynamic).unwrap();

    let clear = ClearOptions {
        color: Some(Color::new(0.1, 0.2, 0.3, 1.0)),
        depth: Some(1.0),
        stencil: None,
    };

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
        pipeline,
        vertex_buffer,
        index_buffer,
        vertices,
        clear,
        indices,
        rotation: (0.0, 0.0),
        mvp,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let mvp = rotate_matrix(state);

    let mut gfx = app.gfx();
    gfx.begin(&state.clear);
    gfx.set_pipeline(&state.pipeline);
    gfx.bind_uniform(0, slice_to_matrix4(mvp.as_slice()));
    gfx.bind_vertex_buffer(&state.vertex_buffer, &state.vertices);
    gfx.bind_index_buffer(&state.index_buffer, &state.indices);
    gfx.draw(0, state.indices.len() as i32);
    gfx.end();
}

fn rotate_matrix(state: &mut State) -> glm::Mat4 {
    let (ref mut rx, ref mut ry) = state.rotation;
    *rx += 0.01;
    *ry += 0.01;
    let rmx = glm::rotate_x(&state.mvp, *rx);
    let mvp = glm::rotate_y(&rmx, *ry);
    mvp
}
