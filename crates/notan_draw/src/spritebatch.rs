use crate::sprite::{Sprite, SpriteId};
use notan_gfx::{
    BindGroup, BindGroupLayout, BindingType, BlendMode, Buffer, DrawFrame, Gfx, IndexFormat,
    NotanRenderPipeline, RenderPipeline, Renderer, VertexFormat, VertexLayout,
};
use notan_math::{Mat4, Vec2};
use std::collections::HashMap;

// language=wgsl
const SHADER: &str = r#"
struct Transform {
    mvp: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> transform: Transform;

struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) uvs: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uvs: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.uvs = model.uvs;
    out.pos = transform.mvp * vec4<f32>(model.pos, 0.0, 1.0);
    return out;
}

@group(0) @binding(1)
var t_texture: texture_2d<f32>;
@group(0) @binding(2)
var s_texture: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_texture, s_texture, in.uvs);
}
"#;

struct BatchData {
    sprite: Sprite,
    bind_group: Option<BindGroup>,
    start_element: usize,
    end_element: Option<usize>,
}

pub struct SpriteBatch {
    batches: Vec<BatchData>,
    cached_bind_groups: HashMap<SpriteId, BindGroup>,
    pip: RenderPipeline,
    vbo: Buffer,
    ebo: Buffer,
    ubo: Buffer,
    vbo_data: Vec<f32>,
    ebo_data: Vec<u32>,
    projection: Mat4,
    dirty_upload: bool,
    dirty_resize: bool,
    dirty_projection: bool,
    max_elements: usize,
    element_index: usize,
}

impl SpriteBatch {
    pub fn new(projection: Mat4, gfx: &mut Gfx) -> Result<Self, String> {
        let pip = gfx
            .create_render_pipeline(SHADER)
            .with_vertex_layout(
                VertexLayout::new()
                    .with_attr(0, VertexFormat::Float32x2)
                    .with_attr(1, VertexFormat::Float32x2),
            )
            .with_bind_group_layout(
                BindGroupLayout::new()
                    .with_entry(BindingType::uniform(0).with_vertex_visibility(true))
                    .with_entry(BindingType::texture(1).with_fragment_visibility(true))
                    .with_entry(BindingType::sampler(2).with_fragment_visibility(true)),
            )
            .with_index_format(IndexFormat::UInt32)
            .with_blend_mode(BlendMode::NORMAL)
            .build()?;

        let max_elements = 256;

        let vbo_data: Vec<f32> = vec![0.0; max_elements * 16];
        let vbo = gfx
            .create_vertex_buffer(&vbo_data)
            .with_write_flag(true)
            .build()?;

        let ebo_data: Vec<u32> = vec![0; max_elements * 6];
        let ebo = gfx
            .create_index_buffer(&ebo_data)
            .with_write_flag(true)
            .build()?;

        let ubo = gfx
            .create_uniform_buffer(projection.as_ref())
            .with_write_flag(true)
            .build()?;

        Ok(Self {
            batches: vec![],
            cached_bind_groups: Default::default(),
            pip,
            vbo,
            ebo,
            ubo,
            vbo_data,
            ebo_data,
            dirty_upload: false,
            dirty_resize: false,
            dirty_projection: false,
            max_elements,
            element_index: 0,
            projection,
        })
    }

    pub fn elements(&self) -> usize {
        self.element_index
    }

    fn increase_data_buffers(&mut self) {
        self.max_elements *= 2;
        self.vbo_data.resize(self.max_elements * 16, 0.0);
        self.ebo_data.resize(self.max_elements * 6, 0);
        self.dirty_resize = true;
    }

    fn need_new_batch(&self, sprite: &Sprite) -> bool {
        self.batches
            .last()
            .map_or(true, |batch| &batch.sprite != sprite)
    }

