use lyon::math::{point, vector, Angle, Box2D};
use lyon::path::builder::BorderRadii;
use lyon::path::Path;
use lyon::path::Winding;

pub(super) fn rectangle(x: f32, y: f32, width: f32, height: f32) -> Path {
    let mut builder = Path::builder();
    builder.add_rectangle(
        &Box2D {
            min: point(x, y),
            max: point(x + width, y + height),
        },
        Winding::Positive,
    );
    builder.build()
}

pub(super) fn circle(x: f32, y: f32, radius: f32) -> Path {
    let mut builder = Path::builder();
    builder.add_circle(point(x, y), radius, Winding::Positive);
    builder.build()
}

pub(super) fn rounded_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    corner: (f32, f32, f32, f32),
) -> Path {
    let (tl, tr, bl, br) = corner;

    let mut builder = Path::builder();
    builder.add_rounded_rectangle(
        &Box2D {
            min: point(x, y),
            max: point(x + width, y + height),
        },
        &BorderRadii {
            top_left: tl,
            top_right: tr,
            bottom_left: bl,
            bottom_right: br,
        },
        Winding::Positive,
    );
    builder.build()
}

pub fn ellipse(x: f32, y: f32, width: f32, height: f32, rotation: f32) -> Path {
    let mut builder = Path::builder();
    builder.add_ellipse(
        point(x, y),
        vector(width, height),
        Angle::radians(rotation),
        Winding::Positive,
    );
    builder.build()
}
