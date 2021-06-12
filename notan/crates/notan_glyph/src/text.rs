use crate::font::Font;
use glyph_brush::Text as GlyphText;
use glyph_brush::{ab_glyph::*, *};
use notan_graphics::color::Color;

pub struct Text<'a> {
    text: &'a str,
    size: f32,
    width: Option<f32>,
    h_align: HorizontalAlign,
    v_align: VerticalAlign,
    xyz: [f32; 3],
    color: Color,
}

impl<'a> Text<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            size: 16.0,
            width: None,
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
            xyz: [0.0; 3],
            color: Color::WHITE,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.xyz = [x, y, z];
        self
    }

    pub fn max_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn h_align_left(mut self) -> Self {
        self.h_align = HorizontalAlign::Left;
        self
    }

    pub fn h_align_center(mut self) -> Self {
        self.h_align = HorizontalAlign::Center;
        self
    }

    pub fn h_align_right(mut self) -> Self {
        self.h_align = HorizontalAlign::Right;
        self
    }

    pub fn v_align_top(mut self) -> Self {
        self.v_align = VerticalAlign::Top;
        self
    }

    pub fn v_align_middle(mut self) -> Self {
        self.v_align = VerticalAlign::Center;
        self
    }

    pub fn v_align_bottom(mut self) -> Self {
        self.v_align = VerticalAlign::Bottom;
        self
    }

    pub fn size(mut self, value: f32) -> Self {
        self.size = value;
        self
    }
}

pub(crate) fn section_from_text<'a>(font: &Font, from: &Text<'a>) -> Section<'a> {
    let [x, y, z] = from.xyz;
    let width = from.width.unwrap_or(std::f32::INFINITY);

    let glyph_text = GlyphText::new(from.text)
        .with_scale(PxScale::from(from.size))
        .with_font_id(font.id)
        .with_z(z)
        .with_color(from.color.to_rgba());

    Section::default()
        .add_text(glyph_text)
        .with_screen_position((x, y))
        .with_layout(
            Layout::default()
                .h_align(from.h_align)
                .v_align(from.v_align),
        )
        .with_bounds((width, std::f32::INFINITY))
}
