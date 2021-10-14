use super::path::Path;
use crate::builder::DrawProcess;
use crate::draw::Draw;
use crate::transform::DrawTransform;
use notan_graphics::color::Color;
use notan_math::glam::Mat3;

pub struct Line {
    p1: (f32, f32),
    p2: (f32, f32),
    color: Color,
    stroke_width: f32,
    alpha: f32,
    matrix: Option<Mat3>,
}

impl Line {
    pub fn new(p1: (f32, f32), p2: (f32, f32)) -> Self {
        Self {
            p1,
            p2,
            color: Color::WHITE,
            stroke_width: 1.0,
            alpha: 1.0,
            matrix: None,
        }
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn width(&mut self, width: f32) -> &mut Self {
        self.stroke_width = width;
        self
    }

    pub fn alpha(&mut self, alpha: f32) -> &mut Self {
        self.alpha = alpha;
        self
    }
}

impl DrawTransform for Line {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Line {
    fn draw_process(self, draw: &mut Draw) {
        let Self {
            p1: (x1, y1),
            p2: (x2, y2),
            color,
            stroke_width,
            alpha,
            matrix,
        } = self;

        let mut path = Path::new();
        path.move_to(x1, y1)
            .line_to(x2, y2)
            .stroke(stroke_width)
            .color(color.with_alpha(color.a * alpha))
            .close();

        if let Some(m) = matrix {
            path.transform(m);
        }

        path.draw_process(draw);
    }
}
