use notan::app::App;
use notan::core::events::DrawEvent;
use notan::gfx::{Buffer, Color, Gfx, RenderPipeline, Renderer, VertexFormat, VertexLayout};
use notan::prelude::*;

// language=wgsl
const SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.position = vec4<f32>(model.position - 0.5, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

#[derive(AppState)]
struct State {
    pip: RenderPipeline,
    vbo: Buffer,
}

impl State {
    fn new(gfx: &mut Gfx) -> Result<Self, String> {
        let pip = gfx
            .create_render_pipeline(SHADER)
            .with_vertex_layout(
                VertexLayout::new()
                    .with_attr(0, VertexFormat::Float32x2)
                    .with_attr(1, VertexFormat::Float32x3),
            )
            .build()?;

        #[rustfmt::skip]
            let vertices: &[f32] = &[
            0.5, 1.0,   1.0, 0.0, 0.0,
            0.0, 0.0,   0.0, 1.0, 0.0,
            1.0, 0.0,   0.0, 0.0, 1.0,
        ];

        let vbo = gfx.create_vertex_buffer(vertices).build()?;

        Ok(State { pip, vbo })
    }
}

fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(App::config())?
        .add_config(Gfx::config())?
        .on(on_draw)
        .build()
}

fn on_draw(evt: &DrawEvent, gfx: &mut Gfx, state: &mut State) {
    // Create new frame
    let frame = gfx.create_frame(evt.window_id).unwrap();

    // Renderer with the render pass for the triangle
    let mut renderer = Renderer::new();
    renderer
        .begin_pass()
        .clear_color(Color::rgb(0.1, 0.2, 0.3))
        .pipeline(&state.pip)
        .buffers(&[&state.vbo])
        .draw(0..3);

    // Render to the frame
    gfx.render(&frame, &renderer).unwrap();
    // gfx.render_to(&frame, &renderer).unwrap();

    // Present frame to screen
    gfx.present(frame).unwrap();
}
