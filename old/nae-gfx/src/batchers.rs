//TODO https://www.gamedev.net/forums/topic/613184-what-is-the-vertex-limit-number-of-gldrawarrays/

// https://github.com/rustwasm/wasm-bindgen/issues/1389
// WASM32 uses vec because the initial memory is too low for a big array
type VERTICES = Vec<f32>;
type INDICES = Vec<u32>;

use crate::font::{Font, FontManager, FontTextureData};
use crate::pipeline::Pipeline;
use crate::texture::{texture_from_gl_context, Texture, TextureOptions};
use crate::{
    matrix4_identity, matrix4_mul_vector4, DrawData, Graphics, IndexBuffer, MaskMode, Matrix4,
    Uniform, VertexAttr, VertexBuffer, VertexFormat,
};
use glow::TEXTURE_BUFFER;
use nae_core::{
    log, BaseGfx, BasePipeline, BlendMode, ClearOptions, Color, ColorMask, CompareMode, DrawUsage,
    GraphicsAPI, HorizontalAlign, PipelineOptions, StencilAction, StencilOptions, TextureFilter,
    TextureFormat, VerticalAlign,
};

pub(crate) trait BaseBatcher {
    fn flush(
        &mut self,
        gfx: &mut Graphics,
        pipeline: &Option<Pipeline>,
        projection: &Matrix4,
        mask: &MaskMode,
    );
    fn set_mask(&mut self, mask: &MaskMode);
    fn clear_mask(&mut self, gfx: &mut Graphics, mask: &MaskMode, color: Color);
}

/// TextBatcher
pub(crate) struct TextBatcher {
    pipeline: Pipeline,
    vbo: VertexBuffer,
    ibo: IndexBuffer,
    vertices: VERTICES,
    indices: INDICES,
    matrix_loc: Uniform,
    texture_loc: Uniform,
    pub(crate) texture: Texture,
    font: Option<Font>,
    index: usize,
    max_vertices: usize,
    batch_size: usize,
    mask: MaskMode,
    font_manager: FontManager<'static>,
    cached_buffers: Option<Vec<TextBuffer>>,
    current_matrix: Matrix4,
}

impl TextBatcher {
    pub fn text_size(
        &mut self,
        font: &Font,
        text: &str,
        size: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        max_width: Option<f32>,
    ) -> (f32, f32) {
        self.font_manager
            .text_size(font, text, size, h_align, v_align, max_width)
    }

    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let pipeline = Pipeline::from_text_fragment(gfx, Pipeline::TEXT_FRAG)?;
        let matrix_loc = pipeline.uniform_location("u_matrix")?;
        let texture_loc = pipeline.uniform_location("u_texture")?;

        let vertex_buffer = VertexBuffer::new(gfx, DrawUsage::Dynamic)?;

        let index_buffer = IndexBuffer::new(gfx, DrawUsage::Dynamic)?;

        let max_vertices = max_vertices(gfx);
        let batch_size = batch_vertices(pipeline.offset());

        let vertices = vec![0.0; batch_size];
        let indices = vec![0; batch_size / pipeline.offset()];

        let font_manager = FontManager::new(&gfx.gl)?;
        let (width, height) = font_manager.texture_dimensions();

        let texture = texture_from_gl_context(
            &gfx.gl,
            width as _,
            height as _,
            &TextureOptions {
                format: TextureFormat::Red,
                internal_format: TextureFormat::R8,
                min_filter: TextureFilter::Linear,
                mag_filter: TextureFilter::Linear,
            },
        )?;

