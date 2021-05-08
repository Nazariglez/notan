use super::path::Path;
use super::tess::TessMode;
use crate::builder::{DrawBuilder, DrawProcess};
use crate::draw::{Draw, ShapeInfo};
use crate::transform::DrawTransform;
use glam::Mat3;
use notan_graphics::color::Color;

pub struct Rectangle {
    colors: [Color; 4],
    pos: (f32, f32),
    size: (f32, f32),
    mode: TessMode,
    stroke_width: f32,
    alpha: f32,
    matrix: Option<Mat3>,
}

impl Rectangle {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            colors: [Color::WHITE; 4],
            pos: position,
            size: size,
            mode: TessMode::Fill,
            stroke_width: 1.0,
            alpha: 1.0,
            matrix: None,
        }
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.colors.fill(color);
        self
    }

    pub fn color_vertex(&mut self, a: Color, b: Color, c: Color, d: Color) -> &mut Self {
        self.colors[0] = a;
        self.colors[1] = b;
        self.colors[2] = c;
        self.colors[3] = d;
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

impl DrawTransform for Rectangle {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Rectangle {
    fn draw_process(self, draw: &mut Draw) {
        match self.mode {
            TessMode::Fill => fill(self, draw),
            TessMode::Stroke => stroke(self, draw),
        }
    }
}

fn stroke(quad: Rectangle, draw: &mut Draw) {
    let Rectangle {
        colors: [ca, ..],
        pos: (x, y),
        size: (width, height),
        stroke_width,
        alpha,
        matrix,
        ..
    } = quad;

    let mut path = Path::new();
    path.move_to(x, y)
        .line_to(x, y + height)
        .line_to(x + width, y + height)
        .line_to(x + width, y)
        .stroke(stroke_width)
        .color(ca.with_alpha(ca.a * alpha))
        .close();

    if let Some(m) = matrix {
        path.transform(m);
    }

    path.draw_process(draw);
}

fn fill(quad: Rectangle, draw: &mut Draw) {
    let Rectangle {
        colors: [ca, cb, cc, cd],
        pos: (x1, y1),
        size: (width, height),
        mode,
        alpha,
        matrix,
        ..
    } = quad;

    let x2 = x1 + width;
    let y2 = y1 + height;

    let indices = [0, 1, 2, 0, 2, 3];
    #[rustfmt::skip]
    let vertices = [
        x1, y1, ca.r, ca.g, ca.b, ca.a * alpha,
        x1, y2, cb.r, cb.g, cb.b, cb.a * alpha,
        x2, y2, cc.r, cc.g, cc.b, cc.a * alpha,
        x2, y1, cd.r, cd.g, cd.b, cd.a * alpha,
    ];

    draw.add_shape(&ShapeInfo {
        transform: matrix.as_ref(),
        vertices: &vertices,
        indices: &indices,
    });
}
