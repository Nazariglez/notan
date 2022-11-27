use super::geometry;
use super::tess::TessMode;
use super::tess::*;
use crate::builder::DrawProcess;
use crate::draw::{Draw, ShapeInfo};
use crate::transform::DrawTransform;
use lyon::tessellation::*;
use notan_graphics::color::Color;
use notan_graphics::pipeline::BlendMode;
use notan_math::Mat3;

pub struct Circle {
    color: Color,
    pos: (f32, f32),
    radius: f32,
    stroke_width: f32,
    alpha: f32,
    matrix: Option<Mat3>,
    tolerance: f32,
    blend_mode: Option<BlendMode>,
    alpha_mode: Option<BlendMode>,
    modes: [Option<TessMode>; 2],
    mode_index: usize,
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self {
            color: Color::WHITE,
            pos: (0.0, 0.0),
            radius,
            stroke_width: 1.0,
            alpha: 1.0,
            matrix: None,
            tolerance: StrokeOptions::DEFAULT_TOLERANCE,
            blend_mode: None,
            alpha_mode: None,
            modes: [None; 2],
            mode_index: 0,
            fill_color: None,
            stroke_color: None,
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

    pub fn fill_color(&mut self, color: Color) -> &mut Self {
        self.fill_color = Some(color);
        self
    }

    pub fn stroke_color(&mut self, color: Color) -> &mut Self {
        self.stroke_color = Some(color);
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
        self.modes[self.mode_index] = Some(TessMode::Fill);
        self.mode_index = (self.mode_index + 1) % 2;
        self
    }

    pub fn stroke(&mut self, width: f32) -> &mut Self {
        self.modes[self.mode_index] = Some(TessMode::Stroke);
        self.stroke_width = width;
        self.mode_index = (self.mode_index + 1) % 2;
        self
    }

    pub fn blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        self.blend_mode = Some(mode);
        self
    }

    pub fn alpha_mode(&mut self, mode: BlendMode) -> &mut Self {
        self.alpha_mode = Some(mode);
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
        let modes = self.modes;
        modes.iter().enumerate().for_each(|(i, mode)| match mode {
            None => {
                if i == 0 {
                    // fill by default
                    fill(&self, draw);
                }
            }
            Some(mode) => match mode {
                TessMode::Fill => fill(&self, draw),
                TessMode::Stroke => stroke(&self, draw),
            },
        });
    }
}

fn stroke(circle: &Circle, draw: &mut Draw) {
    let Circle {
        color,
        pos: (x, y),
        radius,
        stroke_width,
        alpha,
        matrix,
        tolerance,
        blend_mode,
        alpha_mode,
        stroke_color,
        ..
    } = *circle;

    let stroke_options = StrokeOptions::default()
        .with_line_width(stroke_width)
        .with_tolerance(tolerance);

    let color = stroke_color.unwrap_or(color);
    let color = color.with_alpha(color.a * alpha);

    let path = geometry::circle(x, y, radius);
    let (vertices, indices) = stroke_lyon_path(&path, color, &stroke_options);

    draw.add_shape(&ShapeInfo {
        transform: matrix.as_ref(),
        vertices: &vertices,
        indices: &indices,
        blend_mode,
        alpha_mode,
    });
}

fn fill(circle: &Circle, draw: &mut Draw) {
    let Circle {
        color,
        pos: (x, y),
        radius,
        alpha,
        matrix,
        tolerance,
        blend_mode,
        alpha_mode,
        fill_color,
        ..
    } = *circle;

    let fill_options = FillOptions::default().with_tolerance(tolerance);

    let color = fill_color.unwrap_or(color);
    let color = color.with_alpha(color.a * alpha);

    let path = geometry::circle(x, y, radius);
    let (vertices, indices) = fill_lyon_path(&path, color, &fill_options);

    draw.add_shape(&ShapeInfo {
        transform: matrix.as_ref(),
        vertices: &vertices,
        indices: &indices,
        blend_mode,
        alpha_mode,
    });
}