        Ok(Self {
            pipeline,
            vbo: vertex_buffer,
            ibo: index_buffer,
            matrix_loc,
            texture_loc,
            vertices,
            indices,
            index: 0,
            max_vertices,
            batch_size,
            texture,
            font: None,
            mask: MaskMode::None,
            font_manager,
            cached_buffers: None,
            current_matrix: matrix4_identity(),
        })
    }

    pub fn add_font(&mut self, data: Vec<u8>) -> usize {
        self.font_manager.add(data)
    }

    pub fn push_text(
        &mut self,
        gfx: &mut Graphics,
        font: &Font,
        x: f32,
        y: f32,
        z: f32,
        text: &str,
        size: f32,
        max_width: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        data: DrawData,
    ) {
        let matrix_change = data.matrix != &self.current_matrix;
        let font_change = match &self.font {
            Some(f) => f.raw() != font.raw(),
            None => true,
        };

        let needs_update = matrix_change || font_change;

        if needs_update {
            self.flush(gfx, data.pipeline, data.projection, data.mask);
            self.font = Some(font.clone());
            self.current_matrix = data.matrix.clone();
        }

        self.pipeline.options.color_blend = data.blend;

        let [r, g, b, a] = data.color.to_rgba();
        self.font_manager.queue(
            font,
            x,
            y,
            z,
            text,
            size,
            [r, g, b, a * data.alpha],
            max_width,
            h_align,
            v_align,
        );
    }

    fn push_data(
        &mut self,
        gfx: &mut Graphics,
        pipeline: &Option<Pipeline>,
        projection: &Matrix4,
        mask: &MaskMode,
        data: &TextBuffer,
    ) {
        let next_index = self.index + data.indices.len();
        if next_index >= self.indices.len() {
            self.flush_to_gpu(gfx, pipeline, projection, mask);
        }

        for (i, index) in data.indices.iter().enumerate() {
            self.indices[self.index + i] = self.index as u32 + *index;
        }

        let offset = self.pipeline.offset();
        let [r, g, b, a] = data.color;
        let mut index_offset = self.index * offset;

        let mut uv_index = 0;
        for (i, _) in data.vertices.iter().enumerate().step_by(3) {
            let [x, y, z, _] = matrix4_mul_vector4(
                &self.current_matrix,
                &[
                    data.vertices[i + 0],
                    data.vertices[i + 1],
                    data.vertices[i + 2],
                    1.0,
                ],
            );

            self.vertices[0 + index_offset] = x;
            self.vertices[1 + index_offset] = y;
            self.vertices[2 + index_offset] = z;
            self.vertices[3 + index_offset] = r;
            self.vertices[4 + index_offset] = g;
            self.vertices[5 + index_offset] = b;
            self.vertices[6 + index_offset] = a;
            self.vertices[7 + index_offset] = data.uvs[0 + uv_index];
            self.vertices[8 + index_offset] = data.uvs[1 + uv_index];

            uv_index += 2;
            index_offset += offset;
        }

        self.index += data.indices.len();
    }

    fn check_batch_size(&mut self, gfx: &mut Graphics, data: &DrawData) {
        // let next_size = self.vertices.len() + self.batch_size;
        // let can_be_bigger = next_size < self.max_vertices;
        // if can_be_bigger {
        //     let is_bigger = data.indices.len() > self.indices.len();
        //     let is_more = self.index + data.indices.len() >= self.indices.len();
        //     if is_bigger || is_more {
        //         self.flush(gfx, data.pipeline, data.projection, data.mask);
        //
        //         let index_next_size = next_size / self.pipeline.offset();
        //         log::debug!(
        //             "ColorBatcher -> Increasing vertex_buffer to {} and index_buffer to {}",
        //             next_size,
        //             index_next_size
        //         );
        //
        //         self.vertices.resize(next_size, 0.0);
        //         self.indices.resize(index_next_size, 0);
        //     }
        // }
    }

    fn parse_letters(&mut self, data: Vec<FontTextureData>) {
        let img_ww = self.texture.width();
        let img_hh = self.texture.height();

        let buffers = data
            .iter()
            .map(|data| {
                let sx = data.source_x;
                let sy = data.source_y;
                let sw = data.source_width;
                let sh = data.source_height;

                let x = data.x;
                let y = data.y;
                let x2 = x + sw;
                let y2 = y + sh;

                let z = data.z;

                let sx1 = sx / img_ww;
                let sy1 = sy / img_hh;
                let sx2 = (sx + sw) / img_ww;
                let sy2 = (sy + sh) / img_hh;

                #[rustfmt::skip]
                let buffer = TextBuffer {
                    vertices: [
                        x, y, z,
                        x2, y, z,
                        x, y2, z,
                        x2, y2, z,
                    ],
                    indices: [
                        0, 1, 2, 2, 1, 3
                    ],
                    uvs: [
                        sx1, sy1,
                        sx2, sy1,
                        sx1, sy2,
                        sx2, sy2
                    ],
                    color: data.color
                };

                buffer
            })
            .collect::<Vec<_>>();

        self.cached_buffers = Some(buffers);
    }

    fn flush_to_gpu(
        &mut self,
        gfx: &mut Graphics,
        pipeline: &Option<Pipeline>,
        projection: &Matrix4,
        mask: &MaskMode,
    ) {
        if self.index == 0 {
            return;
        }

        self.set_mask(mask);
        match pipeline {
            Some(pipe) => {
                let tex_loc = batch_uniform(pipe, "TextBatcher", "u_texture").unwrap();
                let mvp_loc = batch_uniform(pipe, "TextBatcher", "u_matrix").unwrap();
                gfx.set_pipeline(pipe);
                gfx.bind_uniform(&mvp_loc, projection);
                gfx.bind_texture(&tex_loc, &self.texture);
            }
            None => {
                gfx.set_pipeline(&self.pipeline);
                gfx.bind_uniform(&self.matrix_loc, projection);
                gfx.bind_texture(&self.texture_loc, &self.texture);
            }
        };

        gfx.bind_vertex_buffer(&self.vbo, &self.vertices);
        gfx.bind_index_buffer(&self.ibo, &self.indices);
        gfx.draw(0, self.index as _);

        self.index = 0;
    }
}

