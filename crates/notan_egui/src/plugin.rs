use crate::input::{to_egui_key, to_egui_pointer};
use crate::EguiExtension;
use notan_app::assets::Assets;
use notan_app::{
    App, AppFlow, ClearOptions, Color, Commands, Device, Event, ExtContainer, GfxExtension,
    GfxRenderer, Plugin, RenderTexture,
};
use std::cell::RefCell;

#[derive(Default)]
pub struct EguiPlugin {
    ctx: egui::Context,
    raw_input: egui::RawInput,
}

impl EguiPlugin {
    #[inline]
    pub(crate) fn add_event(&mut self, evt: egui::Event) {
        self.raw_input.events.push(evt);
    }

    pub fn run(&mut self, run_ui: impl FnOnce(&egui::Context)) -> Output {
        let new_input = self.raw_input.take();

        let egui::FullOutput {
            platform_output,
            needs_repaint,
            textures_delta,
            shapes,
        } = self.ctx.run(new_input, run_ui);

        let egui::PlatformOutput {
            cursor_icon,
            open_url,
            copied_text,
            events,
            mutable_text_under_cursor,
            text_cursor_pos,
        } = platform_output;

        // TODO cursor, url, etc...

        Output {
            ctx: self.ctx.clone(),
            shapes: RefCell::new(Some(shapes)),
            textures_delta,
            clear_color: None,
        }
    }
}

pub struct Output {
    ctx: egui::Context,
    shapes: RefCell<Option<Vec<egui::epaint::ClippedShape>>>,
    textures_delta: egui::TexturesDelta,
    clear_color: Option<Color>,
}

impl Output {
    pub fn clear_color(&mut self, color: Color) {
        self.clear_color = Some(color);
    }
}

impl GfxExtension<Output> for EguiExtension {
    fn commands<'a>(&'a mut self, device: &mut Device, renderer: &'a Output) -> &'a [Commands] {
        &[]
    }
}

impl GfxRenderer for Output {
    fn render(
        &self,
        device: &mut Device,
        extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    ) {
        let mut ext = extensions.get_mut::<Self, EguiExtension>().unwrap();
        if let Some(shapes) = self.shapes.borrow_mut().take() {
            if self.clear_color.is_some() {
                let mut clear_renderer = device.create_renderer();
                clear_renderer.begin(Some(&ClearOptions {
                    color: self.clear_color,
                    ..Default::default()
                }));
                clear_renderer.end();

                match target {
                    Some(rt) => device.render_to(rt, clear_renderer.commands()),
                    _ => device.render(clear_renderer.commands()),
                }
            }

            let meshes = self.ctx.tessellate(shapes);
            if let Err(err) =
                ext.paint_and_update_textures(device, meshes, &self.textures_delta, target)
            {
                log::error!("{}", err);
            }
        }
    }
}

impl Plugin for EguiPlugin {
    fn event(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        event: &Event,
    ) -> Result<AppFlow, String> {
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
                    let factor = (delta_y / 200.0).exp();
                    self.add_event(egui::Event::Zoom(factor));
                } else if cfg!(target_os = "macos") && modifiers.shift {
                    self.add_event(egui::Event::Scroll(egui::vec2(delta_x + delta_y, 0.0)));
                } else {
                    self.add_event(egui::Event::Scroll(egui::vec2(*delta_x, *delta_y)));
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

    fn update(&mut self, app: &mut App, _assets: &mut Assets) -> Result<AppFlow, String> {
        self.raw_input.pixels_per_point = Some(app.window().dpi() as _);
        self.raw_input.time = Some(app.timer.time_since_init() as _);
        self.raw_input.predicted_dt = app.timer.delta_f32();

        let (w, h) = app.window().size();
        self.raw_input.screen_rect = Some(egui::Rect {
            min: egui::pos2(0.0, 0.0),
            max: egui::pos2(w as _, h as _),
        });
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
