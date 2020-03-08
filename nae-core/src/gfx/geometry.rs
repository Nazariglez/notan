use super::color::Color;
use lyon::lyon_algorithms::path::{Builder, Path};
use lyon::lyon_tessellation as tess;
use lyon::math::{point, rect, Point};
use tess::basic_shapes::{
    fill_circle, fill_rectangle, fill_rounded_rectangle, fill_triangle, stroke_circle,
    stroke_rectangle, stroke_rounded_rectangle, stroke_triangle, BorderRadii,
};
use tess::{
    BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator, VertexBuffers,
};

pub use tess::{LineCap, LineJoin};

/// Options to fill a path
pub struct FillConfig {
    /// Maximum allowed distance to the path when building an approximation.
    pub tolerance: f32,
}

impl Default for FillConfig {
    fn default() -> Self {
        Self {
            tolerance: FillOptions::DEFAULT_TOLERANCE,
        }
    }
}

/// Options to stroke a path
pub struct StrokeConfig {
    /// Maximum allowed distance to the path when building an approximation.
    pub tolerance: f32,
    /// What cap uses to start the path
    pub start_cap: LineCap,
    /// What cap uses to end the path
    pub end_cap: LineCap,
    /// What join uses between line segments
    pub line_join: LineJoin,
}

impl Default for StrokeConfig {
    fn default() -> Self {
        Self {
            tolerance: StrokeOptions::DEFAULT_TOLERANCE,
            start_cap: StrokeOptions::DEFAULT_LINE_CAP,
            end_cap: StrokeOptions::DEFAULT_LINE_CAP,
            line_join: StrokeOptions::DEFAULT_LINE_JOIN,
        }
    }
}

enum GeomTypes {
    Circle {
        x: f32,
        y: f32,
        radius: f32,
    },
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    RoundedRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        corner_radius: f32,
    },
    Triangle {
        p1: Point,
        p2: Point,
        p3: Point,
    },
    Path(Path),
}

/// A representation of a geometric object.
/// Useful to cache all the vertices and colors instead of tesselate them every draw frame
/// using the Context2d API.
pub struct Geometry {
    current_path: Option<Builder>,
    stack: Vec<GeomTypes>,
    vertices: Vec<f32>,
    color_vertices: Vec<Color>,
}

impl Geometry {
    /// Create a new Geometry
    pub fn new() -> Self {
        Self {
            current_path: None,
            stack: vec![],
            vertices: vec![],
            color_vertices: vec![],
        }
    }

    /// Returns the cached vertices and color vertices.
    pub fn vertices(&self) -> (&Vec<f32>, &Vec<Color>) {
        (&self.vertices, &self.color_vertices)
    }

    /// Sets the initial point of a path
    pub fn move_to(&mut self, x: f32, y: f32) -> &mut Self {
        self.end_path();

        if self.current_path.is_none() {
            self.current_path = Some(Path::builder());
        }

        if let Some(b) = &mut self.current_path {
            b.move_to(point(x, y));
        }

        self
    }

    /// Creates a straight line to this point from the last one
    pub fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        match &mut self.current_path {
            Some(b) => b.line_to(point(x, y)),
            _ => {
                self.move_to(x, y);
            }
        };