#[derive(Clone, Debug)]
struct TextBuffer {
    vertices: [f32; 12],
    indices: [u32; 6],
    uvs: [f32; 8],
    color: [f32; 4],
}

impl BaseBatcher for TextBatcher {
    fn flush(
        &mut self,
        gfx: &mut Graphics,
        pipeline: &Option<Pipeline>,
        projection: &Matrix4,
        mask: &MaskMode,
    ) {
        if let Some(data) = self.font_manager.process_queue(&gfx.gl, &mut self.texture) {
            self.parse_letters(data);
        }

        match self.cached_buffers.take() {
            Some(buffers) => {
                buffers.iter().for_each(|data| {
                    self.push_data(gfx, pipeline, projection, mask, data);
                });

                self.flush_to_gpu(gfx, pipeline, projection, mask);
                self.cached_buffers = Some(buffers);
            }
            _ => {}
        };
    }

    fn set_mask(&mut self, mask: &MaskMode) {
        if *mask != self.mask {
            apply_mask_to_pipeline(&mut self.pipeline, mask);
            self.mask = *mask;
        }
    }

    fn clear_mask(&mut self, gfx: &mut Graphics, mask: &MaskMode, color: Color) {
        self.set_mask(mask);
        gfx.set_pipeline(&self.pipeline);
        gfx.clear(&ClearOptions {
            stencil: Some(0xff),
            color: Some(color),
            ..Default::default()
        });
    }
}

/// Pattern batcher
pub(crate) struct PatternBatcher {
    pipeline: Pipeline,
    vbo: VertexBuffer,
    ibo: IndexBuffer,
    vertices: VERTICES,
    indices: INDICES,
    matrix_loc: Uniform,
    texture_loc: Uniform,
    texture: Option<Texture>,
    index: usize,
    max_vertices: usize,
    batch_size: usize,
    mask: MaskMode,
}

impl PatternBatcher {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let pipeline = Pipeline::from_pattern_fragment(gfx, Pipeline::PATTERN_FRAG)?;
        let matrix_loc = pipeline.uniform_location("u_matrix")?;
        let texture_loc = pipeline.uniform_location("u_texture")?;

        let vertex_buffer = VertexBuffer::new(gfx, DrawUsage::Dynamic)?;

        let index_buffer = IndexBuffer::new(gfx, DrawUsage::Dynamic)?;

        let max_vertices = max_vertices(gfx);
        let batch_size = batch_vertices(pipeline.offset());

