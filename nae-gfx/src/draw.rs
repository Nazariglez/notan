use nae_core::{
    BaseGfx, BasePipeline, BlendMode, ClearOptions, Color, CompareMode, DrawUsage, Geometry,
    GraphicsAPI, PipelineOptions, StencilAction, StencilOptions,
};

use crate::batchers::{BaseBatcher, ColorBatcher, ImageBatcher, PatternBatcher};
use crate::shapes::ShapeTessellator;
use crate::texture::Texture;
use crate::{
    matrix4_identity, matrix4_mul_matrix4, matrix4_mul_vector4, matrix4_orthogonal,
    matrix4_rotation_z, matrix4_scale, matrix4_skew, matrix4_translate, Device, Graphics,
    IndexBuffer, Matrix4, Pipeline, RenderTarget, Shader, Uniform, VertexAttr, VertexBuffer,
    VertexFormat,
};
use glow::HasContext;
use std::cell::RefMut;
use std::convert::TryInto;

pub struct Draw {
    pub gfx: Graphics,
    pub depth: f32,
    pub color: Color,
    pub alpha: f32,
    pub blend_mode: BlendMode,
    pub projection: Option<Matrix4>,
    pub matrix: Option<Matrix4>,
    pub shader: Option<Shader>,

    last_blend_mode: BlendMode,
    last_shader: Option<Shader>,
    last_paint_mode: PaintMode,

    render_projection: Matrix4,
    matrix_stack: Vec<Matrix4>,
    clear_options: ClearOptions,
    color_batcher: ColorBatcher,
    image_batcher: ImageBatcher,
    pattern_batcher: PatternBatcher,
    current_mode: PaintMode,
    shapes: ShapeTessellator,
    mask: MaskMode,
}

impl Draw {
    pub fn new(device: &Device) -> Result<Self, String> {
        let mut gfx = Graphics::new(device)?;
        let color_batcher = ColorBatcher::new(&mut gfx)?;
        let image_batcher = ImageBatcher::new(&mut gfx)?;
        let pattern_batcher = PatternBatcher::new(&mut gfx)?;

        let (width, height) = gfx.size(); //TODO multiply for dpi
        let render_projection = matrix4_orthogonal(0.0, width, height, 0.0, -1.0, 1.0);

        let blend_mode = BlendMode::NORMAL;
        let paint_mode = PaintMode::None;

        Ok(Self {
            gfx,
            clear_options: Default::default(),
            color: Color::WHITE,
            alpha: 1.0,
            depth: 0.0,
            blend_mode,
            current_mode: paint_mode,
            matrix_stack: vec![matrix4_identity()],
            shader: None,

            last_blend_mode: blend_mode,
            last_shader: None,
            last_paint_mode: paint_mode,

            color_batcher,
            image_batcher,
            pattern_batcher,
            matrix: None,
            projection: None,
            render_projection,
            shapes: ShapeTessellator::new(),
            mask: MaskMode::None,
        })
    }

    pub fn start_mask<T: FnMut(&mut Self)>(&mut self, mut mask: T) {
        debug_assert!(self.gfx.running, "Graphics pass should be already running.");
        debug_assert!(self.mask == MaskMode::None, "Already writing to a mask.");

        flush(self);

        self.mask = MaskMode::Drawing;
        self.clear_options.stencil = Some(0xff);

        clear_mask(self);
        mask(self);

        flush(self);

        self.mask = MaskMode::Masking;
    }

    pub fn end_mask(&mut self) {
        flush(self);
        self.clear_options.stencil = None;
        self.mask = MaskMode::None;
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.gfx.set_size(width, height);
        self.render_projection = projection(width, height, self.gfx.render_target.is_some());
    }

    pub fn size(&self) -> (f32, f32) {
        self.gfx.size()
    }

    pub fn push(&mut self, matrix: &Matrix4) {
        let new_matrix = matrix4_mul_matrix4(self.transform(), matrix);
        self.matrix_stack.push(new_matrix);
    }

