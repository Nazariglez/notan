use crate::draw::{Draw, VertexColor};
// use crate::draw::DrawInfo;
use lyon::math::{point, Point};
use lyon::path::builder::*;
use lyon::path::path::Builder;
use lyon::path::Path as LyonPath;
use lyon::tessellation::*;
use notan_graphics::color::Color;
use std::cell::RefCell;

pub use lyon::tessellation::{FillOptions, FillRule, LineCap, LineJoin, StrokeOptions};

thread_local! {
    static STROKE_TESSELLATOR:RefCell<StrokeTessellator> = RefCell::new(StrokeTessellator::new());
    static FILL_TESSELLATOR:RefCell<FillTessellator> = RefCell::new(FillTessellator::new());
}

#[derive(Debug, Clone)]
pub enum PathLine {
    Begin {
        x: f32,
        y: f32,
    },
    LineTo {
        to: (f32, f32),
    },
    Quadratic {
        to: (f32, f32),
        ctrl: (f32, f32),
    },
    Cubic {
        to: (f32, f32),
        ctrl1: (f32, f32),
        ctrl2: (f32, f32),
    },
    End {
        closed: bool,
    },
}

#[derive(Clone)]
pub struct PathBuilder {
    lines: Vec<PathLine>,
    initialized: bool,
    finished: bool,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            initialized: false,
            finished: false,
            lines: vec![],
        }
    }

    pub fn begin(&mut self, x: f32, y: f32) -> &mut Self {
        debug_assert!(!self.initialized, "path already initialed");
        self.initialized = true;
        self.lines.push(PathLine::Begin { x, y });
        self
    }

    pub fn end(&mut self, close: bool) -> &mut Self {
        debug_assert!(self.initialized, "path already closed");
        self.initialized = false;
        self.finished = true;
        self.lines.push(PathLine::End { closed: close });
        self
    }

    pub fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        debug_assert!(self.initialized, "path should be initialed");
        self.lines.push(PathLine::LineTo { to: (x, y) });
        self
    }

    pub fn quadratic_bezier_to(&mut self, ctrl: (f32, f32), to: (f32, f32)) -> &mut Self {
        debug_assert!(self.initialized, "path should be initialed");
        self.lines.push(PathLine::Quadratic { ctrl, to });
        self
    }

    pub fn cubic_bezier_to(
        &mut self,
        ctrl1: (f32, f32),
        ctrl2: (f32, f32),
        to: (f32, f32),
    ) -> &mut Self {
        debug_assert!(self.initialized, "path should be initialed");
        self.lines.push(PathLine::Cubic { ctrl1, ctrl2, to });
        self
    }

    pub fn stroke(self, line_width: f32) -> Path {
        self.stroke_with_options(StrokeOptions::default().with_line_width(line_width))
    }

    pub fn stroke_with_options(self, options: StrokeOptions) -> Path {
        debug_assert!(!self.lines.is_empty(), "path without lines");
        debug_assert!(self.finished, "end the path first");
        let (lyon_path, lines) = path_from_lines(self);
        let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
        {
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
            lines,
        }
    }

    #[inline]
    pub fn fill(self) -> Path {
        self.fill_with_options(FillOptions::default())
    }

    pub fn fill_with_options(self, options: FillOptions) -> Path {
        debug_assert!(!self.lines.is_empty(), "path without lines");
        debug_assert!(self.finished, "end the path first");
        let (lyon_path, lines) = path_from_lines(self);
        let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
        {
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
            lines,
        }
    }
}

fn path_from_lines(builder: PathBuilder) -> (LyonPath, Vec<PathLine>) {
    let PathBuilder { lines, .. } = builder;

    let mut path = LyonPath::builder();
    lines.iter().for_each(|line| match line {
        PathLine::Begin { x, y } => {
            path.begin(point(*x, *y));
        }
        PathLine::LineTo { to, .. } => {
            path.line_to(point(to.0, to.1));
        }
        PathLine::Quadratic { ctrl, to, .. } => {
            path.quadratic_bezier_to(point(ctrl.0, ctrl.1), point(to.0, to.1));
        }
        PathLine::Cubic {
            ctrl1, ctrl2, to, ..
        } => {
            path.cubic_bezier_to(
                point(ctrl1.0, ctrl1.1),
                point(ctrl2.0, ctrl2.1),
                point(to.0, to.1),
            );
        }
        PathLine::End { closed } => {
            path.end(*closed);
        }
    });
    (path.build(), lines)
}

#[derive(Clone)]
pub struct Path {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    lines: Vec<PathLine>,
}

impl Path {
    pub fn builder() -> PathBuilder {
        PathBuilder::new()
    }

    pub fn lines(&self) -> &[PathLine] {
        &self.lines
    }
}

/// Wrapper to draw paths directly from the Draw object
pub struct DrawPath<'a> {
    pub(crate) builder: PathBuilder,
    pub(crate) draw: &'a mut Draw,
}

impl<'a> DrawPath<'a> {
    pub fn begin(mut self, x: f32, y: f32) -> Self {
        self.builder.begin(x, y);
        self
    }

    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        self.builder.line_to(x, y);
        self
    }

    pub fn quadratic_bezier_to(mut self, ctrl: (f32, f32), to: (f32, f32)) -> Self {
        self.builder.quadratic_bezier_to(ctrl, to);
        self
    }

    pub fn cubic_bezier_to(mut self, ctrl1: (f32, f32), ctrl2: (f32, f32), to: (f32, f32)) -> Self {
        self.builder.cubic_bezier_to(ctrl1, ctrl2, to);
        self
    }

    pub fn end(mut self, close: bool) -> Self {
        self.builder.end(close);
        self
    }

    pub fn stroke(self, width: f32) {
        let DrawPath { mut builder, draw } = self;
        draw.path(&builder.stroke(width));
    }

    pub fn stroke_with_options(mut self, options: StrokeOptions) {
        let DrawPath { mut builder, draw } = self;
        draw.path(&builder.stroke_with_options(options));
    }

    pub fn fill(mut self) {
        let DrawPath { mut builder, draw } = self;
        draw.path(&builder.fill());
    }

    pub fn fill_with_options(mut self, options: FillOptions) {
        let DrawPath { mut builder, draw } = self;
        draw.path(&builder.fill_with_options(options));
    }
    //
    // pub fn draw(self) {
    //     let DrawPath { mut builder, draw } = self;
    //     let Path {
    //         vertices, indices, ..
    //     } = builder.stroke(10.0);
    //
    //     let info = DrawInfo {
    //         vertices: &(0..vertices.len())
    //             .step_by(2)
    //             .map(|i| VertexColor {
    //                 x: vertices[i],
    //                 y: vertices[i + 1],
    //                 color: Color::WHITE,
    //             })
    //             .collect::<Vec<_>>(),
    //         indices: &indices,
    //     };
    //     draw.test_color(&info);
    // }
}
