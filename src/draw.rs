use nae_core::{BaseGfx, ClearOptions, Color, DrawUsage, GraphicsAPI, PipelineOptions};
use nae_gfx::{Graphics, IndexBuffer, Pipeline, Shader, VertexAttr, VertexBuffer, VertexFormat};
use std::cell::RefMut;
use std::convert::TryInto;

pub const SHADER_COLOR_VERTEX: &'static [u8] = include_bytes!("./shaders/color.vert.spv");
pub const SHADER_COLOR_FRAG: &'static [u8] = include_bytes!("./shaders/color.frag.spv");

pub struct Draw<'gfx> {
    pub gfx: RefMut<'gfx, Graphics>,
    clear_options: ClearOptions,
    current_color: Color,
    current_alpha: f32,
    color_batcher: ColorBatcher,
    max_vertices: usize,
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
            current_color: Color::WHITE,
            current_alpha: 1.0,
            color_batcher,
            max_vertices,
        })
    }

    pub fn begin(&mut self, color: Color) {
        self.clear_options.color = Some(color);

        self.gfx.begin(&self.clear_options);
    }

    pub fn end(&mut self) {
        self.gfx.end();
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.paint_mode(PaintMode::Color);

        self.color_batcher.push_vertices(
            &mut self.gfx,
            DrawData {
                vertices: &[x1, y1, x2, y2, x3, y3],
                indices: &[0, 1, 2],
                color: self.current_color,
                alpha: self.current_alpha,
                max_vertices: self.max_vertices,
            },
        );
    }

    fn paint_mode(&mut self, mode: PaintMode) {
        //TODO
    }
}

enum PaintMode {
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

/// Color batcher
struct ColorBatcher {
    pipeline: Pipeline,
    vbo: VertexBuffer,
    ibo: IndexBuffer,
    vertices: Vec<f32>,
    indices: Vec<u32>,
    index: usize,
}

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

        let max_capacity = (std::u32::MAX).try_into().unwrap();
        let vertices = Vec::with_capacity(max_capacity);
        let indices = Vec::with_capacity(max_capacity);

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
            self.flush(gfx, vertices_len);
        }

        let stride = self.vbo.stride();
        let [r, g, b, a] = data.color.to_rgba();
        let mut index_offset = self.index * stride;
        for (i, v_index) in data.indices.iter().enumerate() {
            self.indices[index_offset] = *v_index;
            self.vertices[index_offset] = data.vertices[i];
            self.vertices[index_offset + 1] = data.vertices[i + 1];
            self.vertices[index_offset + 2] = data.vertices[i + 2];
            self.vertices[index_offset + 3] = r;
            self.vertices[index_offset + 4] = g;
            self.vertices[index_offset + 5] = b;
            self.vertices[index_offset + 6] = a * data.alpha;
            index_offset += stride;
        }

        self.index += vertices_len;
        self.flush(gfx, vertices_len);
    }

    fn flush(&mut self, gfx: &mut Graphics, count: usize) {
        self.index = 0;

        gfx.set_pipeline(&self.pipeline);
        gfx.bind_vertex_buffer(&self.vbo, &self.vertices);
        gfx.bind_index_buffer(&self.ibo, &self.indices);
        gfx.draw(0, count as i32);
    }
}
