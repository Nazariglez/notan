use crate::draw::Draw;
use crate::{local_to_screen_position, screen_to_local_position, DrawTransform};
use notan_math::{vec2, Mat3, Vec2};
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
        let inverse = *self
            .draw
            .inverse_projection
            .get_or_insert(self.draw.projection().inverse());

        let world_matrix = *self.draw.transform().matrix();
        let local_matrix = self
            .inner
            .as_mut()
            .unwrap()
            .matrix()
            .unwrap_or(Mat3::IDENTITY);

        let view = world_matrix * local_matrix;

        screen_to_local_position(
            vec2(screen_x, screen_y),
            self.draw.size().into(),
            inverse,
            view,
        )
    }

    pub fn local_to_screen_position(&mut self, local_x: f32, local_y: f32) -> Vec2 {
        let world_matrix = *self.draw.transform().matrix();
        let local_matrix = self
            .inner
            .as_mut()
            .unwrap()
            .matrix()
            .unwrap_or(Mat3::IDENTITY);

        let view = world_matrix * local_matrix;

        local_to_screen_position(
            vec2(local_x, local_y),
            self.draw.size().into(),
            self.draw.projection(),
            view,
        )
    }
}