        let vertices = vec![0.0; batch_size];
        let indices = vec![0; batch_size / pipeline.offset()];

        Ok(Self {
            pipeline,
            vbo: vertex_buffer,
            ibo: index_buffer,
            matrix_loc,
            texture_loc,
            vertices,
            indices,
            index: 0,
            max_vertices,
            batch_size,
            texture: None,
            mask: MaskMode::None,
        })
    }

    fn set_texture(
        &mut self,
        gfx: &mut Graphics,
        texture: &Texture,
        pipeline: &Option<Pipeline>,
        projection: &Matrix4,
        mask: &MaskMode,
    ) {
        let needs_update = match &self.texture {
            Some(t) => t.raw() != texture.raw(),
            None => true,
        };

        if needs_update {
            self.flush(gfx, pipeline, projection, mask);
            self.texture = Some(texture.clone());
        }
    }

    pub fn push_data(
        &mut self,
        gfx: &mut Graphics,
        texture: &Texture,
        uvs: &[f32],
        frames: &[f32],
        data: DrawData,
    ) {
        // self.check_batch_size(gfx, &data); //performance is worst with this...
        self.set_texture(gfx, texture, data.pipeline, data.projection, data.mask);
        self.pipeline.options.color_blend = data.blend;

        let next_index = self.index + data.indices.len();
        if next_index >= self.indices.len() {
            self.flush(gfx, data.pipeline, data.projection, data.mask);
        }

        for (i, index) in data.indices.iter().enumerate() {
            self.indices[self.index + i] = self.index as u32 + *index;
        }

        let offset = self.pipeline.offset();
        let [r, g, b, a] = data.color.to_rgba();
        let mut index_offset = self.index * offset;

        let mut uv_index = 0;
        for (i, _) in data.vertices.iter().enumerate().step_by(3) {
            let [x, y, z, _] = matrix4_mul_vector4(
                data.matrix,
                &[
                    data.vertices[i + 0],
                    data.vertices[i + 1],
                    data.vertices[i + 2],
                    1.0,
                ],
            );

            self.vertices[0 + index_offset] = x;
            self.vertices[1 + index_offset] = y;
            self.vertices[2 + index_offset] = z;
            self.vertices[3 + index_offset] = r;
            self.vertices[4 + index_offset] = g;
            self.vertices[5 + index_offset] = b;
            self.vertices[6 + index_offset] = a * data.alpha;
            self.vertices[7 + index_offset] = uvs[0 + uv_index];
            self.vertices[8 + index_offset] = uvs[1 + uv_index];
            self.vertices[9 + index_offset] = frames[0];
            self.vertices[10 + index_offset] = frames[1];
            self.vertices[11 + index_offset] = frames[2];
            self.vertices[12 + index_offset] = frames[3];

            uv_index += 2;
            index_offset += offset;
        }

        self.index += data.indices.len();
    }

    fn check_batch_size(&mut self, gfx: &mut Graphics, data: &DrawData) {
        let next_size = self.vertices.len() + self.batch_size;
        let can_be_bigger = next_size < self.max_vertices;
        if can_be_bigger {
            let is_bigger = data.indices.len() > self.indices.len();
            let is_more = self.index + data.indices.len() >= self.indices.len();
            if is_bigger || is_more {
                self.flush(gfx, data.pipeline, data.projection, data.mask);

                let index_next_size = next_size / self.pipeline.offset();
                log::debug!(
                    "PatternBatcher -> Increasing vertex_buffer to {} and index_buffer to {}",
                    next_size,
                    index_next_size
                );

                self.vertices.resize(next_size, 0.0);
                self.indices.resize(index_next_size, 0);
            }
        }
    }
}

