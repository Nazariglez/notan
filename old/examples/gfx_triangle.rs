use nae::prelude::*;

struct State {
    pipeline: Pipeline,
    vertex_buffer: VertexBuffer,
    clear: ClearOptions,
    vertices: [f32; 21],
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    let mut gfx = app.gfx();
    let pipeline = Pipeline::new(
        &gfx,
        include_bytes!("assets/shaders/color.vert.spv"),
        include_bytes!("assets/shaders/color.frag.spv"),
        &[
            VertexAttr::new(0, VertexFormat::Float3),
            VertexAttr::new(1, VertexFormat::Float4),
        ],
        PipelineOptions::default(),
    )
    .unwrap();

    let vertex_buffer = VertexBuffer::new(&gfx, DrawUsage::Dynamic).unwrap();

    let clear = ClearOptions::new(Color::new(0.1, 0.2, 0.3, 1.0));

    #[rustfmt::skip]
    let vertices = [
        -0.5, -0.5, 0.0,    1.0, 0.2, 0.3, 1.0,
        0.5, -0.5, 0.0,     0.1, 1.0, 0.3, 1.0,
        0.0, 0.5, 0.0,      0.1, 0.2, 1.0, 1.0,
    ];

    State {
        pipeline,
        vertex_buffer,
        vertices,
        clear,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let mut gfx = app.gfx();
    gfx.begin(&state.clear);
    gfx.set_pipeline(&state.pipeline);
    gfx.bind_vertex_buffer(&state.vertex_buffer, &state.vertices);
    gfx.draw(0, 3);
    gfx.end();
}
