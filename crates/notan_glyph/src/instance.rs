use glyph_brush::ab_glyph::{point, Rect};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GlyphInstance {
    pub left_top: [f32; 3],
    pub right_bottom: [f32; 2],
    pub tex_left_top: [f32; 2],
    pub tex_right_bottom: [f32; 2],
    pub color: [f32; 4],
}

unsafe impl bytemuck::Zeroable for GlyphInstance {}
unsafe impl bytemuck::Pod for GlyphInstance {}

impl GlyphInstance {
    pub fn from_vertex(
        glyph_brush::GlyphVertex {
            mut tex_coords,
            pixel_coords,
            bounds,
            extra,
        }: glyph_brush::GlyphVertex,
    ) -> GlyphInstance {
        let gl_bounds = bounds;

        let mut gl_rect = Rect {
            min: point(pixel_coords.min.x, pixel_coords.min.y),
            max: point(pixel_coords.max.x, pixel_coords.max.y),
        };

        // handle overlapping bounds, modify uv_rect to preserve texture aspect
        if gl_rect.max.x > gl_bounds.max.x {
            let old_width = gl_rect.width();
            gl_rect.max.x = gl_bounds.max.x;
            tex_coords.max.x = tex_coords.min.x + tex_coords.width() * gl_rect.width() / old_width;
        }

        if gl_rect.min.x < gl_bounds.min.x {
            let old_width = gl_rect.width();
            gl_rect.min.x = gl_bounds.min.x;
            tex_coords.min.x = tex_coords.max.x - tex_coords.width() * gl_rect.width() / old_width;
        }

        if gl_rect.max.y > gl_bounds.max.y {
            let old_height = gl_rect.height();
            gl_rect.max.y = gl_bounds.max.y;
            tex_coords.max.y =
                tex_coords.min.y + tex_coords.height() * gl_rect.height() / old_height;
        }

        if gl_rect.min.y < gl_bounds.min.y {
            let old_height = gl_rect.height();
            gl_rect.min.y = gl_bounds.min.y;
            tex_coords.min.y =
                tex_coords.max.y - tex_coords.height() * gl_rect.height() / old_height;
        }

        GlyphInstance {
            left_top: [gl_rect.min.x, gl_rect.max.y, extra.z],
            right_bottom: [gl_rect.max.x, gl_rect.min.y],
            tex_left_top: [tex_coords.min.x, tex_coords.max.y],
            tex_right_bottom: [tex_coords.max.x, tex_coords.min.y],
            color: extra.color,
        }
    }
}
