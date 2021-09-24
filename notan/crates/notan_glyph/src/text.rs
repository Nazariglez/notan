use crate::font::Font;
use glyph_brush::Text as GlyphText;
use glyph_brush::{ab_glyph::*, *};
// use notan_graphics::color::Color;
use notan_app::graphics::color::Color;

/// Represents a Text object with options
#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub(crate) text: &'a str,
    pub(crate) size: f32,
    pub(crate) width: Option<f32>,
    pub(crate) h_align: HorizontalAlign,
    pub(crate) v_align: VerticalAlign,
    pub(crate) xyz: [f32; 3],
    pub(crate) color: Color,
    pub(crate) alpha: f32,
}

impl<'a> Text<'a> {
    /// Create a new Text object with the string passed in
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            size: 16.0,
            width: None,
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
            xyz: [0.0; 3],
            color: Color::WHITE,
            alpha: 1.0,
        }
    }

    /// Sets the text's color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the text's position on screen
    pub fn position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.xyz = [x, y, z];
        self
    }

    /// Sets the text's alpha
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Sets the max widht used to render the text. Any text outside this will be move to a new line
    pub fn max_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the horizontal align to left
    pub fn h_align_left(mut self) -> Self {
        self.h_align = HorizontalAlign::Left;
        self
    }

    /// Sets the horizontal align to the center
    pub fn h_align_center(mut self) -> Self {
        self.h_align = HorizontalAlign::Center;
        self
    }

    /// Sets the horizontal align to rigth
    pub fn h_align_right(mut self) -> Self {
        self.h_align = HorizontalAlign::Right;
        self
    }

    /// Sets the vertical align to the top
    pub fn v_align_top(mut self) -> Self {
        self.v_align = VerticalAlign::Top;
        self
    }

    /// Sets the vertical align to the middle
    pub fn v_align_middle(mut self) -> Self {
        self.v_align = VerticalAlign::Center;
        self
    }

    /// Sets the vertical align to the bottom
    pub fn v_align_bottom(mut self) -> Self {
        self.v_align = VerticalAlign::Bottom;
        self
    }

    /// Sets the text font's size
    pub fn size(mut self, value: f32) -> Self {
        self.size = value;
        self
    }

    pub fn text(&self) -> &str {
        self.text
    }
}

pub(crate) fn section_from_text<'a>(font: &Font, from: &Text<'a>) -> Section<'a> {
    let [x, y, z] = from.xyz;
    let width = from.width.unwrap_or(std::f32::INFINITY);

    let color = from.color.with_alpha(from.color.a * from.alpha);

    let glyph_text = GlyphText::new(from.text)
        .with_scale(PxScale::from(from.size))
        .with_font_id(font.id)
        .with_z(z)
        .with_color(color.rgba());

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
