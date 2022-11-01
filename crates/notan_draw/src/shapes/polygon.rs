use super::path::Path;
use super::tess::TessMode;
use crate::builder::DrawProcess;
use crate::draw::Draw;
use crate::transform::DrawTransform;
use crate::{DrawBuilder, DrawShapes};
use notan_graphics::color::Color;
use notan_graphics::pipeline::BlendMode;
use notan_math::Mat3;
use std::f32::consts::PI;

pub struct Polygon {
    color: Color,
    pos: (f32, f32),
    stroke_width: f32,
    alpha: f32,
    matrix: Option<Mat3>,
    blend_mode: Option<BlendMode>,
    modes: [Option<TessMode>; 2],
    mode_index: usize,
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
    sides: u8,
    radius: f32,
}

impl Polygon {
    pub fn new(sides: u8, radius: f32) -> Self {
        Self {
            color: Color::WHITE,
            stroke_width: 1.0,
            pos: (0.0, 0.0),
            alpha: 1.0,
            matrix: None,
            blend_mode: None,
            modes: [None; 2],
            mode_index: 0,
            fill_color: None,
            stroke_color: None,
            sides,
            radius,
        }
    }

    pub fn position(&mut self, x: f32, y: f32) -> &mut Self {
        self.pos = (x, y);
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

impl DrawTransform for Polygon {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Polygon {
    fn draw_process(self, draw: &mut Draw) {
        let mut path_builder = draw.path();
        draw_polygon(
            &mut path_builder,
            self.pos.0,
            self.pos.1,
            self.sides as _,
            self.radius,
        );
        path_builder.color(self.color).alpha(self.alpha);

        if let Some(bm) = self.blend_mode {
            path_builder.blend_mode(bm);
        }

        if let Some(m) = self.matrix {
            path_builder.transform(m);
        }

        let modes = self.modes;
        modes.iter().enumerate().for_each(|(i, mode)| match mode {
            None => {
                if i == 0 {
                    if let Some(c) = self.fill_color {
                        path_builder.fill_color(c);
                    }

                    path_builder.fill();
                }
            }
            Some(mode) => match mode {
                TessMode::Fill => {
                    if let Some(c) = self.fill_color {
                        path_builder.fill_color(c);
                    }
                    path_builder.fill();
                }
                TessMode::Stroke => {
                    if let Some(c) = self.stroke_color {
                        path_builder.stroke_color(c);
                    }

                    path_builder.stroke(self.stroke_width);
                }
            },
        });
    }
}

fn draw_polygon(
    path_builder: &mut DrawBuilder<Path>,
    center_x: f32,
    center_y: f32,
    sides: usize,
    radius: f32,
) {
    for n in 0..sides {
        let i = n as f32;

        let pi_sides = PI / sides as f32;
        let is_even = sides % 2 == 0;
        let offset = if is_even { pi_sides } else { pi_sides * 0.5 };

        let angle = i * 2.0 * pi_sides - offset;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();

        if n == 0 {
            path_builder.move_to(x, y);
        } else {
            path_builder.line_to(x, y);
        }
    }

    path_builder.close();
}
