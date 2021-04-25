use crate::builder::{DrawBuilder, DrawProcess};
use crate::draw2::{Draw2, ShapeInfo};
use crate::geometry::Path;
use notan_graphics::color::Color;

#[derive(Debug, Clone, Copy)]
enum TriangleMode {
    Fill,
    Stroke(f32),
}

pub struct Triangle {
    colors: [Color; 3],
    points: [(f32, f32); 3],
    mode: TriangleMode,
}

impl Triangle {
    pub fn new(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> Self {
        Self {
            colors: [Color::WHITE; 3],
            points: [a, b, c],
            mode: TriangleMode::Fill,
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
        self.mode = TriangleMode::Fill;
        self
    }

    pub fn stroke(&mut self, width: f32) -> &mut Self {
        self.mode = TriangleMode::Stroke(width);
        self
    }
}

impl DrawProcess for Triangle {
    fn draw_process(self, draw: &mut Draw2) {
        match self.mode {
            TriangleMode::Fill => fill(self, draw),
            TriangleMode::Stroke(width) => stroke(self, width, draw),
        }
    }
}

fn stroke(triangle: Triangle, width: f32, draw: &mut Draw2) {
    let Triangle {
        colors: [ca, cb, cc],
        points: [a, b, c],
        mode,
    } = triangle;

    let mut builder = Path::builder();
    builder
        .begin(a.0, a.1)
        .line_to(b.0, b.1)
        .line_to(c.0, c.1)
        .end(true);
    let path = builder.stroke(width);

    let vertices = {
        let mut vertices = vec![];
        for i in (0..path.vertices.len()).step_by(2) {
            vertices.extend(&[
                path.vertices[i],
                path.vertices[i + 1],
                ca.r,
                ca.g,
                ca.b,
                ca.a,
            ]);
        }
        vertices
    };

    draw.shape(&ShapeInfo {
        vertices: &vertices,
        indices: &path.indices,
    });
}

fn fill(triangle: Triangle, draw: &mut Draw2) {
    let Triangle {
        colors: [ca, cb, cc],
        points: [a, b, c],
        mode,
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
