use crate::builder::DrawProcess;
use crate::draw::{Draw, TextInfo};
use crate::transform::DrawTransform;
use glam::Mat3;
use notan_glyph::{Font, Text};
use notan_graphics::color::Color;

pub struct TextSection<'a> {
    matrix: Option<Mat3>,
    text: Option<Text<'a>>,
    font: &'a Font,
}

impl<'a> TextSection<'a> {
    pub fn new(font: &'a Font, text: &'a str) -> Self {
        Self {
            matrix: None,
            font,
            text: Some(Text::new(text)),
        }
    }

    pub fn position(&mut self, x: f32, y: f32) -> &mut Self {
        if let Some(t) = self.text.take() {
            self.text = Some(t.position(x, y, 0.0));
        }

        self
    }

    pub fn size(&mut self, size: f32) -> &mut Self {
        if let Some(t) = self.text.take() {
            self.text = Some(t.size(size));
        }
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        if let Some(t) = self.text.take() {
            self.text = Some(t.color(color));
        }
        self
    }

    pub fn alpha(&mut self, alpha: f32) -> &mut Self {
        if let Some(t) = self.text.take() {
            self.text = Some(t.alpha(alpha));
        }
        self
    }

    pub fn max_width(&mut self, width: f32) -> &mut Self {
        if let Some(t) = self.text.take() {
            self.text = Some(t.max_width(width));
        }
        self
    }

    pub fn h_align_left(&mut self) -> &mut Self {
        if let Some(t) = self.text.take() {
            self.text = Some(t.h_align_left());
        }
        self
    }

    pub fn h_align_center(&mut self) -> &mut Self {
        if let Some(t) = self.text.take() {
            self.text = Some(t.h_align_center());
        }
        self
    }

    pub fn h_align_right(&mut self) -> &mut Self {
        if let Some(t) = self.text.take() {
            self.text = Some(t.h_align_right());
        }
        self
    }

    pub fn v_align_top(&mut self) -> &mut Self {
        if let Some(t) = self.text.take() {
            self.text = Some(t.v_align_top());
        }
        self
    }

    pub fn v_align_middle(&mut self) -> &mut Self {
        if let Some(t) = self.text.take() {
            self.text = Some(t.v_align_middle());
        }
        self
    }

    pub fn v_align_bottom(&mut self) -> &mut Self {
        if let Some(t) = self.text.take() {
            self.text = Some(t.v_align_bottom());
        }
        self
    }
}

impl DrawTransform for TextSection<'_> {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for TextSection<'_> {
    fn draw_process(self, draw: &mut Draw) {
        let Self { matrix, text, font } = self;

        draw.add_text(&TextInfo {
            transform: matrix.as_ref(),
            text: text.as_ref().unwrap(),
            font,
        });
    }
}
