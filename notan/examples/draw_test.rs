use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::graphics::Path;
use notan::app::{App, AppBuilder, AppFlow, Graphics, Plugins};
use notan::log;
use notan::prelude::*;
use notan::{fragment_shader, vertex_shader};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "sb")]
    fn sb();

    #[wasm_bindgen(js_name = "se")]
    fn se();
}

//---
//language=glsl
const COLOR_VERTEX: ShaderSource = vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec4 a_color;

    layout(location = 0) out vec4 v_color;
    layout(set = 0, binding = 0) uniform Locals {
        mat4 u_projection;
    };

    void main() {
        v_color = a_color;
        // gl_Position = vec4(a_pos - 0.5, 0.0, 1.0);
        gl_Position = u_projection * vec4(a_pos, 0.0, 1.0);
    }
    "#
};

//language=glsl
const COLOR_FRAGMENT: ShaderSource = fragment_shader! {
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

#[derive(Debug, Clone)]
enum DrawBatch {
    None,
    Color {
        pipeline: Option<Pipeline>,
        vertices: Vec<f32>,
        indices: Vec<u32>,
    },
}

impl DrawBatch {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

    pub fn is_color(&self) -> bool {
        match self {
            Self::Color { .. } => true,
            _ => false,
        }
    }
}

pub struct Draw2 {
    background: Option<Color>,
    initialized: bool,
    color: Color,
    alpha: f32,
    batches: Vec<DrawBatch>,
    current_batch: DrawBatch,
}

impl Draw2 {
    pub fn new() -> Self {
        Draw2 {
            initialized: false,
            color: Color::WHITE,
            alpha: 1.0,
            background: None,
            batches: vec![],
            current_batch: DrawBatch::None,
        }
    }

    pub fn background(&mut self, color: Color) {
        self.background = Some(color);
    }

    pub fn push_color<'a>(&mut self, info: &DrawInfo2<'a>) {
        //new batch if pipelines changes, otherwise add to the current one the vertices
        if !self.current_batch.is_color() {
            let old = std::mem::replace(
                &mut self.current_batch,
                DrawBatch::Color {
                    pipeline: None,
                    vertices: vec![],
                    indices: vec![],
                },
            );
            if !old.is_none() {
                self.batches.push(old);
            }
        }

        match &mut self.current_batch {
            DrawBatch::Color {
                vertices, indices, ..
            } => {
                //multiply by matrix, alpha and color
                let index = (vertices.len() as u32) / 6;
                vertices.extend(info.vertices);
                indices.extend(info.indices.iter().map(|i| i + index).collect::<Vec<_>>());
            }
            _ => {}
        }
    }
}

pub struct DrawInfo2<'a> {
    vertices: &'a [f32],
    indices: &'a [u32],
}

pub struct ColorBatcher2 {
    vbo: Buffer<f32>,
    ebo: Buffer<u32>,
    ubo: Buffer<f32>,
    pipeline: Pipeline,
    clear_options: ClearOptions,
    index: usize,
}

impl ColorBatcher2 {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let pipeline = gfx.create_pipeline(
            &COLOR_VERTEX,
            &COLOR_FRAGMENT,
            &[
                VertexAttr::new(0, VertexFormat::Float2),
                VertexAttr::new(1, VertexFormat::Float4),
            ],
            PipelineOptions {
                color_blend: Some(BlendMode::NORMAL),
                ..Default::default()
            },
        )?;

        Ok(Self {
            vbo: gfx.create_vertex_buffer(vec![])?,
            ebo: gfx.create_index_buffer(vec![])?,
            ubo: gfx.create_uniform_buffer(0, vec![0.0; 16])?,
            pipeline,
            clear_options: ClearOptions::new(Color::new(0.1, 0.2, 0.3, 1.0)),
            index: 0,
        })
    }

    fn push(&mut self, renderer: &mut Renderer, batch: &DrawBatch) {
        match batch {
            DrawBatch::Color {
                pipeline,
                vertices,
                indices,
            } => {
                // if indices.len() == 0 {
                //     return;
                // }
                renderer.set_pipeline(&self.pipeline);
                {
                    let mut data = self.vbo.data_ptr().write();
                    data.clear();
                    data.extend(vertices);
                }
                {
                    let mut data = self.ebo.data_ptr().write();
                    data.clear();
                    data.extend(indices);
                }
                {
                    self.ubo.data_mut().copy_from_slice(
                        &glam::Mat4::orthographic_lh(0.0, 800.0, 600.0, 0.0, -1.0, 1.0)
                            .to_cols_array(),
                    );
                }

                renderer.bind_vertex_buffer(&self.vbo);
                renderer.bind_index_buffer(&self.ebo);
                renderer.bind_uniform_buffer(&self.ubo);
                renderer.draw(0, indices.len() as i32);
            }
            _ => {}
        }
    }
}

