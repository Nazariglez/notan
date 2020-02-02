pub use glm::{
    identity, mat3, mat4, rotation2d, scaling2d, translation2d, vec2, vec3, Mat3, Mat4, Vec2, Vec3,
};
use lazy_static::lazy_static;
pub use nalgebra_glm as glm;
use rand::distributions::uniform::{SampleBorrow, SampleUniform};
use rand::distributions::{Distribution, Standard};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
pub use std::f32::consts::PI;
use std::ops::*;
use std::sync::{Arc, Mutex};

//TODO evluate replace all the vecs and mats with the `ultraviolet` crate? (SIMD = performace) (nalgebra is needed to collision libs?)
pub fn eq_float(a: f32, b: f32) -> bool {
    //TODO improve this? https://floating-point-gui.de/errors/comparison/ it worth it? performance problems?
    (a - b).abs() < std::f32::EPSILON
}

pub fn projection_2d(width: i32, height: i32, flipped: bool, dpi: f32) -> Mat3 {
    let ww = width as f32 / dpi;
    let hh = height as f32 / dpi;
    let bottom = if flipped { 0.0 } else { hh };
    let top = if flipped { hh } else { 0.0 };
    let translate = vec2(-ww * 0.5, -hh * 0.5);
    let ortho = glm::mat4_to_mat3(&glm::ortho(0.0, ww, bottom, top, -1.0, 1.0));

    glm::translate2d(&ortho, &translate)
}
