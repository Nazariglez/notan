use nae_core::{
    BaseGfx, BasePipeline, BlendMode, ClearOptions, Color, DrawUsage, GraphicsAPI, PipelineOptions,
};
use nae_gfx::ultraviolet::{projection::lh_yup::orthographic_gl as ortho, Mat4};
use nae_gfx::{
    Graphics, IndexBuffer, Pipeline, Shader, Uniform, VertexAttr, VertexBuffer, VertexFormat,
};
use std::cell::RefMut;
use std::convert::TryInto;

pub const SHADER_COLOR_VERTEX: &'static [u8] = include_bytes!("./shaders/color.vert.spv");
pub const SHADER_COLOR_FRAG: &'static [u8] = include_bytes!("./shaders/color.frag.spv");

pub struct Draw<'gfx> {
    pub gfx: RefMut<'gfx, Graphics>,
    pub depth: f32,
    pub color: Color,
    pub alpha: f32,
    pub blend_mode: Option<BlendMode>,
    pub projection: Option<Mat4>,

    render_projection: Mat4,
    matrix_stack: Vec<Mat4>,
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

        let (width, height) = gfx.size();
        let render_projection = ortho(0.0, width, height, 0.0, -1.0, 1.0);

        Ok(Self {
            gfx,
            clear_options: Default::default(),
            color: Color::WHITE,
            alpha: 1.0,
            depth: 0.0,
            blend_mode: Some(BlendMode::NORMAL),
            current_mode: PaintMode::None,
            matrix_stack: vec![Mat4::identity()],
            color_batcher,
            max_vertices,
            projection: None,
            render_projection,
        })
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.gfx.set_size(width, height);
        self.render_projection = ortho(0.0, width, height, 0.0, -1.0, 1.0);
    }

    pub fn size(&self) -> (f32, f32) {
        self.gfx.size()
    }

    pub fn push(&mut self, matrix: &Mat4) {
        //TODO PR to ultraviolet to avoid clone variables to do ops
        let new_matrix = self.matrix().clone() * matrix.clone();
        self.matrix_stack.push(new_matrix);
    }

    pub fn pop(&mut self) {
        if self.matrix_stack.len() <= 1 {
            return;
        }

        self.matrix_stack.pop();
    }

    pub fn matrix(&mut self) -> &mut Mat4 {
        let len = self.matrix_stack.len();
        &mut self.matrix_stack[len - 1]
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
        PaintMode::Color => draw.color_batcher.flush(
            &mut draw.gfx,
            match &draw.projection {
                Some(p) => p,
                _ => &draw.render_projection,
            },
        ),
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
            projection: match &draw.projection {
                Some(p) => p,
                _ => &draw.render_projection,
            },
            blend: draw.blend_mode,
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
    blend: Option<BlendMode>,
    max_vertices: usize,
    projection: &'data Mat4,
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
    matrix_loc: Uniform,
    index: usize,
}

use nae_core::log;

impl ColorBatcher {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let shader = Shader::new(gfx, SHADER_COLOR_VERTEX, SHADER_COLOR_FRAG)?;
        let pipeline = Pipeline::new(
            gfx,
            &shader,
            PipelineOptions {
                color_blend: Some(BlendMode::NORMAL),
                ..Default::default()
            },
        );

        let matrix_loc = pipeline.uniform_location("u_matrix");

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
            matrix_loc,
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
            self.flush(gfx, data.projection);
        }

        if self.pipeline.options.color_blend != data.blend {
            self.flush(gfx, data.projection);
            self.pipeline.options.color_blend = data.blend;
        }

        // Prepare the vertices and indices for the next draw
        for (i, index) in data.indices.iter().enumerate() {
            self.indices[self.index + i] = self.index as u32 + *index;
        }

        let offset = self.vbo.offset();
        let [r, g, b, a] = data.color.to_rgba();
        let mut index_offset = self.index * offset;
        for (i, _) in data.vertices.iter().enumerate().step_by(3) {
            self.vertices[index_offset + 0] = data.vertices[i + 0];
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

    fn flush(&mut self, gfx: &mut Graphics, projection: &Mat4) {
        if self.index == 0 {
            return;
        }
        gfx.set_pipeline(&self.pipeline);
        gfx.bind_vertex_buffer(&self.vbo, &self.vertices);
        gfx.bind_index_buffer(&self.ibo, &self.indices);
        gfx.bind_uniform(self.matrix_loc, projection);
        gfx.draw(0, self.index as i32);
        self.index = 0;
    }
}
