use crate::builder::DrawProcess;
use crate::draw::{Draw, ImageInfo};
use crate::transform::DrawTransform;
use notan_graphics::color::Color;
use notan_graphics::pipeline::BlendMode;
use notan_graphics::Texture;
use notan_math::Mat3;
use notan_math::Rect;

pub struct Image<'a> {
    matrix: Option<Mat3>,
    texture: &'a Texture,
    pos: (f32, f32),
    size: Option<(f32, f32)>,
    crop: Option<Rect>,
    color: Color,
    alpha: f32,
    blend_mode: Option<BlendMode>,
    alpha_mode: Option<BlendMode>,
    flip: (bool, bool),
}

impl<'a> Image<'a> {
    pub fn new(texture: &'a Texture) -> Self {
        Self {
            matrix: None,
            texture,
            pos: (0.0, 0.0),
            color: Color::WHITE,
            alpha: 1.0,
            size: None,
            crop: None,
            blend_mode: None,
            alpha_mode: None,
            flip: (false, false),
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

    pub fn crop(&mut self, xy: (f32, f32), size: (f32, f32)) -> &mut Self {
        let (x, y) = xy;
        let (width, height) = size;
        self.crop = Some(Rect {
            x,
            y,
            width,
            height,
        });
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

    pub fn alpha_mode(&mut self, mode: BlendMode) -> &mut Self {
        self.alpha_mode = Some(mode);
        self
    }

    pub fn flip_x(&mut self, flip: bool) -> &mut Self {
        self.flip.0 = flip;
        self
    }

    pub fn flip_y(&mut self, flip: bool) -> &mut Self {
        self.flip.1 = flip;
        self
    }
}

impl DrawTransform for Image<'_> {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for Image<'_> {
    fn draw_process(self, draw: &mut Draw) {
        let Self {
            pos: (x1, y1),
            texture,
            color,
            matrix,
            alpha,
            size,
            crop,
            blend_mode,
            alpha_mode,
            flip: (flip_x, flip_y),
        } = self;

        let c = color.with_alpha(color.a * alpha);
        let frame = texture.frame();

        let (ww, hh) = size.unwrap_or((frame.width, frame.height));
        let x2 = x1 + ww;
        let y2 = y1 + hh;

        let Rect {
            x: sx,
            y: sy,
            width: sw,
            height: sh,
        } = crop.map_or_else(
            || *frame,
            |mut r| {
                r.x += frame.x;
                r.y += frame.y;
                r
            },
        );

        let flip_y = if texture.is_render_texture() {
            !flip_y
        } else {
            flip_y
        };

        let (u1, v1, u2, v2) = {
            let base_width = texture.base_width();
            let base_height = texture.base_height();
            let u1 = sx / base_width;
            let v1 = sy / base_height;
            let u2 = (sx + sw) / base_width;
            let v2 = (sy + sh) / base_height;

            let (u1, u2) = if flip_x { (u2, u1) } else { (u1, u2) };
            let (v1, v2) = if flip_y { (v2, v1) } else { (v1, v2) };

            (u1, v1, u2, v2)
        };

        #[rustfmt::skip]
        let vertices = [
            x1, y1, u1, v1, c.r, c.g, c.b, c.a,
            x2, y1, u2, v1, c.r, c.g, c.b, c.a,
            x1, y2, u1, v2, c.r, c.g, c.b, c.a,
            x2, y2, u2, v2, c.r, c.g, c.b, c.a,
        ];

        draw.add_image(&ImageInfo {
            texture,
            transform: matrix.as_ref(),
            vertices: &vertices,
            indices: &[0, 1, 2, 2, 1, 3],
            blend_mode,
            alpha_mode,
        });
    }
}
