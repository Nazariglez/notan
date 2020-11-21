use crate::color::Color;
use crate::commands::*;
use crate::renderer::Renderer;
use glam::Mat4;

pub struct Draw<'a> {
    renderer: Renderer<'a>,
    transform: Mat4,
}

impl<'a> Draw<'a> {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            renderer: Renderer::new(width, height),
            transform: Mat4::identity(),
        }
    }

    pub fn begin(&mut self, color: Color) {}

    pub fn end(&mut self) {}

    pub fn push(&mut self, transform: Mat4) {}

    pub fn pop(&mut self) {}

    pub fn transform(&self) -> &Mat4 {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Mat4 {
        &mut self.transform
    }

    pub fn push_scale(&mut self, x: f32, y: f32) {}

    pub fn push_translation(&mut self, x: f32, y: f32) {}

    pub fn push_rotation(&mut self, angle: f32) {}

    pub fn push_skew(&mut self, x: f32, y: f32) {}

    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width: f32) {}

    pub fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {}
}

impl<'a> ToCommandBuffer<'a> for Draw<'a> {
    fn commands(&'a self) -> &'a [Commands<'a>] {
        self.renderer.commands()
    }
}