        self
    }

    /// Creates a cubic bezier curve
    pub fn cubic_bezier_to(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> &mut Self {
        if self.current_path.is_none() {
            self.move_to(x1, y1);
        }

        if let Some(b) = &mut self.current_path {
            b.cubic_bezier_to(point(x1, y1), point(x2, y2), point(x3, y3));
        }

        self
    }

    /// Creates a quadratic bezier curve
    pub fn quadratic_bezier_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> &mut Self {
        if self.current_path.is_none() {
            self.move_to(x1, y1);
        }

        if let Some(b) = &mut self.current_path {
            b.quadratic_bezier_to(point(x1, y1), point(x2, y2));
        }

        self
    }

    /// Creates an arc line
    pub fn arc_to(
        &mut self,
        x: f32,
        y: f32,
        _start_angle: f32,
        _end_angle: f32,
        _radius: f32,
    ) -> &mut Self {
        if self.current_path.is_none() {
            self.move_to(x, y);
        }

        if let Some(_b) = &mut self.current_path {
            //TODO add arc support
        }

        self
    }

    /// Create a line between the last point with the first one
    pub fn close_path(&mut self) -> &mut Self {
        if let Some(b) = &mut self.current_path {
            b.close();
        }

        self
    }

    /// Create a circle
    pub fn circle(&mut self, x: f32, y: f32, radius: f32) -> &mut Self {
        self.end_path();
        self.stack.push(GeomTypes::Circle { x, y, radius });
        self
    }

    /// Creates a rectangle
    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> &mut Self {
        self.end_path();
        self.stack.push(GeomTypes::Rect {
            x,
            y,
            width,
            height,
        });
        self
    }

    /// Creates a rectangle with rounded corners
    pub fn rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        corner_radius: f32,
    ) -> &mut Self {
        self.end_path();
        self.stack.push(GeomTypes::RoundedRect {
            x,
            y,
            width,
            height,
            corner_radius,
        });
        self
    }

    /// Creates a triangle
    pub fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> &mut Self {
        self.end_path();
        self.stack.push(GeomTypes::Triangle {
            p1: point(x1, y1),
            p2: point(x2, y2),
            p3: point(x3, y3),
        });
        self
    }

    /// Stroke the geometries created
    pub fn stroke(&mut self, color: Color, strength: f32) -> &mut Self {
        self.stroke_with_config(color, strength, StrokeConfig::default())
    }

    /// Stroke the geometries created using a custom configuration like line caps or line join
    pub fn stroke_with_config(
        &mut self,
        color: Color,
        strength: f32,
        config: StrokeConfig,
    ) -> &mut Self {
        self.end_path();

        let opts = StrokeOptions::tolerance(config.tolerance)
            .with_line_width(strength)
            .with_start_cap(config.start_cap)
            .with_end_cap(config.end_cap)
            .with_line_join(config.line_join);

        let geometries = std::mem::replace(&mut self.stack, vec![]);
        let mut vertices = geometry_stroke(&geometries, opts);

        self.color_vertices
            .append(&mut vec![color; vertices.len() / 2]);
        self.vertices.append(&mut vertices);

        self
    }

    /// Fill the geometries created with a color
    pub fn fill(&mut self, color: Color) -> &mut Self {
        self.fill_with_config(color, FillConfig::default())
    }

    /// Fill the geometries created with a color and with some options
    pub fn fill_with_config(&mut self, color: Color, config: FillConfig) -> &mut Self {
        self.end_path();

        let opts = FillOptions::tolerance(config.tolerance);
        let geometries = std::mem::replace(&mut self.stack, vec![]);
        let mut vertices = geometry_fill(&geometries, opts);
        self.color_vertices
            .append(&mut vec![color; vertices.len() / 2]);
        self.vertices.append(&mut vertices);

        self
    }

    /// Clear all the strokes and fill on this geometry setting it like a empty one
    pub fn clear(&mut self) -> &mut Self {
        self.stack = vec![];
        self.vertices = vec![];
        self.color_vertices = vec![];
        self.current_path = None;
        self
    }

    fn end_path(&mut self) {
        if let Some(b) = self.current_path.take() {
            self.stack.push(GeomTypes::Path(b.build()));
        }
    }
}

