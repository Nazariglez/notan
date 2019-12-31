pub use glm::{
    identity, mat3, mat4, rotation2d, scaling2d, translation2d, vec2, vec3, Mat3, Mat4, Vec2, Vec3,
};
use nalgebra_glm as glm;
pub use std::f32::consts::PI;
use std::ops::*;
use std::sync::{Arc, Mutex};

//TODO evluate replace all the vecs and mats with the `vek` crate? (SIMD = performace) (nalgebra is needed to collision libs?)

pub fn eq_float(a: f32, b: f32) -> bool {
    //TODO improve this? https://floating-point-gui.de/errors/comparison/ it worth it? performance problems?
    (a - b).abs() < std::f32::EPSILON
}

pub fn projection_2d(width: i32, height: i32, flipped: bool, dpi: f32) -> Mat3 {
    let ww = width as f32 / dpi;
    let hh = height as f32 / dpi;
    let bottom = if flipped { 0.0 } else { height as f32 };
    let top = if flipped { height as f32 } else { 0.0 };
    let xx = -ww * 0.5 * dpi;
    let yy = -hh * 0.5 * dpi;
    glm::translate2d(
        &glm::mat4_to_mat3(&glm::ortho(0.0, width as f32, bottom, top, -1.0, 1.0)),
        &vec2(xx, yy),
    )
}
