use super::geometry;
use super::tess::TessMode;
use super::tess::*;
use crate::builder::DrawProcess;
use crate::draw::{Draw, ShapeInfo};
use crate::transform::DrawTransform;
use glam::Mat3;
use lyon::tessellation::*;
use notan_graphics::color::Color;

pub struct Ellipse {
    color: Color,
    pos: (f32, f32),
    size: (f32, f32),
    rotation: f32,
    mode: TessMode,
    stroke_width: f32,
    alpha: f32,
    matrix: Option<Mat3>,
    tolerance: f32,
}

impl Ellipse {
    pub fn new(pos: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            color: Color::WHITE,
            pos,
            size,
            mode: TessMode::Fill,
            stroke_width: 1.0,
            alpha: 1.0,
            matrix: None,
            tolerance: StrokeOptions::DEFAULT_TOLERANCE,
            rotation: 0.0,
        }
    }

    pub fn rotate(&mut self, radians: f32) -> &mut Self {
        self.rotation = radians;
        self
    }

    pub fn rotate_degrees(&mut self, deg: f32) -> &mut Self {
        self.rotation = deg * notan_math::DEG_TO_RAD;
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
}

impl DrawTransform for Ellipse {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Ellipse {
    fn draw_process(self, draw: &mut Draw) {
        match self.mode {
            TessMode::Fill => fill(self, draw),
            TessMode::Stroke => stroke(self, draw),
        }
    }
}

fn stroke(ellipse: Ellipse, draw: &mut Draw) {
    let Ellipse {
        color,
        pos: (x, y),
        size: (width, height),
        rotation,
        stroke_width,
        alpha,
        matrix,
        tolerance,
        ..
    } = ellipse;

    let stroke_options = StrokeOptions::default()
        .with_line_width(stroke_width)
        .with_tolerance(tolerance);
    let color = color.with_alpha(color.a * alpha);

    let path = geometry::ellipse(x, y, width, height, rotation);
    let (vertices, indices) = stroke_lyon_path(&path, color, &stroke_options);

    draw.add_shape(&ShapeInfo {
        transform: matrix.as_ref(),
        vertices: &vertices,
        indices: &indices,
    });
}

fn fill(ellipse: Ellipse, draw: &mut Draw) {
    let Ellipse {
        color,
        pos: (x, y),
        size: (width, height),
        rotation,
        alpha,
        matrix,
        tolerance,
        ..
    } = ellipse;

    let fill_options = FillOptions::default().with_tolerance(tolerance);
    let color = color.with_alpha(color.a * alpha);

    let path = geometry::ellipse(x, y, width, height, rotation);
    let (vertices, indices) = fill_lyon_path(&path, color, &fill_options);

    draw.add_shape(&ShapeInfo {
        transform: matrix.as_ref(),
        vertices: &vertices,
        indices: &indices,
    });
}
