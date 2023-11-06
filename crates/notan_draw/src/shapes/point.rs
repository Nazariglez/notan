use super::path::Path;
use crate::builder::DrawProcess;
use crate::draw::Draw;
use crate::transform::DrawTransform;
use notan_graphics::color::Color;
use notan_math::Mat3;

/// Point alignment in the X axis.
///
/// This is only meaningful for width > 1.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XAlignment {
    Left,
    #[default]
    Center,
    Right,
}
/// Point alignment in the Y axis.
///
/// This is only meaningful for width > 1.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YAlignment {
    Top,
    #[default]
    Center,
    Bottom,
}

/// A single point.
pub struct Point {
    x: f32,
    y: f32,
    color: Color,
    stroke_width: f32,
    x_align: XAlignment,
    y_align: YAlignment,
    alpha: f32,
    matrix: Option<Mat3>,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            color: Color::WHITE,
            stroke_width: 1.0,
            x_align: XAlignment::Center,
            y_align: YAlignment::Center,
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

    pub fn align(&mut self, x_align: XAlignment, y_align: YAlignment) -> &mut Self {
        self.x_align = x_align;
        self.y_align = y_align;
        self
    }
    pub fn x_align(&mut self, x_align: XAlignment) -> &mut Self {
        self.x_align = x_align;
        self
    }
    pub fn y_align(&mut self, y_align: YAlignment) -> &mut Self {
        self.y_align = y_align;
        self
    }
    pub fn x_align_left(&mut self) -> &mut Self {
        self.x_align = XAlignment::Left;
        self
    }
    pub fn x_align_center(&mut self) -> &mut Self {
        self.x_align = XAlignment::Center;
        self
    }
    pub fn x_align_right(&mut self) -> &mut Self {
        self.x_align = XAlignment::Right;
        self
    }
    pub fn y_align_top(&mut self) -> &mut Self {
        self.y_align = YAlignment::Top;
        self
    }
    pub fn y_align_middle(&mut self) -> &mut Self {
        self.y_align = YAlignment::Center;
        self
    }
    pub fn y_align_bottom(&mut self) -> &mut Self {
        self.y_align = YAlignment::Bottom;
        self
    }
}

impl DrawTransform for Point {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Point {
    fn draw_process(self, draw: &mut Draw) {
        let Self {
            x,
            y,
            color,
            stroke_width,
            x_align,
            y_align,
            alpha,
            matrix,
        } = self;

        let mut path = Path::new();

        let x_from = match x_align {
            XAlignment::Left => x + stroke_width / 2.,
            XAlignment::Center => x + 1.0,
            XAlignment::Right => x + 1.0 - stroke_width / 2.,
        };
        let x_to = match x_align {
            XAlignment::Left => x + stroke_width / 2.,
            XAlignment::Center => x + 1.0,
            XAlignment::Right => x + 1.0 - stroke_width / 2.,
        };
        let y_from = match y_align {
            YAlignment::Top => y,
            YAlignment::Center => y + 1.0 - stroke_width / 2.,
            YAlignment::Bottom => y + 1.0 - stroke_width,
        };
        let y_to = match y_align {
            YAlignment::Top => y + stroke_width,
            YAlignment::Center => y + 1.0 + stroke_width / 2.,
            YAlignment::Bottom => y + 1.0,
        };

        path.move_to(x_from, y_from)
            .line_to(x_to, y_to)
            .stroke(stroke_width)
            .color(color.with_alpha(color.a * alpha))
            .close();

        if let Some(m) = matrix {
            path.transform(m);
        }

        path.draw_process(draw);
    }
}
