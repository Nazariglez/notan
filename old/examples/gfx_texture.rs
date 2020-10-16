use nae::prelude::*;

struct State {
    texture: Texture,
    pipeline: Pipeline,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    vertices: [f32; 20],
    indices: [u32; 6],
    tex_location: Uniform,
    clear: ClearOptions,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    let texture = Texture::from_bytes(app, include_bytes!("assets/ferris.png")).unwrap();

    let mut gfx = app.gfx();

    let pipeline = Pipeline::new(
        &gfx,
        include_bytes!("assets/shaders/image.vert.spv"),
        include_bytes!("assets/shaders/image.frag.spv"),
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

    let tex_location = pipeline.uniform_location("u_texture").unwrap();

    let vertex_buffer = VertexBuffer::new(&gfx, DrawUsage::Dynamic).unwrap();

    let index_buffer = IndexBuffer::new(&gfx, DrawUsage::Dynamic).unwrap();

    let clear = ClearOptions {
        color: Some(Color::new(0.1, 0.2, 0.3, 1.0)),
        ..Default::default()
    };

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

    State {
        texture,
        vertex_buffer,
        index_buffer,
        vertices,
        indices,
        pipeline,
        clear,
        tex_location,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let mut gfx = app.gfx();
    gfx.begin(&state.clear);
    gfx.set_pipeline(&state.pipeline);
    gfx.bind_texture(&state.tex_location, &state.texture);
    gfx.bind_vertex_buffer(&state.vertex_buffer, &state.vertices);
    gfx.bind_index_buffer(&state.index_buffer, &state.indices);
    gfx.draw(0, state.indices.len() as _);
    gfx.end();
}
