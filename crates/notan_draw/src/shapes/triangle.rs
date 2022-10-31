use super::path::Path;
use super::tess::TessMode;
use crate::builder::DrawProcess;
use crate::draw::{Draw, ShapeInfo};
use crate::transform::DrawTransform;
use notan_graphics::color::Color;
use notan_graphics::pipeline::BlendMode;
use notan_math::Mat3;

pub struct Triangle {
    colors: [Color; 3],
    points: [(f32, f32); 3],
    stroke_width: f32,
    alpha: f32,
    matrix: Option<Mat3>,
    blend_mode: Option<BlendMode>,
    modes: [Option<TessMode>; 2],
    mode_index: usize,
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
}

impl Triangle {
    pub fn new(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> Self {
        Self {
            colors: [Color::WHITE; 3],
            points: [a, b, c],
            stroke_width: 1.0,
            alpha: 1.0,
            matrix: None,
            blend_mode: None,
            modes: [None; 2],
            mode_index: 0,
            fill_color: None,
            stroke_color: None,
        }
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
        self.colors.fill(color);
        self
    }

    pub fn color_vertex(&mut self, a: Color, b: Color, c: Color) -> &mut Self {
        self.colors[0] = a;
        self.colors[1] = b;
        self.colors[2] = c;
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

impl DrawTransform for Triangle {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Triangle {
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

fn stroke(triangle: &Triangle, draw: &mut Draw) {
    let Triangle {
        colors: [ca, ..],
        points: [a, b, c],
        stroke_width,
        alpha,
        matrix,
        blend_mode,
        stroke_color,
        ..
    } = *triangle;

    let mut path = Path::new();

    if let Some(bm) = blend_mode {
        path.blend_mode(bm);
    }

    let color = stroke_color.unwrap_or(ca);
    path.move_to(a.0, a.1)
        .line_to(b.0, b.1)
        .line_to(c.0, c.1)
        .stroke(stroke_width)
        .stroke_color(color)
        .alpha(alpha)
        .close();

    if let Some(m) = matrix {
        path.transform(m);
    }

    path.draw_process(draw);
}

fn fill(triangle: &Triangle, draw: &mut Draw) {
    let Triangle {
        colors: [ca, cb, cc],
        points: [a, b, c],
        alpha,
        matrix,
        blend_mode,
        fill_color,
        ..
    } = *triangle;

    let ca = fill_color.unwrap_or(ca);
    let cb = fill_color.unwrap_or(cb);
    let cc = fill_color.unwrap_or(cc);

    let indices = [0, 1, 2];
    #[rustfmt::skip]
        let vertices = [
        a.0, a.1, ca.r, ca.g, ca.b, ca.a * alpha,
        b.0, b.1, cb.r, cb.g, cb.b, cb.a * alpha,
        c.0, c.1, cc.r, cc.g, cc.b, cc.a * alpha,
    ];

    draw.add_shape(&ShapeInfo {
        transform: matrix.as_ref(),
        vertices: &vertices,
        indices: &indices,
        blend_mode,
    });
}