    pub fn pop(&mut self) {
        if self.matrix_stack.len() <= 1 {
            return;
        }

        self.matrix_stack.pop();
    }

    pub fn push_scale(&mut self, x: f32, y: f32) {
        self.push(&matrix4_scale(x, y, 1.0));
    }

    pub fn push_skew(&mut self, x: f32, y: f32) {
        self.push(&matrix4_skew(x, y));
    }

    pub fn push_translation(&mut self, x: f32, y: f32) {
        self.push(&matrix4_translate(x, y, 0.0));
    }

    pub fn push_rotation(&mut self, angle: f32) {
        self.push(&matrix4_rotation_z(angle));
    }

    pub fn transform(&mut self) -> &Matrix4 {
        self.matrix_stack.last().as_ref().unwrap()
    }

    pub fn begin(&mut self, color: Color) {
        if self.gfx.render_target.is_some() {
            self.render_projection = projection(self.gfx.width, self.gfx.height, false);
        }

        self.clear_options.color = Some(color);
        self.gfx.begin(&self.clear_options);
    }

    pub fn begin_to(&mut self, target: &RenderTarget, color: Color) {
        self.render_projection = projection(target.width(), target.height(), true);

        self.clear_options.color = Some(color);
        self.gfx.begin_to(Some(target), &self.clear_options);
    }

    pub fn end(&mut self) {
        paint_mode(self, PaintMode::None);
        self.gfx.end();
    }

