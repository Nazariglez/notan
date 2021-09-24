
pub use crate::text::Text;

use glyph_brush::{*};
// use notan_graphics::color::Color;
use notan_app::graphics::color::Color;

/// Represents a Text object with options
#[derive(Debug, Clone)]
pub struct OwnedText {
    text: String,
    size: f32,
    width: Option<f32>,
    h_align: HorizontalAlign,
    v_align: VerticalAlign,
    xyz: [f32; 3],
    color: Color,
    alpha: f32,
}

impl OwnedText {
    /// Create a new Text object with the string passed in
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
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
        &self.text
    }
}

impl From<&Text<'_>> for OwnedText {
    fn from(text: &Text) -> Self {
        Self {
            text: text.text.into(),
            size: text.size,
            width: text.width,
            h_align: text.h_align,
            v_align: text.v_align,
            xyz: text.xyz,
            color: text.color,
            alpha: text.alpha,
        }
    }
}

impl<'a> From<&'a OwnedText> for Text<'a> {
    fn from(text: &'a OwnedText) -> Self {
        Self {
            text: &text.text,
            size: text.size,
            width: text.width,
            h_align: text.h_align,
            v_align: text.v_align,
            xyz: text.xyz,
            color: text.color,
            alpha: text.alpha,
        }
    }
}
