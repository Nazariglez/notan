// // use crate::draw::{Batcher, Draw};
use notan_graphics::buffer::*;
use notan_graphics::commands::*;
use notan_graphics::prelude::*;
use notan_macro::{fragment_shader, vertex_shader};

//
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
        gl_Position = u_projection * vec4(a_pos.x, a_pos.y, 0.0, 1.0);
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

pub(crate) fn create_color_pipeline(
    device: &mut Device,
    fragment: Option<&ShaderSource>,
) -> Result<Pipeline, String> {
    let fragment = fragment.unwrap_or_else(|| &COLOR_FRAGMENT);

    device.create_pipeline(
        &COLOR_VERTEX,
        fragment,
        &[
            VertexAttr::new(0, VertexFormat::Float2),
            VertexAttr::new(1, VertexFormat::Float4),
        ],
        PipelineOptions {
            color_blend: Some(BlendMode::NORMAL),
            ..Default::default()
        },
    )
}
//
pub(crate) fn create_color_pipeline_from_raw(
    device: &mut Device,
    fragment: Option<&[u8]>,
) -> Result<Pipeline, String> {
    unimplemented!()
}

pub(crate) struct ColorBatcher {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    vbo: Buffer<f32>,
    ebo: Buffer<u32>,
    ubo: Buffer<f32>,
    pipeline: Pipeline,
    clear_options: ClearOptions,
    index: usize,
}

impl ColorBatcher {
    pub fn new(device: &mut Device) -> Result<Self, String> {
        // TODO max batch size
        let pipeline = create_color_pipeline(device, None)?;
        Ok(Self {
            vertices: vec![],
            indices: vec![],
            vbo: device.create_vertex_buffer(vec![])?,
            ebo: device.create_index_buffer(vec![])?,
            ubo: device.create_uniform_buffer(0, vec![0.0; 16])?,
            pipeline,
            clear_options: ClearOptions::new(Color::new(0.1, 0.2, 0.3, 1.0)),
            index: 0,
        })
    }

    pub fn push(&mut self, data: ColorData, commands: &mut Vec<Commands>) {
        //flush if needed

        let vertex_offset = self.pipeline.offset();
        let indices_len = data.indices.len();
        let next_indices_len = self.index + indices_len;
        if self.indices.len() < next_indices_len {
            self.indices.resize(next_indices_len, 0);
        }

        let vertices_len = data.vertices.len() / 2;
        let next_vertices_len = self.index * vertex_offset + vertices_len * vertex_offset;
        if self.vertices.len() < next_vertices_len {
            self.vertices.resize(next_vertices_len, 0.0);
        }

        for i in 0..indices_len {
            self.indices[self.index + i] = data.indices[i] + self.index as u32;
        }

        let vertex_len = vertices_len;
        for i in 0..vertex_len {
            let index = (self.index + i) * vertex_offset;
            let n = i * 2;
            self.vertices[index] = data.vertices[n];
            self.vertices[index + 1] = data.vertices[n + 1];

            self.vertices[index + 2] = data.color[0];
            self.vertices[index + 3] = data.color[1];
            self.vertices[index + 4] = data.color[2];
            self.vertices[index + 5] = data.color[3];
        }

        self.index += indices_len;

        // notan_log::info!("original_v: {:?}", data.vertices);
    }

    pub fn flush(
        &mut self,
        pipeline: Option<&Pipeline>,
        projection: &[f32; 16],
        commands: &mut Vec<Commands>,
    ) {
        let pipeline = pipeline.unwrap_or_else(|| &self.pipeline);
        commands.push(Commands::Pipeline {
            id: pipeline.id(),
            options: pipeline.options.clone(),
        });
        //
        // notan_log::info!("{} flush! v: {:?}, i: {:?}", self.index, self.vertices, self.indices);
        // panic!();

        std::mem::swap(&mut self.vertices, &mut self.vbo.data_ptr().write());
        commands.push((&self.vbo).into());

        std::mem::swap(&mut self.indices, &mut self.ebo.data_ptr().write());
        commands.push((&self.ebo).into());

        self.ubo.data_mut().copy_from_slice(projection);
        commands.push((&self.ubo).into());

        let offset = 0;
        let count = self.index as _;
        commands.push(Commands::Draw { offset, count });

        self.index = 0;
    }
}

pub struct ColorData<'a> {
    pub vertices: &'a [f32],
    pub indices: &'a [u32],
    pub pipeline: Option<&'a Pipeline>,
    pub color: &'a [f32; 4],
    pub projection: &'a [f32; 16],
}
