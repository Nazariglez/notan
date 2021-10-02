use crate::context::EguiContext;
use crate::input::{to_egui_key, to_egui_pointer};
use notan_app::{App, AppFlow, Color, Event, Plugin};

#[derive(Default)]
pub struct EguiPlugin {
    ctx: egui::CtxRef,
    raw_input: egui::RawInput,
}

impl EguiPlugin {
    pub fn raw_ctx(&self) -> &egui::CtxRef {
        &self.ctx
    }

    pub fn create_context(&mut self, clear_color: Option<Color>) -> EguiContext {
        self.ctx.begin_frame(self.raw_input.take());
        EguiContext {
            ctx: self.ctx.clone(),
            clear_color,
        }
    }

    #[inline]
    pub(crate) fn add_event(&mut self, evt: egui::Event) {
        self.raw_input.events.push(evt);
    }
}

impl Plugin for EguiPlugin {
    fn event(&mut self, app: &mut App, event: &Event) -> Result<AppFlow, String> {
        let command_modifier = if cfg!(target_arch = "macos") {
            app.keyboard.logo()
        } else {
            app.keyboard.ctrl()
        };

        let modifiers = egui::Modifiers {
            alt: app.keyboard.alt(),
            ctrl: app.keyboard.ctrl(),
            shift: app.keyboard.shift(),
            mac_cmd: cfg!(target_os = "macos") && app.keyboard.logo(),
            command: command_modifier,
        };

        match event {
            Event::Exit => {}
            Event::WindowMove { .. } => {}
            Event::WindowResize { .. } => {}
            Event::ScreenAspectChange { .. } => {}
            Event::MouseMove { .. } => self.add_event(egui::Event::PointerMoved(egui::Pos2::new(
                app.mouse.x,
                app.mouse.y,
            ))),
            Event::MouseDown { button, .. } => {
                if let Some(btn) = to_egui_pointer(button) {
                    self.add_event(egui::Event::PointerButton {
                        pos: egui::Pos2::new(app.mouse.x, app.mouse.y),
                        button: btn,
                        pressed: true,
                        modifiers,
                    })
                }
            }
            Event::MouseUp { button, .. } => {
                if let Some(btn) = to_egui_pointer(button) {
                    self.add_event(egui::Event::PointerButton {
                        pos: egui::Pos2::new(app.mouse.x, app.mouse.y),
                        button: btn,
                        pressed: false,
                        modifiers,
                    })
                }
            }
            Event::MouseWheel { delta_x, delta_y } => {
                if modifiers.ctrl || modifiers.command {
                    self.raw_input.zoom_delta *= (delta_y / 200.0).exp();
                } else {
                    self.raw_input.scroll_delta += egui::vec2(*delta_x, *delta_y);
                }
            }
            Event::MouseEnter { .. } => {}
            Event::MouseLeft { .. } => self.add_event(egui::Event::PointerGone),
            Event::KeyDown { key } => {
                if let Some(key) = to_egui_key(key) {
                    self.add_event(egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                    })
                }
            }
            Event::KeyUp { key } => {
                if let Some(key) = to_egui_key(key) {
                    self.add_event(egui::Event::Key {
                        key,
                        pressed: false,
                        modifiers,
                    })
                }
            }
            Event::ReceivedCharacter(char) => {
                if is_printable(*char, &modifiers) {
                    self.add_event(egui::Event::Text(char.to_string()))
                }
            }
        }

        Ok(AppFlow::Next)
    }

    fn update(&mut self, app: &mut App) -> Result<AppFlow, String> {
        self.raw_input.pixels_per_point = Some(app.window().dpi() as _);
        self.raw_input.time = Some(app.timer.time_since_init() as _);
        Ok(AppFlow::Next)
    }
}

// impl code from here https://github.com/hasenbanck/egui_winit_platform/blob/master/src/lib.rs#L397
#[allow(clippy::manual_range_contains)]
fn is_printable(chr: char, modifiers: &egui::Modifiers) -> bool {
    if modifiers.ctrl || modifiers.mac_cmd {
        return false;
    }

    let is_in_private_use_area = '\u{e000}' <= chr && chr <= '\u{f8ff}'
        || '\u{f0000}' <= chr && chr <= '\u{ffffd}'
        || '\u{100000}' <= chr && chr <= '\u{10fffd}';

    !is_in_private_use_area && !chr.is_ascii_control()
}
