mod circle;
mod ellipse;
mod geometry;
mod line;
mod painter;
mod path;
mod point;
mod polygon;
mod rect;
mod star;
mod tess;
mod triangle;

pub use crate::builder::DrawBuilder;
pub use crate::draw::Draw;
pub use circle::Circle;
pub use ellipse::Ellipse;
pub use line::Line;
pub use painter::create_shape_pipeline;
pub(crate) use painter::*;
pub use path::Path;
pub use point::{Point, XAlignment, YAlignment};
pub use polygon::Polygon;
pub use rect::Rectangle;
pub use star::Star;
pub use triangle::Triangle;

#[allow(mismatched_lifetime_syntaxes)]
pub trait DrawShapes {
    fn point(&mut self, x: f32, y: f32) -> DrawBuilder<'_, Point>;
    fn line(&mut self, p1: (f32, f32), p2: (f32, f32)) -> DrawBuilder<'_, Line>;
    fn triangle(
        &mut self,
        a: (f32, f32),
        b: (f32, f32),
        c: (f32, f32),
    ) -> DrawBuilder<'_, Triangle>;
    fn path(&mut self) -> DrawBuilder<'_, Path>;
    fn rect(&mut self, position: (f32, f32), size: (f32, f32)) -> DrawBuilder<'_, Rectangle>;
    fn circle(&mut self, radius: f32) -> DrawBuilder<'_, Circle>;
    fn ellipse(&mut self, position: (f32, f32), size: (f32, f32)) -> DrawBuilder<'_, Ellipse>;
    fn star(&mut self, spikes: u8, outer_radius: f32, inner_radius: f32) -> DrawBuilder<'_, Star>;
    fn polygon(&mut self, sides: u8, radius: f32) -> DrawBuilder<'_, Polygon>;
}

#[allow(mismatched_lifetime_syntaxes)]
impl DrawShapes for Draw {
    fn point(&mut self, x: f32, y: f32) -> DrawBuilder<'_, Point> {
        DrawBuilder::new(self, Point::new(x, y))
    }

    fn line(&mut self, p1: (f32, f32), p2: (f32, f32)) -> DrawBuilder<'_, Line> {
        DrawBuilder::new(self, Line::new(p1, p2))
    }

    fn triangle(
        &mut self,
        a: (f32, f32),
        b: (f32, f32),
        c: (f32, f32),
    ) -> DrawBuilder<'_, Triangle> {
        DrawBuilder::new(self, Triangle::new(a, b, c))
    }

    fn path(&mut self) -> DrawBuilder<'_, Path> {
        DrawBuilder::new(self, Path::new())
    }

    fn rect(&mut self, position: (f32, f32), size: (f32, f32)) -> DrawBuilder<'_, Rectangle> {
        DrawBuilder::new(self, Rectangle::new(position, size))
    }

    fn circle(&mut self, radius: f32) -> DrawBuilder<'_, Circle> {
        DrawBuilder::new(self, Circle::new(radius))
    }

    fn ellipse(&mut self, position: (f32, f32), size: (f32, f32)) -> DrawBuilder<'_, Ellipse> {
        DrawBuilder::new(self, Ellipse::new(position, size))
    }

    fn star(&mut self, spikes: u8, outer_radius: f32, inner_radius: f32) -> DrawBuilder<'_, Star> {
        DrawBuilder::new(self, Star::new(spikes, outer_radius, inner_radius))
    }

    fn polygon(&mut self, sides: u8, radius: f32) -> DrawBuilder<'_, Polygon> {
        DrawBuilder::new(self, Polygon::new(sides, radius))
    }
}
