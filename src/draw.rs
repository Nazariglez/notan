use nae_core::{BaseGfx, ClearOptions, Color, DrawUsage, GraphicsAPI, PipelineOptions};
use nae_gfx::{Graphics, IndexBuffer, Pipeline, Shader, VertexAttr, VertexBuffer, VertexFormat};
use std::cell::RefMut;
use std::convert::TryInto;

pub const SHADER_COLOR_VERTEX: &'static [u8] = include_bytes!("./shaders/color.vert.spv");
pub const SHADER_COLOR_FRAG: &'static [u8] = include_bytes!("./shaders/color.frag.spv");

pub struct Draw<'gfx> {
    pub gfx: RefMut<'gfx, Graphics>,
    pub depth: f32,
    pub color: Color,
    pub alpha: f32,

    clear_options: ClearOptions,
    color_batcher: ColorBatcher,
    max_vertices: usize,
    current_mode: PaintMode,
}

impl<'gfx> Draw<'gfx> {
    pub fn new(mut gfx: RefMut<'gfx, Graphics>) -> Result<Self, String> {
        let color_batcher = ColorBatcher::new(&mut gfx)?;
        let max_vertices = match gfx.api() {
            GraphicsAPI::WebGl => std::u16::MAX as usize,
            _ => std::u32::MAX as usize,
        };

        Ok(Self {
            gfx,
            clear_options: Default::default(),
            color: Color::WHITE,
            alpha: 1.0,
            depth: 0.0,
            current_mode: PaintMode::None,
            color_batcher,
            max_vertices,
        })
    }

    pub fn begin(&mut self, color: Color) {
        self.clear_options.color = Some(color);
        self.gfx.begin(&self.clear_options);
    }

    pub fn end(&mut self) {
        paint_mode(self, PaintMode::None);
        self.gfx.end();
    }

    pub fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        paint_mode(self, PaintMode::Color);

        #[rustfmt::skip]
        draw_color(
            self,
            &[
                x1, y1, self.depth,
                x2, y2, self.depth,
                x3, y3, self.depth
            ],
            &[0, 1, 2]
        );
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        paint_mode(self, PaintMode::Color);

        let x2 = x + width;
        let y2 = y + height;

        #[rustfmt::skip]
        draw_color(
            self,
            &[
                x, y, self.depth,
                x2, y, self.depth,
                x, y2, self.depth,
                x2, y2, self.depth,
            ],
            &[0, 1, 2, 2, 1, 3]
        );
    }
}

fn flush(draw: &mut Draw) {
    match draw.current_mode {
        PaintMode::Color => draw.color_batcher.flush(&mut draw.gfx),
        _ => {}
    }
}

fn paint_mode(draw: &mut Draw, mode: PaintMode) {
    if draw.current_mode == mode {
        return;
    }

    flush(draw);
    draw.current_mode = mode;
}

fn draw_color(draw: &mut Draw, vertices: &[f32], indices: &[u32]) {
    draw.color_batcher.push_vertices(
        &mut draw.gfx,
        DrawData {
            vertices,
            indices,
            color: draw.color,
            alpha: draw.alpha,
            max_vertices: draw.max_vertices,
        },
    );
}

#[derive(Debug, PartialEq)]
enum PaintMode {
    None,
    Color,
}

struct DrawData<'data> {
    vertices: &'data [f32],
    indices: &'data [u32],
    color: Color,
    alpha: f32,
    max_vertices: usize,
}

//TODO https://www.gamedev.net/forums/topic/613184-what-is-the-vertex-limit-number-of-gldrawarrays/
const MAX_ARRAY_LEN: usize = 65535; //std::u16::MAX as usize;

// https://github.com/rustwasm/wasm-bindgen/issues/1389
// WASM32 uses vec because the initial memory is too low for a big array
#[cfg(not(target_arch = "wasm32"))]
type VERTICES = [f32; MAX_ARRAY_LEN];

#[cfg(target_arch = "wasm32")]
type VERTICES = Vec<f32>;

#[cfg(not(target_arch = "wasm32"))]
type INDICES = [u32; MAX_ARRAY_LEN / 7];

#[cfg(target_arch = "wasm32")]
type INDICES = Vec<u32>;

/// Color batcher
struct ColorBatcher {
    pipeline: Pipeline,
    vbo: VertexBuffer,
    ibo: IndexBuffer,
    vertices: VERTICES,
    indices: INDICES,
    index: usize,
}

use nae_core::log;

impl ColorBatcher {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let shader = Shader::new(gfx, SHADER_COLOR_VERTEX, SHADER_COLOR_FRAG)?;
        let pipeline = Pipeline::new(gfx, &shader, PipelineOptions::default());
        let vertex_buffer = VertexBuffer::new(
            &gfx,
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float4),
            ],
            DrawUsage::Dynamic,
        )?;

        let index_buffer = IndexBuffer::new(gfx, DrawUsage::Dynamic)?;

        #[cfg(not(target_arch = "wasm32"))]
        let vertices = [0.0; MAX_ARRAY_LEN];

        #[cfg(target_arch = "wasm32")]
        let vertices = vec![0.0; MAX_ARRAY_LEN];

        #[cfg(not(target_arch = "wasm32"))]
        let indices = [0; MAX_ARRAY_LEN / 7];

        #[cfg(target_arch = "wasm32")]
        let indices = vec![0; MAX_ARRAY_LEN / 7];

        Ok(Self {
            pipeline,
            vbo: vertex_buffer,
            ibo: index_buffer,
            vertices,
            indices,
            index: 0,
        })
    }

    fn push_vertices(&mut self, gfx: &mut Graphics, data: DrawData) {
        //TODO check if the call needs split

        let vertices_len = data.indices.len();
        let next_index = self.index + vertices_len;
        if next_index >= data.max_vertices {
            self.flush(gfx);
        }

        for (i, index) in data.indices.iter().enumerate() {
            self.indices[self.index + i] = self.index as u32 + *index;
        }

        let offset = self.vbo.offset();
        let [r, g, b, a] = data.color.to_rgba();
        let mut index_offset = self.index * offset;
        for (i, _) in data.vertices.iter().enumerate().step_by(3) {
            self.vertices[index_offset] = data.vertices[i];
            self.vertices[index_offset + 1] = data.vertices[i + 1];
            self.vertices[index_offset + 2] = data.vertices[i + 2];
            self.vertices[index_offset + 3] = r;
            self.vertices[index_offset + 4] = g;
            self.vertices[index_offset + 5] = b;
            self.vertices[index_offset + 6] = a * data.alpha;

            index_offset += offset;
        }

        self.index += vertices_len;
    }

    fn flush(&mut self, gfx: &mut Graphics) {
        gfx.set_pipeline(&self.pipeline);
        gfx.bind_vertex_buffer(&self.vbo, &self.vertices);
        gfx.bind_index_buffer(&self.ibo, &self.indices);
        gfx.draw(0, self.index as i32);
        self.index = 0;
    }
}