impl BaseBatcher for PatternBatcher {
    fn flush(
        &mut self,
        gfx: &mut Graphics,
        pipeline: &Option<Pipeline>,
        projection: &Matrix4,
        mask: &MaskMode,
    ) {
        if self.index == 0 {
            return;
        }

        self.set_mask(mask);
        if let Some(tex) = &self.texture {
            match pipeline {
                Some(pipe) => {
                    let tex_loc = batch_uniform(pipe, "PatternBatcher", "u_texture").unwrap();
                    let mvp_loc = batch_uniform(pipe, "PatternBatcher", "u_matrix").unwrap();
                    gfx.set_pipeline(pipe);
                    gfx.bind_texture(&tex_loc, tex);
                    gfx.bind_uniform(&mvp_loc, projection);
                }
                None => {
                    gfx.set_pipeline(&self.pipeline);
                    gfx.bind_texture(&self.texture_loc, tex);
                    gfx.bind_uniform(&self.matrix_loc, projection);
                }
            };

            gfx.bind_vertex_buffer(&self.vbo, &self.vertices);
            gfx.bind_index_buffer(&self.ibo, &self.indices);
            gfx.draw(0, self.index as _);
        }

        self.index = 0;
    }

    fn set_mask(&mut self, mask: &MaskMode) {
        if *mask != self.mask {
            apply_mask_to_pipeline(&mut self.pipeline, mask);
            self.mask = *mask;
        }
    }

    fn clear_mask(&mut self, gfx: &mut Graphics, mask: &MaskMode, color: Color) {
        self.set_mask(mask);
        gfx.set_pipeline(&self.pipeline);
        gfx.clear(&ClearOptions {
            stencil: Some(0xff),
            color: Some(color),
            ..Default::default()
        });
    }
}

/// Image batcher
pub(crate) struct ImageBatcher {
    pipeline: Pipeline,
    vbo: VertexBuffer,
    ibo: IndexBuffer,
    vertices: VERTICES,
    vertices_size: usize,
    indices: INDICES,
    indices_size: usize,
    matrix_loc: Uniform,
    texture_loc: Uniform,
    texture: Option<Texture>,
    index: usize,
    max_vertices: usize,
    batch_size: usize,
    mask: MaskMode,
}

impl ImageBatcher {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let pipeline = Pipeline::from_image_fragment(gfx, Pipeline::IMAGE_FRAG)?;
        let matrix_loc = pipeline.uniform_location("u_matrix")?;
        let texture_loc = pipeline.uniform_location("u_texture")?;

        let vertex_buffer = VertexBuffer::new(gfx, DrawUsage::Dynamic)?;
        let index_buffer = IndexBuffer::new(gfx, DrawUsage::Dynamic)?;
        let offset = pipeline.offset();

        let max_vertices = max_vertices(gfx);
        let vertices_size = batch_vertices(offset);
        let indices_size = vertices_size / offset;

        let vertices = vec![0.0; vertices_size];
        let indices = vec![0; indices_size];

