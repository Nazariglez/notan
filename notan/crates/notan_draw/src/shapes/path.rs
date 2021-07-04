use super::tess::{fill_lyon_path, stroke_lyon_path, TessMode};
use crate::builder::DrawProcess;
use crate::draw::{Draw, ShapeInfo};
use crate::transform::DrawTransform;
use glam::Mat3;
use lyon::math::point;
use lyon::path::path::Builder;
use lyon::tessellation::*;
use notan_graphics::color::Color;

pub struct Path {
    stroke_options: StrokeOptions,
    fill_options: FillOptions,
    builder: Builder,
    initialized: bool,
    mode: TessMode,
    color: Color,
    alpha: f32,
    matrix: Option<Mat3>,
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl Path {
    pub fn new() -> Self {
        Self {
            stroke_options: StrokeOptions::default(),
            fill_options: FillOptions::default(),
            builder: lyon::path::Path::builder(),
            initialized: false,
            mode: TessMode::Stroke,
            color: Color::WHITE,
            alpha: 1.0,
            matrix: None,
        }
    }

    pub fn alpha(&mut self, alpha: f32) -> &mut Self {
        self.alpha = alpha;
        self
    }

    // Start the path on the point given
    pub fn move_to(&mut self, x: f32, y: f32) -> &mut Self {
        if self.initialized {
            self.builder.end(false);
        }
        self.builder.begin(point(x, y));
        self.initialized = true;
        self
    }

    // Draw a line from the previous point to the new point
    pub fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        debug_assert!(self.initialized, "You should use move_to first");
        self.builder.line_to(point(x, y));
        self
    }

    pub fn quadratic_bezier_to(&mut self, ctrl: (f32, f32), to: (f32, f32)) -> &mut Self {
        debug_assert!(self.initialized, "You should use move_to first");
        self.builder
            .quadratic_bezier_to(point(ctrl.0, ctrl.1), point(to.0, to.1));
        self
    }

    pub fn cubic_bezier_to(
        &mut self,
        ctrl1: (f32, f32),
        ctrl2: (f32, f32),
        to: (f32, f32),
    ) -> &mut Self {
        debug_assert!(self.initialized, "You should use move_to first");
        self.builder.cubic_bezier_to(
            point(ctrl1.0, ctrl1.1),
            point(ctrl2.0, ctrl2.1),
            point(to.0, to.1),
        );
        self
    }

    // Closes the line drawing a line to the last move_to point
    pub fn close(&mut self) -> &mut Self {
        debug_assert!(self.initialized, "You should use move_to first");
        self.initialized = false;
        self.builder.end(true);
        self
    }

    pub fn tolerance(&mut self, tolerance: f32) -> &mut Self {
        self.stroke_options = self.stroke_options.with_tolerance(tolerance);
        self.fill_options = self.fill_options.with_tolerance(tolerance);
        self
    }

    pub fn round_cap(&mut self) -> &mut Self {
        self.stroke_options = self
            .stroke_options
            .with_start_cap(LineCap::Round)
            .with_end_cap(LineCap::Round);
        self
    }

    pub fn butt_cap(&mut self) -> &mut Self {
        self.stroke_options = self
            .stroke_options
            .with_start_cap(LineCap::Butt)
            .with_end_cap(LineCap::Butt);
        self
    }

    pub fn square_cap(&mut self) -> &mut Self {
        self.stroke_options = self
            .stroke_options
            .with_start_cap(LineCap::Square)
            .with_end_cap(LineCap::Square);
        self
    }

    pub fn miter_join(&mut self) -> &mut Self {
        self.stroke_options = self.stroke_options.with_line_join(LineJoin::Miter);
        self
    }

    pub fn round_join(&mut self) -> &mut Self {
        self.stroke_options = self.stroke_options.with_line_join(LineJoin::Round);
        self
    }

    pub fn bevel_join(&mut self) -> &mut Self {
        self.stroke_options = self.stroke_options.with_line_join(LineJoin::Bevel);
        self
    }

    pub fn fill(&mut self) -> &mut Self {
        self.mode = TessMode::Fill;
        self
    }

    pub fn stroke(&mut self, width: f32) -> &mut Self {
        self.stroke_options = self.stroke_options.with_line_width(width);
        self.mode = TessMode::Stroke;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }
}

impl DrawProcess for Path {
    fn draw_process(mut self, draw: &mut Draw) {
        if self.initialized {
            self.builder.end(false);
        }

        let Self {
            builder,
            stroke_options,
            fill_options,
            color,
            alpha,
            matrix,
            ..
        } = self;

        let color = color.with_alpha(color.a * alpha);

        let path = builder.build();
        let (vertices, indices) = match self.mode {
            TessMode::Fill => fill_lyon_path(&path, color, &fill_options),
            TessMode::Stroke => stroke_lyon_path(&path, color, &stroke_options),
        };

        draw.add_shape(&ShapeInfo {
            transform: matrix.as_ref(),
            vertices: &vertices,
            indices: &indices,
        });
    }
}

impl DrawTransform for Path {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}
