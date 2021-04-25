mod circle;
mod geometry;
mod line;
mod painter;
mod path;
mod rect;
mod tess;
mod triangle;

pub use crate::builder::DrawBuilder;
pub use crate::draw2::Draw2;
pub(crate) use painter::ShapePainter;
pub use triangle::Triangle;

pub trait DrawShapes {
    fn triangle(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> DrawBuilder<Triangle>;
}

impl DrawShapes for Draw2 {
    fn triangle(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> DrawBuilder<Triangle> {
        DrawBuilder::new(self, Triangle::new(a, b, c))
    }
}
