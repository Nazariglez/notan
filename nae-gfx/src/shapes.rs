use lyon::lyon_tessellation::basic_shapes::{
    fill_circle, fill_rounded_rectangle, stroke_circle, stroke_rectangle, stroke_rounded_rectangle,
    stroke_triangle, BorderRadii,
};
use lyon::lyon_tessellation::math::Rect;
use lyon::lyon_tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, FillVertexConstructor, StrokeAttributes,
    StrokeOptions, StrokeTessellator, VertexBuffers,
};
use lyon::math::{rect, Point};

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

    pub fn circle(&mut self, x: f32, y: f32, radius: f32, depth: f32) -> (Vec<f32>, Vec<u32>) {
        let opts = FillOptions::default();
        let mut output: VertexBuffers<[f32; 3], u32> = VertexBuffers::new();
        fill_circle(
            Point::new(x, y),
            radius,
            &opts,
            &mut BuffersBuilder::new(&mut output, |pos: Point| [pos.x, pos.y, depth]),
        );

        vertices_and_indices(output)
    }

    pub fn stroke_circle(
        &mut self,
        x: f32,
        y: f32,
        radius: f32,
        line_width: f32,
        depth: f32,
    ) -> (Vec<f32>, Vec<u32>) {
        let opts = StrokeOptions::default().with_line_width(line_width);
        let mut output: VertexBuffers<[f32; 3], u32> = VertexBuffers::new();
        stroke_circle(
            Point::new(x, y),
            radius,
            &opts,
            &mut BuffersBuilder::new(&mut output, |pos: Point, _: StrokeAttributes| {
                [pos.x, pos.y, depth]
            }),
        );

        vertices_and_indices(output)
    }

    pub fn stroke_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        line_width: f32,
        depth: f32,
    ) -> (Vec<f32>, Vec<u32>) {
        let opts = StrokeOptions::default().with_line_width(line_width);
        let mut output: VertexBuffers<[f32; 3], u32> = VertexBuffers::new();
        stroke_rectangle(
            &rect(x, y, width, height),
            &opts,
            &mut BuffersBuilder::new(&mut output, |pos: Point, _: StrokeAttributes| {
                [pos.x, pos.y, depth]
            }),
        );

        vertices_and_indices(output)
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

    pub fn stroke_rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        line_width: f32,
        corner_radius: f32,
        depth: f32,
    ) -> (Vec<f32>, Vec<u32>) {
        let opts = StrokeOptions::default().with_line_width(line_width);
        let mut output: VertexBuffers<[f32; 3], u32> = VertexBuffers::new();
        stroke_rounded_rectangle(
            &rect(x, y, width, height),
            &BorderRadii::new(corner_radius, corner_radius, corner_radius, corner_radius),
            &opts,
            &mut BuffersBuilder::new(&mut output, |pos: Point, _: StrokeAttributes| {
                [pos.x, pos.y, depth]
            }),
        );

        vertices_and_indices(output)
    }

    pub fn rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        corner_radius: f32,
        depth: f32,
    ) -> (Vec<f32>, Vec<u32>) {
        let opts = FillOptions::default();
        let mut output: VertexBuffers<[f32; 3], u32> = VertexBuffers::new();
        fill_rounded_rectangle(
            &rect(x, y, width, height),
            &BorderRadii::new(corner_radius, corner_radius, corner_radius, corner_radius),
            &opts,
            &mut BuffersBuilder::new(&mut output, |pos: Point| [pos.x, pos.y, depth]),
        );

        vertices_and_indices(output)
    }
}

fn vertices_and_indices(buffer: VertexBuffers<[f32; 3], u32>) -> (Vec<f32>, Vec<u32>) {
    let VertexBuffers { vertices, indices } = buffer;
    let aux_buffer: Vec<f32> = vertices.iter().flat_map(|v| v.iter().cloned()).collect();
    (aux_buffer, indices)
}