        Ok(Self {
            pipeline,
            vbo: vertex_buffer,
            ibo: index_buffer,
            matrix_loc,
            texture_loc,
            vertices,
            vertices_size,
            indices,
            indices_size,
            index: 0,
            max_vertices,
            batch_size: vertices_size,
            texture: None,
            mask: MaskMode::None,
        })
    }

    fn set_texture(
        &mut self,
        gfx: &mut Graphics,
        texture: &Texture,
        pipeline: &Option<Pipeline>,
        projection: &Matrix4,
        mask: &MaskMode,
    ) {
        match &self.texture {
            Some(tex) => {
                if tex.raw() != texture.raw() {
                    self.flush(gfx, pipeline, projection, mask);
                    self.texture = Some(texture.clone());
                }
            }
            None => {
                self.flush(gfx, pipeline, projection, mask);
                self.texture = Some(texture.clone());
            }
        }
    }

    pub fn push_data(
        &mut self,
        gfx: &mut Graphics,
        texture: &Texture,
        uvs: &[f32],
        data: DrawData,
    ) {
        self.check_batch_size(gfx, &data);
        self.set_texture(gfx, texture, data.pipeline, data.projection, data.mask);
        self.pipeline.options.color_blend = data.blend;

        let next_index = self.index + data.indices.len();
        if next_index >= self.indices_size {
            self.flush(gfx, data.pipeline, data.projection, data.mask);
        }

        for (i, index) in data.indices.iter().enumerate() {
            self.indices[self.index + i] = self.index as u32 + *index;
        }

        let offset = self.pipeline.offset();
        let [r, g, b, a] = data.color.to_rgba();
        let mut index_offset = self.index * offset;

        let mut uv_index = 0;
        for (i, _) in data.vertices.iter().enumerate().step_by(3) {
            let [x, y, z, _] = matrix4_mul_vector4(
                data.matrix,
                &[
                    data.vertices[i + 0],
                    data.vertices[i + 1],
                    data.vertices[i + 2],
                    1.0,
                ],
            );

            self.vertices[0 + index_offset] = x;
            self.vertices[1 + index_offset] = y;
            self.vertices[2 + index_offset] = z;
            self.vertices[3 + index_offset] = r;
            self.vertices[4 + index_offset] = g;
            self.vertices[5 + index_offset] = b;
            self.vertices[6 + index_offset] = a * data.alpha;
            self.vertices[7 + index_offset] = uvs[0 + uv_index];
            self.vertices[8 + index_offset] = uvs[1 + uv_index];

            uv_index += 2;
            index_offset += offset;
        }

        self.index += data.indices.len();
    }

    fn check_batch_size(&mut self, gfx: &mut Graphics, data: &DrawData) {
        let data_indices_len = data.indices.len();

        let next_size = self.vertices_size + self.batch_size;
        let can_be_bigger = next_size < self.max_vertices;
        if can_be_bigger {
            let is_bigger = data_indices_len > self.indices_size;
            let is_more = self.index + data_indices_len >= self.indices_size;
            if is_bigger || is_more {
                self.flush(gfx, data.pipeline, data.projection, data.mask);
                self.resize(next_size);
            }
        }
    }

    fn resize(&mut self, size: usize) {
        let index_size = size / self.pipeline.offset();
        log::debug!(
            "ImageBatcher -> Increasing vertex_buffer to {} and index_buffer to {}",
            size,
            index_size
        );

        self.vertices.resize(size, 0.0);
        self.indices.resize(index_size, 0);
        self.vertices_size = size;
        self.indices_size = index_size;
    }
}

impl BaseBatcher for ImageBatcher {
    fn flush(
        &mut self,
        gfx: &mut Graphics,
        pipeline: &Option<Pipeline>,
        projection: &Matrix4,
        mask: &MaskMode,
    ) {
        if self.index == 0 {
            return;
        }

        self.set_mask(mask);
        if let Some(tex) = &self.texture {
            match pipeline {
                Some(pipe) => {
                    let tex_loc = batch_uniform(pipe, "ImageBatcher", "u_texture").unwrap();
                    let mvp_loc = batch_uniform(pipe, "ImageBatcher", "u_matrix").unwrap();
                    gfx.set_pipeline(pipe);
                    gfx.bind_uniform(&mvp_loc, projection);
                    gfx.bind_texture(&tex_loc, tex);
                }
                None => {
                    gfx.set_pipeline(&self.pipeline);
                    gfx.bind_uniform(&self.matrix_loc, projection);
                    gfx.bind_texture(&self.texture_loc, tex);
                }
            };

            gfx.bind_vertex_buffer(&self.vbo, &self.vertices);
            gfx.bind_index_buffer(&self.ibo, &self.indices);
            gfx.draw(0, self.index as _);
        }

        self.index = 0;
    }

    fn set_mask(&mut self, mask: &MaskMode) {
        if *mask != self.mask {
            apply_mask_to_pipeline(&mut self.pipeline, mask);
            self.mask = *mask;
        }
    }

    fn clear_mask(&mut self, gfx: &mut Graphics, mask: &MaskMode, color: Color) {
        self.set_mask(mask);
        gfx.set_pipeline(&self.pipeline);
        gfx.clear(&ClearOptions {
            stencil: Some(0xff),
            color: Some(color),
            ..Default::default()
        });
    }
}