    pub fn geometry(&mut self, geometry: &Geometry) {
        paint_mode(self, PaintMode::Color);
        geometry.data().iter().for_each(|data| {
            draw_color(self, &data.vertices, &data.indices, Some(data.color));
        });
    }

    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width: f32) {
        paint_mode(self, PaintMode::Color);

        let (mut xx, mut yy) = if y1 == y2 {
            (0.0, -1.0)
        } else {
            (1.0, -(x2 - x1) / (y2 - y1))
        };

        let len = (xx * xx + yy * yy).sqrt();
        if len != 0.0 {
            let mul = width / len;
            xx *= mul;
            yy *= mul;
        }

        let px1 = x1 + 0.5 * xx;
        let py1 = y1 + 0.5 * yy;
        let px2 = x2 + 0.5 * xx;
        let py2 = y2 + 0.5 * yy;
        let px3 = px1 - xx;
        let py3 = py1 - yy;
        let px4 = px2 - xx;
        let py4 = py2 - yy;

        #[rustfmt::skip]
            draw_color(
            self,
            &[
                px1, py1, self.depth,
                px2, py2, self.depth,
                px3, py3, self.depth,
                px3, py3, self.depth,
                px2, py2, self.depth,
                px4, py4, self.depth
            ],
            &[
                0, 1, 2, 3, 4, 5
            ],
            None,
        );
    }

    pub fn stroke_triangle(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        line_width: f32,
    ) {
        paint_mode(self, PaintMode::Color);
        let (vertices, indices) = self
            .shapes
            .stroke_triangle(x1, y1, x2, y2, x3, y3, line_width, self.depth);

        draw_color(self, &vertices, &indices, None);
    }

    pub fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        paint_mode(self, PaintMode::Color);

        #[rustfmt::skip]
            draw_color(
            self,
            &[
                x1, y1, self.depth,
                x2, y2, self.depth,
                x3, y3, self.depth
            ],
            &[0, 1, 2],
            None,
        );
    }

    pub fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32, line_width: f32) {
        paint_mode(self, PaintMode::Color);
        let (vertices, indices) = self
            .shapes
            .stroke_rect(x, y, width, height, line_width, self.depth);

        draw_color(self, &vertices, &indices, None);
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        paint_mode(self, PaintMode::Color);

        let x2 = x + width;
        let y2 = y + height;

        #[rustfmt::skip]
            draw_color(
            self,
            &[
                x, y, self.depth,
                x2, y, self.depth,
                x, y2, self.depth,
                x2, y2, self.depth,
            ],
            &[0, 1, 2, 2, 1, 3],
            None
        );
    }

    pub fn stroke_circle(&mut self, x: f32, y: f32, radius: f32, line_width: f32) {
        paint_mode(self, PaintMode::Color);
        let (vertices, indices) = self
            .shapes
            .stroke_circle(x, y, radius, line_width, self.depth);

        draw_color(self, &vertices, &indices, None);
    }

    pub fn circle(&mut self, x: f32, y: f32, radius: f32) {
        paint_mode(self, PaintMode::Color);
        let (vertices, indices) = self.shapes.circle(x, y, radius, self.depth);

        draw_color(self, &vertices, &indices, None);
    }

    pub fn rounded_rect(&mut self, x: f32, y: f32, width: f32, height: f32, corner_radius: f32) {
        paint_mode(self, PaintMode::Color);
        let (vertices, indices) =
            self.shapes
                .rounded_rect(x, y, width, height, corner_radius, self.depth);

        draw_color(self, &vertices, &indices, None);
    }

    pub fn stroke_rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        corner_radius: f32,
        line_width: f32,
    ) {
        paint_mode(self, PaintMode::Color);
        let (vertices, indices) = self.shapes.stroke_rounded_rect(
            x,
            y,
            width,
            height,
            line_width,
            corner_radius,
            self.depth,
        );

        draw_color(self, &vertices, &indices, None);
    }

    pub fn image(&mut self, img: &Texture, x: f32, y: f32) {
        self.image_ext(img, x, y, img.width(), img.height(), 0.0, 0.0, 0.0, 0.0);
    }

    pub fn image_resized(&mut self, img: &Texture, x: f32, y: f32, width: f32, height: f32) {
        self.image_ext(img, x, y, width, height, 0.0, 0.0, 0.0, 0.0);
    }

    pub fn image_crop(
        &mut self,
        img: &Texture,
        x: f32,
        y: f32,
        source_x: f32,
        source_y: f32,
        source_width: f32,
        source_height: f32,
    ) {
        self.image_ext(
            img,
            x,
            y,
            source_width,
            source_height,
            source_x,
            source_y,
            source_width,
            source_height,
        );
    }

    pub fn image_ext(
        &mut self,
        img: &Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        source_x: f32,
        source_y: f32,
        source_width: f32,
        source_height: f32,
    ) {
        if !img.is_loaded() {
            return;
        }

        let frame = img.frame();
        let base_width = img.base_width();
        let base_height = img.base_height();

        let ww = if width == 0.0 {
            frame.width //frame or base_width?
        } else {
            width
        };
        let hh = if height == 0.0 { frame.height } else { height };

        let x2 = x + ww;
        let y2 = y + hh;

        let sx = frame.x + source_x;
        let sy = frame.y + source_y;
        let sw = if source_width == 0.0 {
            frame.width
        } else {
            source_width
        };
        let sh = if source_height == 0.0 {
            frame.height
        } else {
            source_height
        };

        let sx1 = sx / base_width;
        let sy1 = sy / base_height;
        let sx2 = (sx + sw) / base_width;
        let sy2 = (sy + sh) / base_height;

        //http://webglstats.com/webgl/parameter/MAX_TEXTURE_IMAGE_UNITS
        paint_mode(self, PaintMode::Image);

        #[rustfmt::skip]
        draw_image(
            self,
            img,
            &[
                x, y, self.depth,
                x2, y, self.depth,
                x, y2, self.depth,
                x2, y2, self.depth,
            ],
            &[
                sx1, sy1,
                sx2, sy1,
                sx1, sy2,
                sx2, sy2
            ],
            &[
                0, 1, 2, 2, 1, 3
            ]
        );
    }

    pub fn image_9slice(&mut self, img: &Texture, x: f32, y: f32, width: f32, height: f32) {
        let ww = img.width() / 3.0;
        let hh = img.height() / 3.0;
        self.image_9slice_ext(img, x, y, width, height, ww, ww, hh, hh);
    }

    pub fn image_9slice_ext(
        &mut self,
        img: &Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
    ) {
        let center_sw = img.width() - (left + right);
        let center_sh = img.height() - (top + bottom);
        let center_w = width - (left + right);
        let center_h = height - (top + bottom);

        self.image_crop(img, x, y, 0.0, 0.0, left, top);
        self.image_ext(img, x + left, y, center_w, top, left, 0.0, center_sw, top);
        self.image_crop(
            img,
            x + left + center_w,
            y,
            left + center_sw,
            0.0,
            right,
            top,
        );

        self.image_ext(img, x, y + top, left, center_h, 0.0, top, left, center_sh);
        self.image_ext(
            img,
            x + left,
            y + top,
            center_w,
            center_h,
            left,
            top,
            center_sw,
            center_sh,
        );
        self.image_ext(
            img,
            x + left + center_w,
            y + top,
            right,
            center_h,
            left + center_sw,
            top,
            right,
            center_sh,
        );

        self.image_crop(
            img,
            x,
            y + top + center_h,
            0.0,
            top + center_sh,
            left,
            bottom,
        );
        self.image_ext(
            img,
            x + left,
            y + top + center_h,
            center_w,
            bottom,
            left,
            top + center_sh,
            center_sw,
            bottom,
        );
        self.image_crop(
            img,
            x + left + center_w,
            y + top + center_h,
            left + center_sw,
            top + center_sh,
            right,
            bottom,
        );
    }

    pub fn pattern(
        &mut self,
        img: &Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        self.pattern_ext(img, x, y, width, height, offset_x, offset_y, 1.0, 1.0);
    }

    pub fn pattern_ext(
        &mut self,
        img: &Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        offset_x: f32,
        offset_y: f32,
        scale_x: f32,
        scale_y: f32,
    ) {
        if !img.is_loaded() {
            return;
        }

        let x2 = x + width;
        let y2 = y + height;

        let frame = img.frame();
        let offset_x = offset_x * scale_x;
        let offset_y = offset_y * scale_y;
        let ww = frame.width * scale_x;
        let hh = frame.height * scale_y;

        let tex_uv_width = width / ww;
        let tex_uv_height = height / hh;

        let offset_x = (offset_x / ww).fract();
        let offset_y = (offset_y / hh).fract();
        let sx1 = tex_uv_width + offset_x;
        let sy1 = tex_uv_height + offset_y;
        let sx2 = offset_x;
        let sy2 = offset_y;

        paint_mode(self, PaintMode::Pattern);

        #[rustfmt::skip]
        draw_pattern(
            self,
            img,
            &[
                x, y, self.depth,
                x2, y, self.depth,
                x, y2, self.depth,
                x2, y2, self.depth,
            ],
            &[
                sx2, sy2,
                sx1, sy2,
                sx2, sy1,
                sx1, sy1,
            ],
            &[
                0, 1, 2, 2, 1, 3
            ]
        );
    }
}

