pub use nalgebra_glm::{
    identity, mat3, mat4, rotation2d, scaling2d, translation2d, vec2, vec3, Mat3, Mat4, Vec2, Vec3,
};
pub use std::f32::consts::PI;
use std::ops::*;
use std::sync::{Arc, Mutex};

//TODO Replace all the vecs and mats with the `vek` crate? (SIMD = performace)

pub fn eq_float(a: f32, b: f32) -> bool {
    //TODO improve this? https://floating-point-gui.de/errors/comparison/ it worth it? performance problems?
    (a - b).abs() < std::f32::EPSILON
}
