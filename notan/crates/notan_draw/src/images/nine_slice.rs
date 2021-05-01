use super::image::Image;
use crate::builder::{DrawBuilder, DrawProcess};
use crate::draw2::Draw2;
use crate::transform::{DrawTransform, Transform};
use glam::Mat3;
use notan_graphics::color::Color;
use notan_graphics::Texture;

pub struct NineSlice<'a> {
    texture: &'a Texture,
    color: Color,
    alpha: f32,
    pos: (f32, f32),
    size: Option<(f32, f32)>,
    left: Option<f32>,
    right: Option<f32>,
    top: Option<f32>,
    bottom: Option<f32>,
    matrix: Option<Mat3>,
}

impl<'a> NineSlice<'a> {
    pub fn new(texture: &'a Texture) -> Self {
        Self {
            texture: texture,
            color: Color::WHITE,
            alpha: 1.0,
            pos: (0.0, 0.0),
            size: None,
            left: None,
            right: None,
            top: None,
            bottom: None,
            matrix: None,
        }
    }

    pub fn position(&mut self, x: f32, y: f32) -> &mut Self {
        self.pos = (x, y);
        self
    }

    pub fn size(&mut self, width: f32, height: f32) -> &mut Self {
        self.size = Some((width, height));
        self
    }

    pub fn left(&mut self, left: f32) -> &mut Self {
        self.left = Some(left);
        self
    }

    pub fn right(&mut self, right: f32) -> &mut Self {
        self.right = Some(right);
        self
    }

    pub fn top(&mut self, top: f32) -> &mut Self {
        self.top = Some(top);
        self
    }

    pub fn bottom(&mut self, bottom: f32) -> &mut Self {
        self.bottom = Some(bottom);
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
}

impl DrawTransform for NineSlice<'_> {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for NineSlice<'_> {
    fn draw_process(self, draw: &mut Draw2) {
        let Self {
            texture,
            color,
            alpha,
            pos: (x, y),
            size,
            left,
            right,
            top,
            bottom,
            matrix,
        } = self;

        let img_ww = texture.width();
        let img_hh = texture.height();

        let (width, height) = size.unwrap_or_else(|| (img_ww, img_hh));
        let left = left.unwrap_or_else(|| img_ww / 3.0);
        let right = right.unwrap_or_else(|| img_ww / 3.0);
        let top = top.unwrap_or_else(|| img_hh / 3.0);
        let bottom = bottom.unwrap_or_else(|| img_hh / 3.0);
        let center_w = width - (left + right);
        let center_h = height - (top + bottom);
        let center_img_w = img_ww - (left + right);
        let center_img_h = img_hh - (top + bottom);

        let uses_matrix = match matrix {
            Some(m) => {
                draw.transform().push(m);
                true
            }
            _ => false,
        };

        //top-left
        img(draw, texture)
            .color(color)
            .alpha(alpha)
            .position(x, y)
            .size(left, top)
            .crop((0.0, 0.0), (left, top));
        //top-center
        img(draw, texture)
            .color(color)
            .alpha(alpha)
            .position(x + left, y)
            .size(center_w, top)
            .crop((left, 0.0), (center_img_w, top));
        //top-right
        img(draw, texture)
            .color(color)
            .alpha(alpha)
            .position(x + left + center_w, y)
            .size(right, top)
            .crop((left + center_img_w, 0.0), (right, top));

        //center-left
        img(draw, texture)
            .color(color)
            .alpha(alpha)
            .position(x, y + top)
            .size(left, center_h)
            .crop((0.0, top), (left, center_img_h));
        //center-center
        img(draw, texture)
            .color(color)
            .alpha(alpha)
            .position(x + left, y + top)
            .size(center_w, center_h)
            .crop((left, top), (center_img_w, center_img_h));
        //center-right
        img(draw, texture)
            .color(color)
            .alpha(alpha)
            .position(x + left + center_w, y + top)
            .size(right, center_h)
            .crop((left + center_img_w, top), (right, center_img_h));

        //bottom-left
        img(draw, texture)
            .color(color)
            .alpha(alpha)
            .position(x, y + top + center_h)
            .size(left, bottom)
            .crop((0.0, top + center_img_h), (left, bottom));
        //bottom-center
        img(draw, texture)
            .color(color)
            .alpha(alpha)
            .position(x + left, y + top + center_h)
            .size(center_w, bottom)
            .crop((left, top + center_img_h), (center_img_w, bottom));
        //bottom-right
        img(draw, texture)
            .color(color)
            .alpha(alpha)
            .size(right, bottom)
            .position(x + left + center_w, y + top + center_h)
            .crop((left + center_img_w, top + center_img_h), (right, top));

        if uses_matrix {
            draw.transform().pop();
        }
    }
}

#[inline(always)]
fn img<'a>(draw: &'a mut Draw2, tex: &'a Texture) -> DrawBuilder<'a, Image<'a>> {
    DrawBuilder::new(draw, Image::new(tex))
}
