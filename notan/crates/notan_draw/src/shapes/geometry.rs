use glam::Mat3;
use lyon::math::*;
use lyon::path::builder::{BorderRadii, PathBuilder};
use lyon::path::path::Builder;
use lyon::path::Path;
use lyon::path::Winding;
use lyon::tessellation::*;
use notan_graphics::color::Color;

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
    tl: f32,
    tr: f32,
    bl: f32,
    br: f32,
) -> Path {
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

pub fn ellipse() -> Path {
    todo!()
}
