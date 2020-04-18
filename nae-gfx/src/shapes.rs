use lyon::lyon_tessellation::basic_shapes::stroke_triangle;
use lyon::lyon_tessellation::{
    BuffersBuilder, FillTessellator, FillVertexConstructor, StrokeAttributes, StrokeOptions,
    StrokeTessellator, VertexBuffers,
};
use lyon::math::Point;
use nae_core::Vertex;

pub(crate) struct ShapeTessellator {
    fill: FillTessellator,
    stroke: StrokeTessellator,
}

impl ShapeTessellator {
    pub fn new() -> Self {
        ShapeTessellator {
            fill: FillTessellator::new(),
            stroke: StrokeTessellator::new(),
        }
    }

    pub fn stroke_triangle(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        line_width: f32,
        depth: f32,
    ) -> (Vec<f32>, Vec<u32>) {
        let opts = StrokeOptions::default().with_line_width(line_width);
        let mut output: VertexBuffers<[f32; 3], u32> = VertexBuffers::new();
        stroke_triangle(
            Point::new(x1, y1),
            Point::new(x2, y2),
            Point::new(x3, y3),
            &opts,
            &mut BuffersBuilder::new(&mut output, |pos: Point, _: StrokeAttributes| {
                [pos.x, pos.y, depth]
            }),
        );

        vertices_and_indices(output)
    }
}

fn vertices_and_indices(buffer: VertexBuffers<[f32; 3], u32>) -> (Vec<f32>, Vec<u32>) {
    let VertexBuffers { vertices, indices } = buffer;
    let aux_buffer: Vec<f32> = vertices.iter().flat_map(|v| v.iter().cloned()).collect();
    (aux_buffer, indices)
}
