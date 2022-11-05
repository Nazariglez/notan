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

pub struct Star {
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
    spikes: u8,
    outer_radius: f32,
    inner_radius: f32,
}

impl Star {
    pub fn new(spikes: u8, outer_radius: f32, inner_radius: f32) -> Self {
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
            spikes,
            outer_radius,
            inner_radius,
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

impl DrawTransform for Star {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Star {
    fn draw_process(self, draw: &mut Draw) {
        let mut path_builder = draw.path();
        draw_star(
            &mut path_builder,
            self.pos.0,
            self.pos.1,
            self.spikes as _,
            self.outer_radius,
            self.inner_radius,
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

fn draw_star(
    path_builder: &mut DrawBuilder<Path>,
    center_x: f32,
    center_y: f32,
    spikes: usize,
    outer_radius: f32,
    inner_radius: f32,
) {
    let step = PI / spikes as f32;

    path_builder.move_to(center_x, center_y - outer_radius);

    let mut rot = PI / 2.0 * 3.0;
    for _ in 0..spikes {
        let mut x = center_x + rot.cos() * outer_radius;
        let mut y = center_y + rot.sin() * outer_radius;
        rot += step;

        path_builder.line_to(x, y);

        x = center_x + rot.cos() * inner_radius;
        y = center_y + rot.sin() * inner_radius;
        rot += step;

        path_builder.line_to(x, y);
    }

    path_builder
        .line_to(center_x, center_y - outer_radius)
        .close();
}