    pub fn draw(&mut self, sprite: &Sprite, pos: Vec2) {
        // TODO instead of "pos" pass Transform2d which converts into Mat3
        let current_index = self.element_index;
        let next_index = self.element_index + 1;

        if self.need_new_batch(sprite) {
            // set the end of the batch
            if let Some(batch) = self.batches.last_mut() {
                batch.end_element = Some(current_index);
            }

            // starts a new batch
            self.batches.push(BatchData {
                sprite: sprite.clone(),
                bind_group: self.cached_bind_groups.get(&sprite.id()).cloned(),
                start_element: current_index,
                end_element: None,
            });
        }

        if self.max_elements < next_index {
            self.increase_data_buffers();
        }

        let Vec2 { x: x1, y: y1 } = pos;
        let Vec2 { x: x2, y: y2 } = pos + sprite.size();

        #[rustfmt::skip]
            let vertices = [
            x1, y1, 0.0, 0.0,
            x2, y1, 1.0, 0.0,
            x1, y2, 0.0, 1.0,
            x2, y2, 1.0, 1.0
        ];

        let vbo_index_start = current_index * 16;
        let vbo_index_end = vbo_index_start + 16;
        self.vbo_data
            .splice(vbo_index_start..vbo_index_end, vertices);

        let ebo_index_start = current_index * 6;
        let ebo_index_end = ebo_index_start + 6;
        let i = (current_index * 4) as u32; //4 vertices per element
        #[rustfmt::skip]
            let indices = [
            0+i, 1+i, 2+i,
            1+i, 3+i, 2+i,
        ];

        self.ebo_data
            .splice(ebo_index_start..ebo_index_end, indices);

        self.dirty_upload = true;
        self.element_index = next_index;
    }

    fn resize_gpu_buffers(&mut self, gfx: &mut Gfx) -> Result<(), String> {
        if !self.dirty_resize {
            return Ok(());
        }

        log::info!(
            "Creating a new Vertex Buffer with size: {}",
            self.vbo_data.len()
        );
        let vbo = gfx
            .create_vertex_buffer(&self.vbo_data)
            .with_write_flag(true)
            .build()?;

        self.vbo = vbo;

        log::info!(
            "Creating a new Index Buffer with size: {}",
            self.ebo_data.len()
        );
        let ebo = gfx
            .create_index_buffer(&self.ebo_data)
            .with_write_flag(true)
            .build()?;

        self.ebo = ebo;

        self.dirty_resize = false;
        self.dirty_upload = false;

        Ok(())
    }

    fn upload_gpu_buffers(&mut self, gfx: &mut Gfx) -> Result<(), String> {
        if !self.dirty_upload {
            return Ok(());
        }

        log::info!("Uploading buffer to gpu");
        gfx.write_buffer(&self.vbo)
            .with_data(&self.vbo_data)
            .build()?;
        gfx.write_buffer(&self.ebo)
            .with_data(&self.ebo_data)
            .build()?;

        self.dirty_upload = false;

        Ok(())
    }

    fn upload_gpu_projection(&mut self, gfx: &mut Gfx) -> Result<(), String> {
        if !self.dirty_projection {
            return Ok(());
        }

        gfx.write_buffer(&self.ubo)
            .with_data(self.projection.as_ref())
            .build()?;

        self.dirty_projection = false;
        Ok(())
    }

    pub fn set_projection(&mut self, projection: Mat4) {
        self.projection = projection;
        self.dirty_projection = true;
    }

    /// Upload buffers and render everything that's batched
    pub fn flush<'a>(
        &'a mut self,
        gfx: &mut Gfx,
        frame: &DrawFrame,
        mut renderer: Renderer<'a>,
    ) -> Result<(), String> {
        self.resize_gpu_buffers(gfx)?;
        self.upload_gpu_buffers(gfx)?;
        self.upload_gpu_projection(gfx)?;

        for batch in &mut self.batches {
            // if the batch do not have bind group, create, assign and cache it
            if batch.bind_group.is_none() {
                let bind_group = gfx
                    .create_bind_group()
                    .with_layout(self.pip.bind_group_layout_id(0)?)
                    .with_uniform(0, &self.ubo)
                    .with_texture(1, batch.sprite.texture())
                    .with_sampler(2, batch.sprite.sampler())
                    .build()?;

                batch.bind_group = Some(bind_group.clone());
                self.cached_bind_groups
                    .insert(batch.sprite.id(), bind_group);
            }

            debug_assert!(
                batch.bind_group.is_some(),
                "This should not happen. BindGroup is not present for batch"
            );

            // draw the batch
            let start = (batch.start_element as u32) * 6;
            let end = (batch.end_element.unwrap_or(self.element_index) as u32) * 6;

            renderer
                .begin_pass()
                .pipeline(&self.pip)
                .buffers(&[&self.vbo, &self.ebo])
                .bindings(&[batch.bind_group.as_ref().unwrap()])
                .draw(start..end);
        }

        gfx.render(frame, &renderer)?;

        Ok(())
    }

    /// Reset the batches to start drawing again
    pub fn reset(&mut self) {
        self.element_index = 0;
        self.batches.clear();
    }

    /// Clear everything in memory
    pub fn clear(&mut self) {
        self.reset();
        self.cached_bind_groups.clear();
        self.vbo_data.clear();
        self.ebo_data.clear();
    }
}
