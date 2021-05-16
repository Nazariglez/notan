mod circle;
mod ellipse;
mod geometry;
mod line;
mod painter;
mod path;
mod rect;
mod tess;
mod triangle;

pub use crate::builder::DrawBuilder;
pub use crate::draw::Draw;
pub use circle::Circle;
pub use ellipse::Ellipse;
pub use line::Line;
pub(crate) use painter::*;
pub use path::Path;
pub use rect::Rectangle;
pub use triangle::Triangle;

pub trait DrawShapes {
    fn line(&mut self, p1: (f32, f32), p2: (f32, f32)) -> DrawBuilder<Line>;
    fn triangle(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> DrawBuilder<Triangle>;
    fn path(&mut self) -> DrawBuilder<Path>;
    fn rect(&mut self, position: (f32, f32), size: (f32, f32)) -> DrawBuilder<Rectangle>;
    fn circle(&mut self, radius: f32) -> DrawBuilder<Circle>;
    fn ellipse(&mut self, position: (f32, f32), size: (f32, f32)) -> DrawBuilder<Ellipse>;
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
}
