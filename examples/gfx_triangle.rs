use nae::prelude::*;
use nae_gfx::{Graphics, IndexBuffer, Pipeline, VertexAttr, VertexBuffer, VertexFormat};

struct State {
    pipeline: Pipeline,
    vertex_buffer: VertexBuffer,
    clear: ClearOptions,
    vertices: [f32; 21],
}

#[nae::main]
fn main() {
    if let Err(e) = nae::init_with(|app| {
        log::init();
        let mut gfx = app.gfx();

        #[cfg(not(target_arch = "wasm32"))]
        let shader = nae_gfx::Shader::new(
            &gfx,
            include_bytes!("./assets/color.vert.spv"),
            include_bytes!("./assets/color.frag.spv"),
        )
        .unwrap();

        #[cfg(target_arch = "wasm32")]
        let shader = nae_gfx::Shader::from_source(
            &gfx,
            r#"#version 300 es

out vec4 v_color;
layout(location = 1) in vec4 a_color;
layout(location = 0) in vec4 a_position;

void main()
{
    v_color = a_color;
    gl_Position = a_position;
}
            "#,
            r#"#version 300 es
precision mediump float;
precision highp int;

layout(location = 0) out highp vec4 color;
in highp vec4 v_color;

void main()
{
    color = v_color;
}
            "#,
        ).unwrap();

        let pipeline = Pipeline::new(&gfx, &shader, PipelineOptions::default());

        let vertex_buffer = VertexBuffer::new(
            &gfx,
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float4),
            ],
            DrawUsage::Dynamic,
        )
        .unwrap();

        let clear = ClearOptions {
            color: Some(Color::new(0.1, 0.2, 0.3, 1.0)),
            depth: None,
            stencil: None,
        };

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
    })
    .draw(draw)
    .build()
    {
        log::info!("{}", e);
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
