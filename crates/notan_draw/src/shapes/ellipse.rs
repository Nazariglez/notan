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

pub struct Ellipse {
    color: Color,
    pos: (f32, f32),
    size: (f32, f32),
    rotation: f32,
    stroke_width: f32,
    alpha: f32,
    matrix: Option<Mat3>,
    tolerance: f32,
    blend_mode: Option<BlendMode>,
    modes: [Option<TessMode>; 2],
    mode_index: usize,
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
}

impl Ellipse {
    pub fn new(pos: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            color: Color::WHITE,
            pos,
            size,
            stroke_width: 1.0,
            alpha: 1.0,
            matrix: None,
            tolerance: StrokeOptions::DEFAULT_TOLERANCE,
            rotation: 0.0,
            blend_mode: None,
            modes: [None; 2],
            mode_index: 0,
            fill_color: None,
            stroke_color: None,
        }
    }

    pub fn rotate(&mut self, radians: f32) -> &mut Self {
        self.rotation = radians;
        self
    }

    pub fn rotate_degrees(&mut self, deg: f32) -> &mut Self {
        self.rotation = deg.to_radians();
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
}

impl DrawTransform for Ellipse {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Ellipse {
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
        })
    }
}

fn stroke(ellipse: &Ellipse, draw: &mut Draw) {
    let Ellipse {
        color,
        pos: (x, y),
        size: (width, height),
        rotation,
        stroke_width,
        alpha,
        matrix,
        tolerance,
        blend_mode,
        stroke_color,
        ..
    } = *ellipse;

    let stroke_options = StrokeOptions::default()
        .with_line_width(stroke_width)
        .with_tolerance(tolerance);

    let color = stroke_color.unwrap_or(color);
    let color = color.with_alpha(color.a * alpha);

    let path = geometry::ellipse(x, y, width, height, rotation);
    let (vertices, indices) = stroke_lyon_path(&path, color, &stroke_options);

    draw.add_shape(&ShapeInfo {
        transform: matrix.as_ref(),
        vertices: &vertices,
        indices: &indices,
        blend_mode,
    });
}

fn fill(ellipse: &Ellipse, draw: &mut Draw) {
    let Ellipse {
        color,
        pos: (x, y),
        size: (width, height),
        rotation,
        alpha,
        matrix,
        tolerance,
        blend_mode,
        fill_color,
        ..
    } = *ellipse;

    let fill_options = FillOptions::default().with_tolerance(tolerance);

    let color = fill_color.unwrap_or(color);
    let color = color.with_alpha(color.a * alpha);

    let path = geometry::ellipse(x, y, width, height, rotation);
    let (vertices, indices) = fill_lyon_path(&path, color, &fill_options);

    draw.add_shape(&ShapeInfo {
        transform: matrix.as_ref(),
        vertices: &vertices,
        indices: &indices,
        blend_mode,
    });
}
