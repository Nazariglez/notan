use super::path::Path;
use super::tess::TessMode;
use crate::builder::{DrawBuilder, DrawProcess};
use crate::draw2::{Draw2, ShapeInfo};
use notan_graphics::color::Color;

pub struct Triangle {
    colors: [Color; 3],
    points: [(f32, f32); 3],
    mode: TessMode,
    stroke_width: f32,
}

impl Triangle {
    pub fn new(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> Self {
        Self {
            colors: [Color::WHITE; 3],
            points: [a, b, c],
            mode: TessMode::Fill,
            stroke_width: 1.0,
        }
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.colors.fill(color);
        self
    }

    pub fn color_vertex(&mut self, a: Color, b: Color, c: Color) -> &mut Self {
        self.colors[0] = a;
        self.colors[1] = b;
        self.colors[2] = c;
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

impl DrawProcess for Triangle {
    fn draw_process(self, draw: &mut Draw2) {
        match self.mode {
            TessMode::Fill => fill(self, draw),
            TessMode::Stroke => stroke(self, draw),
        }
    }
}

fn stroke(triangle: Triangle, draw: &mut Draw2) {
    let Triangle {
        colors: [ca, ..],
        points: [a, b, c],
        stroke_width,
        ..
    } = triangle;

    let mut path = Path::new();
    path.move_to(a.0, a.1)
        .line_to(b.0, b.1)
        .line_to(c.0, c.1)
        .stroke(stroke_width)
        .color(ca)
        .close();

    path.draw_process(draw);
}

fn fill(triangle: Triangle, draw: &mut Draw2) {
    let Triangle {
        colors: [ca, cb, cc],
        points: [a, b, c],
        mode,
        ..
    } = triangle;

    let indices = [0, 1, 2];
    #[rustfmt::skip]
        let vertices = [
        a.0, a.1, ca.r, ca.g, ca.b, ca.a,
        b.0, b.1, cb.r, cb.g, cb.b, cb.a,
        c.0, c.1, cc.r, cc.g, cc.b, cc.a,
    ];

    draw.shape(&ShapeInfo {
        vertices: &vertices,
        indices: &indices,
    });
}
