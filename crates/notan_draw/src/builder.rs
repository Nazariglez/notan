use crate::draw::Draw;
use crate::DrawTransform;
use notan_math::{vec2, vec3, vec4, Mat3, Vec2};
use std::ops::{Deref, DerefMut};

pub trait DrawProcess {
    fn draw_process(self, draw: &mut Draw);
    //TODO add 'extract' method to cache the vertices and indices?
    // fn extract<T>(self) -> T; where T is ie: impl Into<ShapeInfo>
}

pub struct DrawBuilder<'a, T>
where
    T: DrawProcess,
{
    inner: Option<T>,
    draw: &'a mut Draw,
}

impl<'a, T> DrawBuilder<'a, T>
where
    T: DrawProcess,
{
    pub fn new(draw: &'a mut Draw, item: T) -> Self {
        Self {
            inner: Some(item),
            draw,
        }
    }
}

impl<T> Deref for DrawBuilder<'_, T>
where
    T: DrawProcess,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<T> DerefMut for DrawBuilder<'_, T>
where
    T: DrawProcess,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

impl<T> Drop for DrawBuilder<'_, T>
where
    T: DrawProcess,
{
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            inner.draw_process(self.draw);
        }
    }
}

impl<'a, T> DrawBuilder<'a, T>
where
    T: DrawProcess + DrawTransform,
{
    pub fn screen_to_local_position(&mut self, screen_x: f32, screen_y: f32) -> Vec2 {
        let (width, height) = self.draw.size();

        // normalized coordinates
        let mx = (screen_x / width) * 2.0 - 1.0;
        let my = -(screen_y / height) * 2.0 + 1.0;

        let inverse = self
            .draw
            .inverse_projection
            .get_or_insert(self.draw.projection().inverse());

        let pos = inverse.project_point3(vec3(mx, my, 1.0));

        let world_matrix = *self.draw.transform().matrix();
        let local_matrix = self
            .inner
            .as_mut()
            .unwrap()
            .matrix()
            .unwrap_or(Mat3::IDENTITY);

        let inverse = (world_matrix * local_matrix).inverse();
        inverse.transform_point2(vec2(pos.x, pos.y))
    }

    pub fn local_to_screen_position(&mut self, local_x: f32, local_y: f32) -> Vec2 {
        let (width, height) = self.draw.size();
        let half_width = width * 0.5;
        let half_height = height * 0.5;

        let world_matrix = *self.draw.transform().matrix();
        let local_matrix = self
            .inner
            .as_mut()
            .unwrap()
            .matrix()
            .unwrap_or(Mat3::IDENTITY);

        let pos = (world_matrix * local_matrix) * vec3(local_x, local_y, 1.0);
        let pos = self.draw.projection() * vec4(pos.x, pos.y, pos.z, 1.0);
        vec2(
            half_width + (half_width * pos.x),
            half_height + (half_height * -pos.y),
        )
    }
}
