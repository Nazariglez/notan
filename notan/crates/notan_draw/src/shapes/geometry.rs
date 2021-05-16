use lyon::math::*;
use lyon::path::builder::{BorderRadii, PathBuilder};
use lyon::path::Path;
use lyon::path::Winding;

// https://docs.rs/lyon_path/0.17.2/lyon_path/builder/trait.PathBuilder.html

pub(super) fn rectangle(x: f32, y: f32, width: f32, height: f32) -> Path {
    let mut builder = Path::builder();
    builder.add_rectangle(
        &Rect {
            origin: point(x, y),
            size: size(width, height),
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
        &Rect {
            origin: point(x, y),
            size: size(width, height),
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
