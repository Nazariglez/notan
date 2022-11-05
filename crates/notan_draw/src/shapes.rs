mod circle;
mod ellipse;
mod geometry;
mod line;
mod painter;
mod path;
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
pub use polygon::Polygon;
pub use rect::Rectangle;
pub use star::Star;
pub use triangle::Triangle;

pub trait DrawShapes {
    fn line(&mut self, p1: (f32, f32), p2: (f32, f32)) -> DrawBuilder<Line>;
    fn triangle(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> DrawBuilder<Triangle>;
    fn path(&mut self) -> DrawBuilder<Path>;
    fn rect(&mut self, position: (f32, f32), size: (f32, f32)) -> DrawBuilder<Rectangle>;
    fn circle(&mut self, radius: f32) -> DrawBuilder<Circle>;
    fn ellipse(&mut self, position: (f32, f32), size: (f32, f32)) -> DrawBuilder<Ellipse>;
    fn star(&mut self, spikes: u8, outer_radius: f32, inner_radius: f32) -> DrawBuilder<Star>;
    fn polygon(&mut self, sides: u8, radius: f32) -> DrawBuilder<Polygon>;
}

impl DrawShapes for Draw {
    fn line(&mut self, p1: (f32, f32), p2: (f32, f32)) -> DrawBuilder<Line> {
        DrawBuilder::new(self, Line::new(p1, p2))
    }

    fn triangle(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> DrawBuilder<Triangle> {
        DrawBuilder::new(self, Triangle::new(a, b, c))
    }

    fn path(&mut self) -> DrawBuilder<Path> {
        DrawBuilder::new(self, Path::new())
    }

    fn rect(&mut self, position: (f32, f32), size: (f32, f32)) -> DrawBuilder<Rectangle> {
        DrawBuilder::new(self, Rectangle::new(position, size))
    }

    fn circle(&mut self, radius: f32) -> DrawBuilder<Circle> {
        DrawBuilder::new(self, Circle::new(radius))
    }

    fn ellipse(&mut self, position: (f32, f32), size: (f32, f32)) -> DrawBuilder<Ellipse> {
        DrawBuilder::new(self, Ellipse::new(position, size))
    }

    fn star(&mut self, spikes: u8, outer_radius: f32, inner_radius: f32) -> DrawBuilder<Star> {
        DrawBuilder::new(self, Star::new(spikes, outer_radius, inner_radius))
    }

    fn polygon(&mut self, sides: u8, radius: f32) -> DrawBuilder<Polygon> {
        DrawBuilder::new(self, Polygon::new(sides, radius))
    }
}