fn projection(width: f32, height: f32, is_flipped: bool) -> Matrix4 {
    match is_flipped {
        true => matrix4_orthogonal(0.0, width, 0.0, height, -1.0, 1.0),
        false => matrix4_orthogonal(0.0, width, height, 0.0, -1.0, 1.0),
    }
}

fn clear_mask(draw: &mut Draw) {
    let mut batcher: &mut BaseBatcher = match draw.last_paint_mode {
        PaintMode::Color => &mut draw.color_batcher,
        PaintMode::Image => &mut draw.image_batcher,
        PaintMode::Pattern => &mut draw.pattern_batcher,
        _ => return,
    };

    batcher.clear_mask(&mut draw.gfx, &draw.mask, Color::TRANSPARENT);
}

fn flush(draw: &mut Draw) {
    let mut batcher: &mut BaseBatcher = match draw.last_paint_mode {
        PaintMode::Color => &mut draw.color_batcher,
        PaintMode::Image => &mut draw.image_batcher,
        PaintMode::Pattern => &mut draw.pattern_batcher,
        _ => return,
    };

    batcher.flush(
        &mut draw.gfx,
        match &draw.projection {
            Some(p) => p,
            _ => &draw.render_projection,
        },
        &draw.mask,
    );
}

pub(crate) struct DrawParams<'a> {
    gfx: &'a mut Graphics,
    projection: &'a Matrix4,
    mask: &'a MaskMode,
}

