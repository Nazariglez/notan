use crate::builder::DrawProcess;
use crate::draw::{Draw, ImageInfo};
use crate::transform::DrawTransform;
use notan_graphics::color::Color;
use notan_graphics::pipeline::BlendMode;
use notan_graphics::Texture;
use notan_math::glam::Mat3;

pub struct Pattern<'a> {
    matrix: Option<Mat3>,
    texture: &'a Texture,
    pos: (f32, f32),
    size: (f32, f32),
    offset: (f32, f32),
    scale: (f32, f32),
    color: Color,
    alpha: f32,
    blend_mode: Option<BlendMode>,
}

impl<'a> Pattern<'a> {
    pub fn new(texture: &'a Texture) -> Self {
        Self {
            matrix: None,
            texture,
            pos: (0.0, 0.0),
            color: Color::WHITE,
            alpha: 1.0,
            size: (texture.width(), texture.height()),
            offset: (0.0, 0.0),
            scale: (1.0, 1.0),
            blend_mode: None,
        }
    }

    pub fn position(&mut self, x: f32, y: f32) -> &mut Self {
        self.pos = (x, y);
        self
    }

    pub fn size(&mut self, width: f32, height: f32) -> &mut Self {
        self.size = (width, height);
        self
    }

    pub fn image_scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.scale = (x, y);
        self
    }

    pub fn image_offset(&mut self, x: f32, y: f32) -> &mut Self {
        self.offset = (x, y);
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

    pub fn blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        self.blend_mode = Some(mode);
        self
    }
}

impl DrawTransform for Pattern<'_> {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Pattern<'_> {
    fn draw_process(self, draw: &mut Draw) {
        let Self {
            pos: (x1, y1),
            texture,
            color,
            matrix,
            alpha,
            size: (width, height),
            scale: (sx, sy),
            offset: (ox, oy),
            blend_mode,
        } = self;

        let c = color.with_alpha(color.a * alpha);
        let frame = texture.frame();

        let x2 = x1 + width;
        let y2 = y1 + height;

        let sw = frame.width * sx;
        let sh = frame.height * sy;

        let ox = ((ox * sx) / sw).fract();
        let oy = ((oy * sy) / sh).fract();

        let uv_w = width / sw;
        let uv_h = height / sh;

        let u1 = ox;
        let v1 = oy;
        let u2 = uv_w + ox;
        let v2 = uv_h + oy;

        let base_width = texture.base_width();
        let base_height = texture.base_height();

        let fx = frame.x / base_width;
        let fy = frame.y / base_height;
        let fw = frame.width / base_width;
        let fh = frame.height / base_height;

        #[rustfmt::skip]
        let vertices = [
            x1, y1, u1, v1, fx, fy, fw, fh, c.r, c.g, c.b, c.a,
            x2, y1, u2, v1, fx, fy, fw, fh, c.r, c.g, c.b, c.a,
            x1, y2, u1, v2, fx, fy, fw, fh, c.r, c.g, c.b, c.a,
            x2, y2, u2, v2, fx, fy, fw, fh, c.r, c.g, c.b, c.a,
        ];

        draw.add_pattern(&ImageInfo {
            texture,
            transform: matrix.as_ref(),
            vertices: &vertices,
            indices: &[0, 1, 2, 2, 1, 3],
            blend_mode,
        });
    }
}