fn apply_mask_to_pipeline(pipeline: &mut Pipeline, mask: &MaskMode) {
    match &mask {
        MaskMode::None => {
            pipeline.options.stencil = None;
            pipeline.options.depth_stencil.write = true;
            pipeline.options.color_mask = ColorMask::ALL;
        }
        MaskMode::Drawing => {
            pipeline.options.stencil = Some(StencilOptions {
                stencil_fail: StencilAction::Keep,
                depth_fail: StencilAction::Keep,
                pass: StencilAction::Replace,
                compare: CompareMode::Always,
                read_mask: 0xff,
                write_mask: 0xff,
                reference: 1,
            });
            pipeline.options.depth_stencil.write = false;
            pipeline.options.color_mask = ColorMask::NONE;
        }
        MaskMode::Masking => {
            pipeline.options.stencil = Some(StencilOptions {
                stencil_fail: StencilAction::Keep,
                depth_fail: StencilAction::Keep,
                pass: StencilAction::Replace,
                compare: CompareMode::Equal,
                read_mask: 0xff,
                write_mask: 0x00,
                reference: 1,
            });
            pipeline.options.depth_stencil.write = true;
            pipeline.options.color_mask = ColorMask::ALL;
        }
    }
}

/// Color batcher
pub(crate) struct ColorBatcher {
    pipeline: Pipeline,
    vbo: VertexBuffer,
    ibo: IndexBuffer,
    vertices: VERTICES,
    indices: INDICES,
    matrix_loc: Uniform,
    index: usize,
    max_vertices: usize,
    batch_size: usize,
    mask: MaskMode,
}

impl ColorBatcher {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let pipeline = Pipeline::from_color_fragment(gfx, Pipeline::COLOR_FRAG)?;
        let matrix_loc = pipeline.uniform_location("u_matrix")?;

        let vertex_buffer = VertexBuffer::new(&gfx, DrawUsage::Dynamic)?;

        let index_buffer = IndexBuffer::new(gfx, DrawUsage::Dynamic)?;

        let max_vertices = max_vertices(gfx);
        let batch_size = batch_vertices(pipeline.offset());

        let vertices = vec![0.0; batch_size];
        let indices = vec![0; batch_size / pipeline.offset()];

        Ok(Self {
            pipeline,
            vbo: vertex_buffer,
            ibo: index_buffer,
            matrix_loc,
            vertices,
            indices,
            index: 0,
            max_vertices,
            batch_size,
            mask: MaskMode::None,
        })
    }

    fn check_batch_size(&mut self, gfx: &mut Graphics, data: &DrawData) {
        let next_size = self.vertices.len() + self.batch_size;
        let can_be_bigger = next_size < self.max_vertices;
        if can_be_bigger {
            let is_bigger = data.indices.len() > self.indices.len();
            let is_more = self.index + data.indices.len() >= self.indices.len();
            if is_bigger || is_more {
                self.flush(gfx, data.pipeline, data.projection, data.mask);

                let index_next_size = next_size / self.pipeline.offset();
                log::debug!(
                    "ColorBatcher -> Increasing vertex_buffer to {} and index_buffer to {}",
                    next_size,
                    index_next_size
                );

                self.vertices.resize(next_size, 0.0);
                self.indices.resize(index_next_size, 0);
            }
        }
    }

    pub fn push_data(&mut self, gfx: &mut Graphics, data: DrawData) {
        self.check_batch_size(gfx, &data);
        self.pipeline.options.color_blend = data.blend;

        // Check if the batch is bigger than the max_vertices allowed and split it
        if data.indices.len() > self.indices.len() {
            return self.split_batch(gfx, data);
        }

        // Flush if we reach the end of this batch
        let next_index = self.index + data.indices.len();
        if next_index >= self.indices.len() {
            self.flush(gfx, data.pipeline, data.projection, data.mask);
        }

        // Push the vertices on the current batch
        self.push_vertices(
            data.indices,
            data.vertices,
            &data.color,
            data.matrix,
            data.alpha,
        );
    }

    fn split_batch(&mut self, gfx: &mut Graphics, data: DrawData) {
        // TODO this doesn't care about indices...

        let mut indices = vec![0; self.indices.len()];
        let iterations = (data.indices.len() / self.indices.len()) + 1;

        for i in 0..iterations {
            let start = i * self.indices.len();
            let end = (start + self.indices.len()).min(data.indices.len());
            for (i, v) in (start..end).enumerate() {
                indices[i] = (v - start) as u32;
            }

            self.push_vertices(
                &indices[0..end - start],
                &data.vertices[start * 3..end * 3],
                &data.color,
                data.matrix,
                data.alpha,
            );

            self.flush(gfx, data.pipeline, data.projection, data.mask);
        }
    }

    fn push_vertices(
        &mut self,
        indices: &[u32],
        vertices: &[f32],
        color: &Color,
        matrix: &Matrix4,
        alpha: f32,
    ) {
        for (i, index) in indices.iter().enumerate() {
            self.indices[self.index + i] = self.index as u32 + *index;
        }

        let offset = self.pipeline.offset();
        let [r, g, b, a] = color.to_rgba();
        let mut index_offset = self.index * offset;

        for (i, _) in vertices.iter().enumerate().step_by(3) {
            let [x, y, z, _] = matrix4_mul_vector4(
                matrix,
                &[vertices[i + 0], vertices[i + 1], vertices[i + 2], 1.0],
            );

            self.vertices[index_offset + 0] = x;
            self.vertices[index_offset + 1] = y;
            self.vertices[index_offset + 2] = z;
            self.vertices[index_offset + 3] = r;
            self.vertices[index_offset + 4] = g;
            self.vertices[index_offset + 5] = b;
            self.vertices[index_offset + 6] = a * alpha;

            index_offset += offset;
        }

        self.index += indices.len();
    }
}