fn flush_if_necessary(draw: &mut Draw) {
    let need_flush = draw.current_mode != draw.last_paint_mode
        || draw.blend_mode != draw.last_blend_mode
        || draw.shader != draw.last_shader;
    if need_flush {
        flush(draw);
    }

    draw.last_shader = draw.shader.clone();
    draw.last_blend_mode = draw.blend_mode;
    draw.last_paint_mode = draw.current_mode;
}

fn paint_mode(draw: &mut Draw, mode: PaintMode) {
    draw.current_mode = mode;
    flush_if_necessary(draw);
}

fn draw_color(draw: &mut Draw, vertices: &[f32], indices: &[u32], color: Option<Color>) {
    draw.color_batcher.push_data(
        &mut draw.gfx,
        DrawData {
            vertices,
            indices,
            projection: match &draw.projection {
                Some(p) => p,
                _ => &draw.render_projection,
            },
            matrix: match &draw.matrix {
                Some(p) => p,
                _ => &draw.matrix_stack.last().as_ref().unwrap(),
            },
            blend: Some(draw.last_blend_mode),
            color: color.unwrap_or(draw.color),
            alpha: draw.alpha,
            mask: &draw.mask,
        },
    );
}

fn draw_image(draw: &mut Draw, texture: &Texture, vertices: &[f32], uvs: &[f32], indices: &[u32]) {
    draw.image_batcher.push_data(
        &mut draw.gfx,
        texture,
        uvs,
        DrawData {
            vertices,
            indices,
            projection: match &draw.projection {
                Some(p) => p,
                _ => &draw.render_projection,
            },
            matrix: match &draw.matrix {
                Some(p) => p,
                _ => &draw.matrix_stack.last().as_ref().unwrap(),
            },
            blend: Some(draw.last_blend_mode),
            color: draw.color,
            alpha: draw.alpha,
            mask: &draw.mask,
        },
    )
}

fn draw_pattern(
    draw: &mut Draw,
    texture: &Texture,
    vertices: &[f32],
    uvs: &[f32],
    indices: &[u32],
) {
    draw.pattern_batcher.push_data(
        &mut draw.gfx,
        texture,
        uvs,
        DrawData {
            vertices,
            indices,
            projection: match &draw.projection {
                Some(p) => p,
                _ => &draw.render_projection,
            },
            matrix: match &draw.matrix {
                Some(p) => p,
                _ => &draw.matrix_stack.last().as_ref().unwrap(),
            },
            blend: Some(draw.last_blend_mode),
            color: draw.color,
            alpha: draw.alpha,
            mask: &draw.mask,
        },
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum MaskMode {
    None,
    Drawing,
    Masking,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PaintMode {
    None,
    Color,
    Image,
    Pattern,
    Text,
    //Particles?
}

pub(crate) struct DrawData<'data> {
    pub vertices: &'data [f32],
    pub indices: &'data [u32],
    pub color: Color,
    pub alpha: f32,
    pub blend: Option<BlendMode>,
    pub projection: &'data Matrix4,
    pub matrix: &'data Matrix4,
    pub mask: &'data MaskMode,
}
