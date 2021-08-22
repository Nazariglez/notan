use super::geometry;
use super::tess::TessMode;
use super::tess::*;
use crate::builder::DrawProcess;
use crate::draw::{Draw, ShapeInfo};
use crate::transform::DrawTransform;
use glam::Mat3;
use lyon::tessellation::*;
use notan_graphics::color::Color;
use notan_graphics::pipeline::BlendMode;

pub struct Circle {
    color: Color,
    pos: (f32, f32),
    radius: f32,
    mode: TessMode,
    stroke_width: f32,
    alpha: f32,
    matrix: Option<Mat3>,
    tolerance: f32,
    blend_mode: Option<BlendMode>,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self {
            color: Color::WHITE,
            pos: (0.0, 0.0),
            radius,
            mode: TessMode::Fill,
            stroke_width: 1.0,
            alpha: 1.0,
            matrix: None,
            tolerance: StrokeOptions::DEFAULT_TOLERANCE,
            blend_mode: None,
        }
    }

    pub fn position(&mut self, x: f32, y: f32) -> &mut Self {
        self.pos = (x, y);
        self
    }

    pub fn tolerance(&mut self, value: f32) -> &mut Self {
        self.tolerance = value;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn alpha(&mut self, alpha: f32) -> &mut Self {
        self.alpha = alpha;
        self
    }

    pub fn fill(&mut self) -> &mut Self {
        self.mode = TessMode::Fill;
        self
    }

    pub fn stroke(&mut self, width: f32) -> &mut Self {
        self.mode = TessMode::Stroke;
        self.stroke_width = width;
        self
    }

    pub fn blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        self.blend_mode = Some(mode);
        self
    }
}

impl DrawTransform for Circle {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Circle {
    fn draw_process(self, draw: &mut Draw) {
        match self.mode {
            TessMode::Fill => fill(self, draw),
            TessMode::Stroke => stroke(self, draw),
        }
    }
}

fn stroke(circle: Circle, draw: &mut Draw) {
    let Circle {
        color,
        pos: (x, y),
        radius,
        stroke_width,
        alpha,
        matrix,
        tolerance,
        blend_mode,
        ..
    } = circle;

    let stroke_options = StrokeOptions::default()
        .with_line_width(stroke_width)
        .with_tolerance(tolerance);
    let color = color.with_alpha(color.a * alpha);

    let path = geometry::circle(x, y, radius);
    let (vertices, indices) = stroke_lyon_path(&path, color, &stroke_options);

    draw.add_shape(&ShapeInfo {
        transform: matrix.as_ref(),
        vertices: &vertices,
        indices: &indices,
        blend_mode,
    });
}

fn fill(circle: Circle, draw: &mut Draw) {
    let Circle {
        color,
        pos: (x, y),
        radius,
        alpha,
        matrix,
        tolerance,
        blend_mode,
        ..
    } = circle;

    let fill_options = FillOptions::default().with_tolerance(tolerance);
    let color = color.with_alpha(color.a * alpha);

    let path = geometry::circle(x, y, radius);
    let (vertices, indices) = fill_lyon_path(&path, color, &fill_options);

    draw.add_shape(&ShapeInfo {
        transform: matrix.as_ref(),
        vertices: &vertices,
        indices: &indices,
        blend_mode,
    });
}
