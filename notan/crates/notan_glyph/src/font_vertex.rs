use glyph_brush::GlyphVertex;
use notan_graphics::prelude::*;

#[derive(Debug, Clone)]
pub struct FontVertex {
    pub pos: (f32, f32, f32),
    pub size: (f32, f32),
    pub uvs: [f32; 4],
    pub color: Color,
}

#[inline]
pub(crate) fn to_vertex(
    GlyphVertex {
        mut tex_coords,
        pixel_coords,
        bounds,
        extra,
    }: GlyphVertex,
) -> FontVertex {
    let x = pixel_coords.min.x;
    let y = pixel_coords.min.y;
    let z = extra.z;
    let width = pixel_coords.max.x - x;
    let height = pixel_coords.max.y - y;

    FontVertex {
        pos: (x, y, z),
        size: (width, height),
        uvs: [
            tex_coords.min.x,
            tex_coords.min.y,
            tex_coords.max.x,
            tex_coords.max.y,
        ],
        color: extra.color.into(),
    }
}
