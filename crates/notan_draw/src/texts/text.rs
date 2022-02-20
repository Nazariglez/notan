use crate::builder::DrawProcess;
use crate::draw::{Draw, TextInfo};
use crate::transform::DrawTransform;
use notan_glyph::{HorizontalAlign, Layout, Section, Text, VerticalAlign};
use notan_graphics::color::Color;
use notan_graphics::pipeline::BlendMode;
use notan_math::glam::Mat3;
use notan_text::Font;

pub struct TextSection<'a> {
    text: &'a str,
    matrix: Option<Mat3>,
    font: &'a Font,
    blend_mode: Option<BlendMode>,
    pos: (f32, f32),
    size: f32,
    color: Color,
    max_width: Option<f32>,
    h_align: HorizontalAlign,
    v_align: VerticalAlign,
    alpha: f32,
}

impl<'a> TextSection<'a> {
    pub fn new(font: &'a Font, text: &'a str) -> Self {
        Self {
            text,
            matrix: None,
            font,
            blend_mode: None,
            pos: (0.0, 0.0),
            size: 16.0,
            color: Color::WHITE,
            max_width: None,
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
            alpha: 1.0,
        }
    }

    pub fn position(&mut self, x: f32, y: f32) -> &mut Self {
        self.pos = (x, y);

        self
    }

    pub fn size(&mut self, size: f32) -> &mut Self {
        self.size = size;
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

    pub fn max_width(&mut self, width: f32) -> &mut Self {
        self.max_width = Some(width);
        self
    }

    pub fn h_align_left(&mut self) -> &mut Self {
        self.h_align = HorizontalAlign::Left;
        self
    }

    pub fn h_align_center(&mut self) -> &mut Self {
        self.h_align = HorizontalAlign::Center;
        self
    }

    pub fn h_align_right(&mut self) -> &mut Self {
        self.h_align = HorizontalAlign::Right;
        self
    }

    pub fn v_align_top(&mut self) -> &mut Self {
        self.v_align = VerticalAlign::Top;
        self
    }

    pub fn v_align_middle(&mut self) -> &mut Self {
        self.v_align = VerticalAlign::Center;
        self
    }

    pub fn v_align_bottom(&mut self) -> &mut Self {
        self.v_align = VerticalAlign::Bottom;
        self
    }

    pub fn blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        self.blend_mode = Some(mode);
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
        let Self {
            text,
            matrix,
            font,
            blend_mode,
            pos,
            size,
            color,
            max_width,
            h_align,
            v_align,
            alpha,
        } = self;

        let color = color.with_alpha(alpha);

        let count = text.chars().filter(|c| !c.is_whitespace()).count();

        let g_text = Text::new(text)
            .with_color(color.rgba())
            .with_scale(size)
            .with_font_id(font);

        let mut section = Section::default()
            .add_text(g_text)
            .with_layout(Layout::default().h_align(h_align).v_align(v_align));

        section.screen_position = pos;
        if let Some(mw) = max_width {
            section.bounds.0 = mw;
        }

        draw.add_text(&TextInfo {
            count,
            section: &section,
            transform: matrix.as_ref(),
            font,
            blend_mode,
        });
    }
}