fn geometry_stroke(geometries: &Vec<GeomTypes>, opts: StrokeOptions) -> Vec<f32> {
    let mut tessellator = StrokeTessellator::new();
    let mut output: VertexBuffers<(f32, f32), u16> = VertexBuffers::new();
    let mut vertex_builder = BuffersBuilder::new(&mut output, LyonVertex);

    for g in geometries {
        match g {
            GeomTypes::Path(p) => {
                let _result = tessellator
                    .tessellate_path(p.iter(), &opts, &mut vertex_builder)
                    .unwrap();
            }
            GeomTypes::Circle { x, y, radius } => {
                stroke_circle(point(*x, *y), *radius, &opts, &mut vertex_builder).unwrap();
            }
            GeomTypes::Rect {
                x,
                y,
                width,
                height,
            } => {
                stroke_rectangle(&rect(*x, *y, *width, *height), &opts, &mut vertex_builder)
                    .unwrap();
            }
            GeomTypes::Triangle { p1, p2, p3 } => {
                stroke_triangle(*p1, *p2, *p3, &opts, &mut vertex_builder).unwrap();
            }
            GeomTypes::RoundedRect {
                x,
                y,
                width,
                height,
                corner_radius,
            } => {
                stroke_rounded_rectangle(
                    &rect(*x, *y, *width, *height),
                    &BorderRadii::new(
                        *corner_radius,
                        *corner_radius,
                        *corner_radius,
                        *corner_radius,
                    ),
                    &opts,
                    &mut vertex_builder,
                )
                .unwrap();
            }
        }
    }

    lyon_vbuff_to_vertex(output)
}

fn geometry_fill(geometries: &Vec<GeomTypes>, opts: FillOptions) -> Vec<f32> {
    let mut tessellator = FillTessellator::new();
    let mut output: VertexBuffers<(f32, f32), u16> = VertexBuffers::new();
    let mut vertex_builder = BuffersBuilder::new(&mut output, LyonVertex);

    for g in geometries {
        match g {
            GeomTypes::Path(p) => {
                let _result = tessellator
                    .tessellate_path(p.iter(), &opts, &mut vertex_builder)
                    .unwrap();
            }
            GeomTypes::Circle { x, y, radius } => {
                fill_circle(point(*x, *y), *radius, &opts, &mut vertex_builder).unwrap();
            }
            GeomTypes::Rect {
                x,
                y,
                width,
                height,
            } => {
                fill_rectangle(&rect(*x, *y, *width, *height), &opts, &mut vertex_builder).unwrap();
            }
            GeomTypes::Triangle { p1, p2, p3 } => {
                fill_triangle(*p1, *p2, *p3, &opts, &mut vertex_builder).unwrap();
            }
            GeomTypes::RoundedRect {
                x,
                y,
                width,
                height,
                corner_radius,
            } => {
                fill_rounded_rectangle(
                    &rect(*x, *y, *width, *height),
                    &BorderRadii::new(
                        *corner_radius,
                        *corner_radius,
                        *corner_radius,
                        *corner_radius,
                    ),
                    &opts,
                    &mut vertex_builder,
                )
                .unwrap();
            }
        }
    }

    lyon_vbuff_to_vertex(output)
}

// The vertex constructor. This is the object that will be used to create the custom
// vertices from the information provided by the tessellators.
pub struct LyonVertex;
impl tess::VertexConstructor<tess::StrokeVertex, (f32, f32)> for LyonVertex {
    fn new_vertex(&mut self, vertex: tess::StrokeVertex) -> (f32, f32) {
        (vertex.position.x, vertex.position.y)
    }
}

impl tess::VertexConstructor<tess::FillVertex, (f32, f32)> for LyonVertex {
    fn new_vertex(&mut self, vertex: tess::FillVertex) -> (f32, f32) {
        (vertex.position.x, vertex.position.y)
    }
}

pub fn lyon_vbuff_to_vertex(buff: VertexBuffers<(f32, f32), u16>) -> Vec<f32> {
    buff.indices.iter().fold(vec![], |mut acc, v| {
        let v = *v as usize;
        acc.push(buff.vertices[v].0);
        acc.push(buff.vertices[v].1);
        acc
    })
}
