use crate::builder::DrawProcess;
use crate::draw2::{Draw2, ImageInfo};
use crate::transform::{DrawTransform, Transform};
use glam::Mat3;
use notan_graphics::color::Color;
use notan_graphics::Texture;

pub struct Image<'a> {
    matrix: Option<Mat3>,
    texture: &'a Texture,
    pos: (f32, f32),
    color: Color,
    alpha: f32,
}

impl<'a> Image<'a> {
    pub fn new(texture: &'a Texture) -> Self {
        Self {
            matrix: None,
            texture: texture,
            pos: (0.0, 0.0),
            color: Color::WHITE,
            alpha: 1.0,
        }
    }

    //todo origin?
    pub fn position(&mut self, x: f32, y: f32) -> &mut Self {
        self.pos = (x, y);
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

impl DrawTransform for Image<'_> {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Image<'_> {
    fn draw_process(self, draw: &mut Draw2) {
        let Self {
            pos: (x1, y1),
            texture,
            color,
            matrix,
            alpha,
            ..
        } = self;

        let c = color.with_alpha(color.a * alpha);

        let ww = texture.base_width();
        let hh = texture.base_height();

        let x2 = x1 + ww;
        let y2 = y1 + hh;

        #[rustfmt::skip]
        let vertices = [
            x1, y1, 0.0, 0.0, c.r, c.g, c.b, c.a,
            x2, y1, 1.0, 0.0, c.r, c.g, c.b, c.a,
            x1, y2, 0.0, 1.0, c.r, c.g, c.b, c.a,
            x2, y2, 1.0, 1.0, c.r, c.g, c.b, c.a,
        ];

        draw.add_image(&ImageInfo {
            texture: self.texture,
            transform: self.matrix.as_ref(),
            vertices: &vertices,
            indices: &[0, 1, 2, 2, 1, 3],
        });
    }
}
