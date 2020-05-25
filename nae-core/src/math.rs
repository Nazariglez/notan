use lazy_static::lazy_static;
use rand::distributions::uniform::{SampleBorrow, SampleUniform};
use rand::distributions::{Distribution, Standard};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
pub use std::f32::consts::PI;
use std::ops::*;
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
