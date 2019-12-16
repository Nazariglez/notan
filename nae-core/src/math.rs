pub use nalgebra_glm::{identity, mat3, mat4, vec2, vec3, Mat3, Mat4, Vec2, Vec3};
pub use std::f32::consts::PI;
use std::ops::*;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
pub use super::rand::{Random, RngType, RngRandomType};

lazy_static! {
    static ref RNG:Arc<Mutex<Random>> = Arc::new(Mutex::new(Random::new(crate::date_now())));
}

//TODO Replace all the vecs and mats with the `vek` crate? (SIMD = performace)

pub fn eq_float(a: f32, b: f32) -> bool {
    //TODO improve this? https://floating-point-gui.de/errors/comparison/ it worth it? performance problems?
    (a - b).abs() < std::f32::EPSILON
}

pub fn random<T: RngType>() -> T {
    RNG.lock().unwrap().next()
}

pub fn random_range<T>(min: T, max: T) -> T
where
    T: RngRandomType,
{
    RNG.lock().unwrap().next_range(min, max)
}

pub fn random_seed(seed: u64) {
    RNG.lock().unwrap().reseed(seed);
}

//pub trait RngType {
//    fn random() -> Self;
//    //    fn random_seed(seed: i32) -> Self { unimplemented!() }
//}
//
//impl RngType for bool {
//    #[cfg(target_arch = "wasm32")]
//    fn random() -> Self {
//        js_sys::Math::random() < 0.5
//    }
//}
//
//impl RngType for f64 {
//    #[cfg(target_arch = "wasm32")]
//    fn random() -> Self {
//        js_sys::Math::random()
//    }
//}
//
//impl RngType for f32 {
//    #[cfg(target_arch = "wasm32")]
//    fn random() -> Self {
//        js_sys::Math::random() as f32
//    }
//}
//
//impl RngType for i32 {
//    #[cfg(target_arch = "wasm32")]
//    fn random() -> Self {
//        std::i32::MIN + (js_sys::Math::random() * std::i32::MAX as f64) as i32
//    }
//}
//
//impl RngType for i64 {
//    #[cfg(target_arch = "wasm32")]
//    fn random() -> Self {
//        std::i64::MIN + (js_sys::Math::random() * std::i64::MAX as f64) as i64
//    }
//}
//
//impl RngType for u32 {
//    #[cfg(target_arch = "wasm32")]
//    fn random() -> Self {
//        js_sys::Math::random() as u32
//    }
//}
//
//impl RngType for u64 {
//    #[cfg(target_arch = "wasm32")]
//    fn random() -> Self {
//        js_sys::Math::random() as u64
//    }
//}
//
//pub fn random<T: RngType>() -> T {
//    T::random()
//}
//
//pub fn random_range<T>(min: T, max: T) -> T
//where
//    T: RngType + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy,
//{
//    min + T::random() * (max - min)
//}
