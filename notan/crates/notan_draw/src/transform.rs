use glam::{Mat3, Vec2, Vec3};
use std::ops::Deref;

/// Helper methods to do matrix transformations
pub trait DrawTransform {
    /// Returns the object's matrix
    fn matrix(&mut self) -> &mut Option<Mat3>;

    /// Set the object's matrix
    fn transform(&mut self, matrix: Mat3) -> &mut Self {
        *self.matrix() = Some(matrix);
        self
    }

    /// Set the matrix position
    fn translate(&mut self, x: f32, y: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);
        let matrix = old * Mat3::from_translation(Vec2::new(x, y));
        *self.matrix() = Some(matrix);
        self
    }

    /// Set the matrix scale
    fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);
        let matrix = old * Mat3::from_scale(Vec2::new(x, y));
        *self.matrix() = Some(matrix);
        self
    }

    /// Set the matrix rotation using radians
    fn rotate(&mut self, angle: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);
        let matrix = old * Mat3::from_angle(angle);
        *self.matrix() = Some(matrix);
        self
    }

    #[inline]
    /// Set the matrix rotation using degrees
    fn rotate_degrees(&mut self, deg: f32) -> &mut Self {
        self.rotate(deg * notan_math::DEG_TO_RAD)
    }

    /// Set the matrix skew
    fn skew(&mut self, x: f32, y: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);

        let xt = x.tan();
        let yt = y.tan();

        let new = Mat3::from_cols(
            Vec3::new(1.0, xt, 0.0),
            Vec3::new(yt, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );

        *self.matrix() = Some(old * new);
        self
    }

    /// Set the matrix rotation using radians from the point given
    fn rotate_from(&mut self, point: (f32, f32), angle: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);
        let translate = old * Mat3::from_translation(Vec2::new(point.0, point.1));
        let rotate = translate * Mat3::from_angle(angle);
        let matrix = rotate * Mat3::from_translation(Vec2::new(-point.0, -point.1));
        *self.matrix() = Some(matrix);
        self
    }

    /// Set the matrix rotation using degrees from the point given
    fn rotate_degrees_from(&mut self, point: (f32, f32), deg: f32) -> &mut Self {
        self.rotate_from(point, deg * notan_math::DEG_TO_RAD)
    }

    /// Set the matrix scale from the point given
    fn scale_from(&mut self, point: (f32, f32), scale: (f32, f32)) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);
        let translate = old * Mat3::from_translation(Vec2::new(point.0, point.1));
        let scale = translate * Mat3::from_scale(Vec2::new(scale.0, scale.1));
        let matrix = scale * Mat3::from_translation(Vec2::new(-point.0, -point.1));
        *self.matrix() = Some(matrix);
        self
    }
}

#[derive(Default, Clone, Debug)]
/// This struct represents a stack of matrices
pub struct Transform {
    identity: Mat3,
    stack: Vec<Mat3>,
}

impl Transform {
    /// Create a new instance
    pub fn new() -> Self {
        Self {
            identity: Mat3::IDENTITY,
            stack: vec![],
        }
    }

    /// Returns a read reference of the matrix in use
    pub fn matrix(&self) -> &Mat3 {
        self.stack.last().unwrap_or(&self.identity)
    }

    /// Returns a mutable reference of the matrix in use
    pub fn matrix_mut(&mut self) -> &mut Mat3 {
        self.stack.last_mut().unwrap_or(&mut self.identity)
    }

    /// Set the matrix in use
    pub fn set(&mut self, matrix: Mat3) -> &mut Self {
        *self.matrix_mut() = matrix;
        self
    }

    /// Multiply the last matrix with the new one and adds it to the stack
    pub fn push(&mut self, matrix: Mat3) -> &mut Self {
        self.stack.push(*self.matrix() * matrix);
        self
    }

    /// Remove the last matrix from the stack
    pub fn pop(&mut self) -> &mut Self {
        self.stack.pop();
        self
    }

    /// Resets the base matrix to IDENTITY and remove the matrices on the stack
    pub fn reset(&mut self) -> &mut Self {
        self.identity = Mat3::IDENTITY;
        self.stack.clear();
        self
    }
}

impl Deref for Transform {
    type Target = Mat3;
    fn deref(&self) -> &Self::Target {
        self.matrix()
    }
}