impl BaseBatcher for ColorBatcher {
    fn flush(
        &mut self,
        gfx: &mut Graphics,
        pipeline: &Option<Pipeline>,
        projection: &Matrix4,
        mask: &MaskMode,
    ) {
        if self.index == 0 {
            return;
        }

        self.set_mask(mask);

        match pipeline {
            Some(pipe) => {
                let mvp_loc = batch_uniform(pipe, "ColorBatcher", "u_matrix").unwrap();
                gfx.set_pipeline(pipe);
                gfx.bind_uniform(&mvp_loc, projection);
            }
            None => {
                gfx.set_pipeline(&self.pipeline);
                gfx.bind_uniform(&self.matrix_loc, projection);
            }
        };

        gfx.bind_vertex_buffer(&self.vbo, &self.vertices);
        gfx.bind_index_buffer(&self.ibo, &self.indices);
        gfx.draw(0, self.index as i32);
        self.index = 0;
    }

    fn set_mask(&mut self, mask: &MaskMode) {
        if *mask != self.mask {
            apply_mask_to_pipeline(&mut self.pipeline, mask);
            self.mask = *mask;
        }
    }

    fn clear_mask(&mut self, gfx: &mut Graphics, mask: &MaskMode, color: Color) {
        self.set_mask(mask);
        gfx.set_pipeline(&self.pipeline);
        gfx.clear(&ClearOptions {
            stencil: Some(0xff),
            color: Some(color),
            ..Default::default()
        });
    }
}

//https://webglfundamentals.org/webgl/lessons/webgl-indexed-vertices.html
#[inline]
fn max_vertices(gfx: &Graphics) -> usize {
    match gfx.api() {
        GraphicsAPI::WebGl => std::u16::MAX as usize,
        _ => std::u32::MAX as usize,
    }
}

#[inline]
fn batch_vertices(offset: usize) -> usize {
    let offset = offset as f32;
    let max = std::u16::MAX as usize;
    let size = {
        let mut n = max;
        while n > 0 {
            let nf = n as f32;
            if nf % offset == 0.0 && nf % 3.0 == 0.0 {
                break;
            }
            n -= 1;
        }
        n
    };

    size
}

fn batch_uniform(pipeline: &Pipeline, batcher_name: &str, id: &str) -> Result<Uniform, String> {
    pipeline
        .uniform_location(id)
        .map_err(|e| format!("{} expect {} uniform: {}", batcher_name, id, e))
}
