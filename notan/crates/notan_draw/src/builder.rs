use crate::draw2::Draw2;
use std::ops::{Deref, DerefMut};

pub trait DrawProcess {
    fn draw_process(self, draw: &mut Draw2);
}

pub struct DrawBuilder<'a, T>
where
    T: DrawProcess,
{
    inner: Option<T>,
    draw: &'a mut Draw2,
}

impl<'a, T> DrawBuilder<'a, T>
where
    T: DrawProcess,
{
    pub fn new(draw: &'a mut Draw2, item: T) -> Self {
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
        let mut inner = self.inner.take().unwrap();
        inner.draw_process(&mut self.draw);
    }
}
