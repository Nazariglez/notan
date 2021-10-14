use super::image::Image;
use crate::builder::{DrawBuilder, DrawProcess};
use crate::draw::Draw;
use crate::transform::DrawTransform;
use crate::DrawImages;
use notan_graphics::color::Color;
use notan_graphics::pipeline::BlendMode;
use notan_graphics::Texture;
use notan_math::glam::Mat3;

enum TextureSource<'a> {
    Grid {
        texture: &'a Texture,
        cols: usize,
        rows: usize,
    },
    List(&'a [&'a Texture]),
}

pub struct ImageAnimation<'a> {
    source: TextureSource<'a>,
    color: Color,
    alpha: f32,
    pos: (f32, f32),
    size: Option<(f32, f32)>,
    time: f32,
    frames: Option<&'a [usize]>,
    matrix: Option<Mat3>,
    blend_mode: Option<BlendMode>,
}

impl<'a> ImageAnimation<'a> {
    pub fn from_grid(texture: &'a Texture, cols: usize, rows: usize) -> Self {
        Self {
            source: TextureSource::Grid {
                texture,
                cols,
                rows,
            },
            color: Color::WHITE,
            alpha: 1.0,
            pos: (0.0, 0.0),
            size: None,
            matrix: None,
            blend_mode: None,
            frames: None,
            time: 0.0,
        }
    }

    pub fn from_list(list: &'a [&'a Texture]) -> Self {
        Self {
            source: TextureSource::List(list),
            color: Color::WHITE,
            alpha: 1.0,
            pos: (0.0, 0.0),
            size: None,
            matrix: None,
            blend_mode: None,
            frames: None,
            time: 0.0,
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

    pub fn frames(&mut self, frames: &'a [usize]) -> &mut Self {
        self.frames = Some(frames);
        self
    }

    pub fn time(&mut self, time: f32) -> &mut Self {
        if time < 0.0 {
            self.time = 1.0 + (time % 1.0)
        } else {
            self.time = time % 1.0;
        };

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

impl DrawTransform for ImageAnimation<'_> {
    fn matrix(&mut self) -> &mut Option<Mat3> {
        &mut self.matrix
    }
}

impl DrawProcess for ImageAnimation<'_> {
    fn draw_process(self, draw: &mut Draw) {
        let Self {
            color,
            alpha,
            pos: (x, y),
            size,
            time,
            source,
            matrix,
            blend_mode,
            frames,
        } = self;

        match source {
            TextureSource::Grid {
                texture,
                cols,
                rows,
            } => {
                let (tex_w, tex_h) = texture.size();
                let tw = tex_w as usize / cols;
                let th = tex_h as usize / rows;

                let cs = cols as f32;
                let rs = rows as f32;

                let i = match frames {
                    None => (cs * rs * time).floor() as usize,
                    Some(f) => {
                        debug_assert!(f.iter().max().map(|v| *v < (cols * rows)).unwrap_or(false));

                        let i = (f.len() as f32 * time).floor() as usize;
                        f[i]
                    }
                };

                let xx = i % cols;
                let yy = i / cols;
                let tx = xx * tw;
                let ty = yy * th;

                let size = size.unwrap_or((tw as _, th as _));

                img(draw, texture, matrix, blend_mode)
                    .crop((tx as _, ty as _), (tw as _, th as _))
                    .size(size.0, size.1)
                    .position(x, y)
                    .color(color)
                    .alpha(alpha);
            }
            TextureSource::List(list) => {
                let i = match frames {
                    None => (list.len() as f32 * time).floor() as usize,
                    Some(f) => {
                        debug_assert!(f.iter().max().map(|v| *v < list.len()).unwrap_or(false));

                        let i = (f.len() as f32 * time).floor() as usize;
                        f[i]
                    }
                };

                let texture = list[i];
                let size = size.unwrap_or_else(|| texture.size());
                img(draw, texture, matrix, blend_mode)
                    .size(size.0, size.1)
                    .position(x, y)
                    .color(color)
                    .alpha(alpha);
            }
        }
    }
}

#[inline(always)]
fn img<'a>(
    draw: &'a mut Draw,
    tex: &'a Texture,
    mat: Option<Mat3>,
    blend_mode: Option<BlendMode>,
) -> DrawBuilder<'a, Image<'a>> {
    let mut img = Image::new(tex);
    if let Some(bm) = blend_mode {
        img.blend_mode(bm);
    }
    if let Some(m) = mat {
        img.transform(m);
    }

    DrawBuilder::new(draw, img)
}
