use lyon::math::{point, Point};
use lyon::path::builder::*;
use lyon::path::path::Builder;
use lyon::path::Path as LyonPath;
use lyon::tessellation::*;
use std::cell::RefCell;

pub use lyon::tessellation::{FillOptions, FillRule, LineCap, LineJoin, StrokeOptions};

thread_local! {
    static STROKE_TESSELLATOR:RefCell<StrokeTessellator> = RefCell::new(StrokeTessellator::new());
    static FILL_TESSELLATOR:RefCell<FillTessellator> = RefCell::new(FillTessellator::new());
}

pub struct Quadratic {
    pub ctrl: (f32, f32),
    pub to: (f32, f32),
}

pub struct Cubic {
    pub ctrl1: (f32, f32),
    pub ctrl2: (f32, f32),
    pub to: (f32, f32),
}

pub trait LyonBezierCurve {
    fn bezier_to(&self, builder: &mut Builder);
}

impl LyonBezierCurve for Cubic {
    #[inline]
    fn bezier_to(&self, builder: &mut Builder) {
        builder.cubic_bezier_to(
            point(self.ctrl1.0, self.ctrl1.0),
            point(self.ctrl2.0, self.ctrl2.0),
            point(self.to.0, self.to.0),
        );
    }
}

impl LyonBezierCurve for Quadratic {
    #[inline]
    fn bezier_to(&self, builder: &mut Builder) {
        builder.quadratic_bezier_to(point(self.ctrl.0, self.ctrl.1), point(self.to.0, self.to.1));
    }
}

pub struct PathBuilder {
    lyon_builder: Builder,
    open: bool,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            open: false,
            lyon_builder: LyonPath::builder(),
        }
    }

    #[inline]
    pub fn begin(&mut self, x: f32, y: f32) {
        debug_assert!(!self.open, "path already open");
        self.open = true;
        self.lyon_builder.begin(point(x, y));
    }

    #[inline]
    pub fn end(&mut self, close: bool) {
        debug_assert!(self.open, "path already closed");
        self.open = false;
        self.lyon_builder.end(close);
    }

    #[inline]
    pub fn line_to(&mut self, x: f32, y: f32) {
        debug_assert!(self.open, "path should be open");
        self.lyon_builder.line_to(point(x, y));
    }

    #[inline]
    pub fn bezier_to(&mut self, curve: &impl LyonBezierCurve) {
        debug_assert!(self.open, "path already closed");
        curve.bezier_to(&mut self.lyon_builder);
    }

    #[inline]
    pub fn stroke(self, line_width: f32) -> Path {
        self.stroke_with_options(StrokeOptions::default().with_line_width(line_width))
    }

    pub fn stroke_with_options(self, options: StrokeOptions) -> Path {
        let lyon_path = self.lyon_builder.build();
        let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
        {
            // Compute the tessellation.
            STROKE_TESSELLATOR.with(|tessellator| {
                tessellator
                    .borrow_mut()
                    .tessellate_path(
                        &lyon_path,
                        &options,
                        &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| {
                            vertex.position().to_array()
                        }),
                    )
                    .unwrap()
            });
        }

        Path {
            vertices: geometry.vertices.concat(),
            indices: geometry.indices,
        }
    }

    #[inline]
    pub fn fill(self) -> Path {
        self.fill_with_options(FillOptions::default())
    }

    pub fn fill_with_options(self, options: FillOptions) -> Path {
        let lyon_path = self.lyon_builder.build();
        let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
        {
            // Compute the tessellation.
            FILL_TESSELLATOR.with(|tessellator| {
                tessellator
                    .borrow_mut()
                    .tessellate_path(
                        &lyon_path,
                        &options,
                        &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                            vertex.position().to_array()
                        }),
                    )
                    .unwrap()
            });
        }

        Path {
            vertices: geometry.vertices.concat(),
            indices: geometry.indices,
        }
    }
}

pub struct Path {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

impl Path {
    pub fn calculate_points(&self) -> Vec<[f32; 2]> {
        //get points of the path https://www.youtube.com/watch?v=h4CynGy_5D0
        //https://gist.github.com/JordanDelcros/cea7b8b231660ebccc6f
        (0..self.vertices.len())
            .step_by(6)
            .map(|i| {
                let (x1, y1) = (self.vertices[i + 0], self.vertices[i + 1]);
                let (x2, y2) = (self.vertices[i + 2], self.vertices[i + 3]);
                let (x3, y3) = (self.vertices[i + 4], self.vertices[i + 5]);
                let center_x = (x1 + x2 + x3) / 3.0;
                let center_y = (y1 + y2 + y3) / 3.0;
                [center_x, center_y]
            })
            .collect::<_>()
    }

    pub fn builder() -> PathBuilder {
        PathBuilder::new()
    }
}
