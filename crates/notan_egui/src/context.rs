use notan_app::Color;
use std::ops::{Deref, DerefMut};

pub struct EguiContext {
    pub(crate) ctx: egui::CtxRef,
    pub(crate) clear_color: Option<Color>,
}

impl Deref for EguiContext {
    type Target = egui::CtxRef;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for EguiContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}
