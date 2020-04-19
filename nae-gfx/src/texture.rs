use nae_core::math::Rect;
use std::cell::RefCell;
use std::rc::Rc;

struct InnerTexture {
    width: i32,
    height: i32,
}

impl InnerTexture {
    fn frame(&self) -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            width: self.width as f32,
            height: self.height as f32,
        }
    }
}

/// Represents a texture loaded in memory
#[derive(Clone)]
pub struct Texture {
    inner: Rc<RefCell<InnerTexture>>,
    frame: Option<Rect>,
}

impl Texture {
    /// Returns the current frame
    pub fn frame(&self) -> Rect {
        self.frame.clone().unwrap_or(self.inner.borrow().frame())
    }

    /// Returns a new texture sharing the texture but with a new frame
    pub fn with_frame(&self, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            inner: self.inner.clone(),
            frame: Some(Rect {
                x,
                y,
                width,
                height,
            }),
        }
    }

    /// Width of the base texture without the current frame's size
    pub fn base_width(&self) -> f32 {
        self.inner.borrow().width as _
    }

    /// Height of the base texture without the current frame's size
    pub fn base_height(&self) -> f32 {
        self.inner.borrow().height as _
    }
}
