use notan::prelude::*;

//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;

    void main() {
        gl_Position = vec4(a_pos - 0.5, 0.0, 1.0);
    }
    "#
};

//language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) out vec4 color;

    void main() {
        color = vec4(0.0, 1.0, 1.0, 1.0);
    }
    "#
};

#[derive(AppState)]
struct State {
    pipeline: Pipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    #[rustfmt::skip]
    let vertices = [
        0.0, 1.0, // top-left
        0.0, 0.0, // bottom-left
        1.0, 0.0, // bottom-right
        1.0, 1.0, // top-right
    ];

    let indices = [
        0, 1, 1, 2, 2, 0, // first triangle lines
        0, 2, 2, 3, 3, 0, // second triangle lines
    ];

    let vertex_info = VertexInfo::new().attr(0, VertexFormat::Float32x2);

    let pipeline = gfx
        .create_pipeline()
        .from(&VERT, &FRAG)
        .with_vertex_info(&vertex_info)
        .build()
        .unwrap();

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

    State {
        pipeline,
        vertex_buffer,
        index_buffer,
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut renderer = gfx.create_renderer();

    renderer.begin(Some(&ClearOptions::color(Color::BLACK)));
    renderer.set_pipeline(&state.pipeline);
    renderer.set_primitive(DrawPrimitive::LineStrip);
    renderer.bind_buffers(&[&state.vertex_buffer, &state.index_buffer]);
    renderer.draw(0, 12);
    renderer.end();

    gfx.render(&renderer);
}
