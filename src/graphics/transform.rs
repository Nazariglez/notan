use crate::math::*;

/// Transformation matrix stack
pub struct Transform2d(Vec<Mat3>);

impl Transform2d {
    /// Create a new stack with an identity matrix on it
    pub fn new() -> Self {
        Self(vec![Mat3::identity()])
    }

    /// Push a new matrix to the stack
    pub fn push(&mut self, matrix: Mat3) {
        self.0.push(self.matrix() * matrix);
    }

    /// Remove the last matrix on the stack
    pub fn pop(&mut self) {
        if self.0.len() <= 1 {
            return;
        }
        self.0.pop();
    }

    /// Return the current matrix
    pub fn matrix(&self) -> &Mat3 {
        &self.0[self.0.len() - 1]
    }

    /// Returns a mutable reference for the current matrix, if there is no matrix on the stack a new one will be created
    pub fn matrix_mut(&mut self) -> &mut Mat3 {
        let len = self.0.len();
        if len > 1 {
            return &mut self.0[len - 1];
        }

        self.0.push(identity());
        &mut self.0[len]
    }

    /// Create a new matrix with the position passed and push it to the stack
    pub fn translate(&mut self, x: f32, y: f32) {
        self.push(glm::translation2d(&vec2(x, y)));
    }

    /// Create a new matrix with the scale passed and push it to the stack
    pub fn scale(&mut self, x: f32, y: f32) {
        self.push(glm::scaling2d(&vec2(x, y)));
    }

    /// Create a new matrix with the rotation passed and push it to the stack
    pub fn rotate(&mut self, angle: f32) {
        self.push(glm::rotation2d(angle));
    }

    /// Create a new matrix with the skew passed and push it to the stack
    pub fn skew(&mut self, x: f32, y: f32) {
        let m = mat3(1.0, x.tan(), 0.0, y.tan(), 1.0, 0.0, 0.0, 0.0, 1.0);

        self.push(m);
    }

    /// Same as `skew` but using degrees instead of radians
    pub fn skew_deg(&mut self, x: f32, y: f32) {
        let x = x * (PI / 180.0);
        let y = y * (PI / 180.0);
        self.skew(x, y);
    }

    /// Same as `rotate` but using degrees instead of radians
    pub fn rotate_deg(&mut self, angle: f32) {
        self.rotate(PI / 180.0 * angle);
    }
}