struct DrawTriangle<'a> {
    colors: [Color; 3],
    points: [(f32, f32); 3],
    draw: &'a mut Draw2,
}

impl<'a> DrawTriangle<'a> {
    fn new(draw: &'a mut Draw2, a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> Self {
        Self {
            colors: [Color::WHITE; 3],
            draw,
            points: [a, b, c],
        }
    }

    fn color(mut self, color: Color) -> Self {
        self.colors.fill(color);
        self
    }

    fn color_vertex(mut self, a: Color, b: Color, c: Color) -> Self {
        self.colors[0] = a;
        self.colors[1] = b;
        self.colors[2] = c;
        self
    }

    fn stroke(self, width: f32) {
        let Self {
            colors: [ca, cb, cc],
            points: [a, b, c],
            draw,
        } = self;

        let mut builder = Path::builder();
        builder
            .begin(a.0, a.1)
            .line_to(b.0, b.1)
            .line_to(c.0, c.1)
            .end(true);
        let path = builder.stroke(width);

        let vertices = {
            let mut vertices = vec![];
            for i in (0..path.vertices.len()).step_by(2) {
                vertices.extend(&[
                    path.vertices[i],
                    path.vertices[i + 1],
                    ca.r,
                    ca.g,
                    ca.b,
                    ca.a,
                ]);
            }
            vertices
        };

        draw.push_color(&DrawInfo2 {
            vertices: &vertices,
            indices: &path.indices,
        });
    }

    fn fill(self) {
        let Self {
            colors: [ca, cb, cc],
            points: [a, b, c],
            draw,
        } = self;

        let indices = [0, 1, 2];
        #[rustfmt::skip]
        let vertices = [
            a.0, a.1, ca.r, ca.g, ca.b, ca.a,
            b.0, b.1, cb.r, cb.g, cb.b, cb.a,
            c.0, c.1, cc.r, cc.g, cc.b, cc.a,
        ];

        draw.push_color(&DrawInfo2 {
            vertices: &vertices,
            indices: &indices,
        });
    }
}

trait ColorShapes {
    fn triangle(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> DrawTriangle;
}

impl ColorShapes for Draw2 {
    fn triangle(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> DrawTriangle {
        DrawTriangle::new(self, a, b, c)
    }
}

//---

struct State {
    renderer: Renderer,
    color_batcher: ColorBatcher2,
}

impl AppState for State {}

impl State {
    pub fn create_draw(&self) -> Draw2 {
        unimplemented!()
    }

    pub fn process_draw(&mut self, draw: &Draw2) {
        self.renderer.begin(Some(&ClearOptions {
            color: draw.background.clone(),
            ..Default::default()
        }));
        draw.batches.iter().for_each(|b| match b {
            DrawBatch::Color { .. } => {
                self.color_batcher.push(&mut self.renderer, b);
            }
            _ => {}
        });
        match &draw.current_batch {
            DrawBatch::Color { .. } => {
                self.color_batcher
                    .push(&mut self.renderer, &draw.current_batch);
            }
            _ => {}
        }
        self.renderer.end();
    }
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .set_plugin(StatsPlugin)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let state = State {
        renderer: gfx.create_renderer(),
        color_batcher: ColorBatcher2::new(gfx).unwrap(),
    };

    state
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = Draw2::new();
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0))
        // .color(Color::RED)
        .color_vertex(Color::RED, Color::GREEN, Color::BLUE)
        .fill();
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0))
        .color(Color::RED)
        // .color_vertex(Color::RED, Color::GREEN, Color::BLUE)
        .stroke(10.0);
    state.process_draw(&draw);
    gfx.render(&state.renderer);
    state.renderer.clear();

    // let mut draw = gfx.create_draw();
    // //
    // // draw.begin(Some(&Color::new(0.1, 0.2, 0.3, 1.0)));
    // draw.begin(None);
    // draw.triangle(400.0, 100.0, 100.0, 500.0, 700.0, 500.0);
    // draw.end();
    // gfx.render(&draw);
}

struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn pre_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
        sb();
        Ok(Default::default())
    }

    fn post_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
        se();
        Ok(Default::default())
    }
}
