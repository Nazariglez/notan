use crate::draw::Draw;
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
        if let Some(mut inner) = self.inner.take() {
            inner.draw_process(&mut self.draw);
        }
    }
}
