use crate::EguiExtension;
use notan_app::{Device, ExtContainer, GfxRenderer, Plugin, RenderTexture};
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct EguiPlugin {
    ctx: egui::CtxRef,
    raw_input: egui::RawInput,
}

impl EguiPlugin {
    pub fn ctx(&self) -> &egui::CtxRef {
        &self.ctx
    }

    pub fn begin_frame(&mut self) {
        self.ctx.begin_frame(self.raw_input.take());
    }

    pub fn end_frame(&self) -> EguiRenderer {
        let ctx = self.ctx.clone();
        let (output, shapes) = ctx.end_frame();
        EguiRenderer {
            ctx,
            output,
            shapes,
        }
    }
}

impl Deref for EguiPlugin {
    type Target = egui::CtxRef;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for EguiPlugin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

impl Plugin for EguiPlugin {}

pub struct EguiRenderer {
    pub(crate) ctx: egui::CtxRef,
    pub(crate) shapes: Vec<egui::paint::ClippedShape>,
    pub(crate) output: egui::Output,
}
