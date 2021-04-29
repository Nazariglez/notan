use glam::{Mat3, Vec2, Vec3};
use std::ops::{Deref, DerefMut};

pub trait DrawTransform {
    fn matrix(&mut self) -> &mut Option<Mat3>;

    fn transform(&mut self, matrix: Mat3) -> &mut Self {
        *self.matrix() = Some(matrix);
        self
    }

    fn translate(&mut self, x: f32, y: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::identity());
        let matrix = old * Mat3::from_translation(Vec2::new(x, y));
        *self.matrix() = Some(matrix);
        self
    }

    fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::identity());
        let matrix = old * Mat3::from_scale(Vec2::new(x, y));
        *self.matrix() = Some(matrix);
        self
    }

    fn rotate(&mut self, angle: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::identity());
        let matrix = old * Mat3::from_angle(angle);
        *self.matrix() = Some(matrix);
        self
    }

    #[inline]
    fn rotate_degree(&mut self, deg: f32) -> &mut Self {
        self.rotate(deg * notan_math::DEG_TO_RAD)
    }

    fn skew(&mut self, x: f32, y: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::identity());

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

    fn rotate_from(&mut self, point: (f32, f32), angle: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::identity());
        let translate = old * Mat3::from_translation(Vec2::new(point.0, point.1));
        let rotate = translate * Mat3::from_angle(angle);
        let matrix = rotate * Mat3::from_translation(Vec2::new(-point.0, -point.1));
        *self.matrix() = Some(matrix);
        self
    }

    fn rotate_degree_from(&mut self, point: (f32, f32), deg: f32) -> &mut Self {
        self.rotate_from(point, deg * notan_math::DEG_TO_RAD)
    }

    fn scale_from(&mut self, point: (f32, f32), scale: (f32, f32)) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::identity());
        let translate = old * Mat3::from_translation(Vec2::new(point.0, point.1));
        let scale = translate * Mat3::from_scale(Vec2::new(scale.0, scale.1));
        let matrix = scale * Mat3::from_translation(Vec2::new(-point.0, -point.1));
        *self.matrix() = Some(matrix);
        self
    }
}

pub struct Transform {
    identity: Mat3,
    stack: Vec<Mat3>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            identity: Mat3::identity(),
            stack: vec![],
        }
    }

    pub fn matrix(&self) -> &Mat3 {
        self.stack.last().unwrap_or(&self.identity)
    }

    pub fn matrix_mut(&mut self) -> &mut Mat3 {
        self.stack.last_mut().unwrap_or(&mut self.identity)
    }

    pub fn set(&mut self, matrix: Mat3) -> &mut Self {
        *self.matrix_mut() = matrix;
        self
    }

    pub fn push(&mut self, matrix: Mat3) -> &mut Self {
        self.stack.push(*self.matrix() * matrix);
        self
    }

    pub fn pop(&mut self) -> &mut Self {
        self.stack.pop();
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.identity = Mat3::identity();
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

/*
trait drawposition {
    fn center(x, y);
    fn top_left(x, y);
    etc...
}

enum position {
    center(x, y),
    top_left(x, y)
    etc...
}
 */
