use crate::frame::NotanDrawFrame;
use crate::render_target::RenderTarget;
use crate::render_texture::NotanRenderTexture;
use crate::wgpu::surface::Surface;
use crate::NotanTexture;
use notan_core::window::WindowId;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

pub struct DrawFrame {
    pub(crate) window_id: WindowId,
    // pub(crate) surface: Surface,
    pub(crate) frame: SurfaceTexture,
    pub(crate) view: TextureView,
    pub(crate) encoder: RefCell<CommandEncoder>,
    pub(crate) dirty: RefCell<bool>,
    pub(crate) present_check: FramePresented,
}

impl NotanDrawFrame for DrawFrame {}

impl Debug for DrawFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DrawFrame {{ window_id: {:?} }}", self.window_id)
    }
}

#[derive(Default)]
pub(crate) struct FramePresented(bool);
impl FramePresented {
    pub fn validate(&mut self) {
        self.0 = true;
    }
}

impl Drop for FramePresented {
    fn drop(&mut self) {
        debug_assert!(self.0, "DrawFrame must be presented before drop it");
        log::error!("DrawFrame must be presented before drop it");
    }
}

impl<'a, RT> Into<RenderTarget<'a, DrawFrame, RT>> for &'a DrawFrame
where
    RT: NotanRenderTexture,
{
    fn into(self) -> RenderTarget<'a, DrawFrame, RT> {
        RenderTarget::Frame(self)
    }
}
