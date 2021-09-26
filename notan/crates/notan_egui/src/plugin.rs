use crate::EguiExtension;
use notan_app::{
    App, AppBuilder, AppFlow, Device, Event, ExtContainer, GfxRenderer, Plugin, RenderTexture,
};
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

    #[inline]
    fn add_event(&mut self, evt: egui::Event) {
        self.raw_input.events.push(evt);
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

impl Plugin for EguiPlugin {
    fn event(&mut self, app: &mut App, event: &Event) -> Result<AppFlow, String> {
        match event {
            Event::Exit => {}
            Event::WindowMove { .. } => {}
            Event::WindowResize { .. } => {}
            Event::ScreenAspectChange { .. } => {}
            Event::MouseMove { x, y } => {
                self.add_event(egui::Event::PointerMoved(egui::Pos2::new(*x as _, *y as _)))
            }
            Event::MouseDown { button, x, y } => self.add_event(egui::Event::PointerButton {
                pos: egui::Pos2::new(app.mouse.x, app.mouse.y),
                button: egui::PointerButton::Primary, // TODO
                pressed: true,
                modifiers: Default::default(), // TODO fill this event
            }),
            Event::MouseUp { button, x, y } => self.add_event(egui::Event::PointerButton {
                pos: egui::Pos2::new(app.mouse.x, app.mouse.y),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: Default::default(),
            }),
            Event::MouseWheel { .. } => {}
            Event::MouseEnter { .. } => {}
            Event::MouseLeft { .. } => {}
            Event::KeyDown { .. } => {}
            Event::KeyUp { .. } => {}
            Event::ReceivedCharacter(char) => self.add_event(egui::Event::Text(char.to_string())),
        }
        Ok(AppFlow::Next)
    }

    fn update(&mut self, app: &mut App) -> Result<AppFlow, String> {
        self.raw_input.pixels_per_point = Some(app.window().dpi() as _);
        self.raw_input.time = Some(app.timer.time_since_init() as _);
        Ok(AppFlow::Next)
    }
}

pub struct EguiRenderer {
    pub(crate) ctx: egui::CtxRef,
    pub(crate) shapes: Vec<egui::paint::ClippedShape>,
    pub(crate) output: egui::Output,
}
