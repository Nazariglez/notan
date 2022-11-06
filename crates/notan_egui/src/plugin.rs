use crate::input::{to_egui_key, to_egui_pointer};
use crate::EguiExtension;
use egui::{Context, CursorIcon};
use notan_app::assets::Assets;
use notan_app::{
    App, AppFlow, ClearOptions, Color, CursorIcon as NCursorIcon, Device, Event, ExtContainer,
    GfxExtension, GfxRenderer, Graphics, Plugin, Plugins, RenderTexture,
};

use std::cell::RefCell;

#[cfg(feature = "links")]
use egui::output::OpenUrl;

pub struct EguiPlugin {
    ctx: egui::Context,
    raw_input: egui::RawInput,
    platform_output: Option<egui::PlatformOutput>,
    latest_evt_was_touch: bool,
    needs_repaint: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for EguiPlugin {
    fn default() -> Self {
        Self {
            ctx: Default::default(),
            raw_input: Default::default(),
            platform_output: Default::default(),
            latest_evt_was_touch: Default::default(),
            needs_repaint: Default::default(),
        }
    }
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
            repaint_after,
            textures_delta,
            shapes,
        } = self.ctx.run(new_input, run_ui);

        let needs_repaint = repaint_after.is_zero();

        // On post frame needs repaint is set to false
        // set it again if true after a egui output.
        if !self.needs_repaint {
            self.needs_repaint = needs_repaint;
        }

        self.platform_output = Some(platform_output);

        Output {
            ctx: self.ctx.clone(),
            shapes: RefCell::new(Some(shapes)),
            textures_delta,
            clear_color: None,
            needs_repaint,
        }
    }
}

pub struct Output {
    ctx: egui::Context,
    shapes: RefCell<Option<Vec<egui::epaint::ClippedShape>>>,
    textures_delta: egui::TexturesDelta,
    clear_color: Option<Color>,
    needs_repaint: bool,
}

impl Output {
    pub fn clear_color(&mut self, color: Color) {
        self.clear_color = Some(color);
    }

    pub fn needs_repaint(&self) -> bool {
        self.needs_repaint
    }
}

impl GfxExtension<Output> for EguiExtension {}

impl GfxRenderer for Output {
    fn render(
        &self,
        device: &mut Device,
        extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    ) -> Result<(), String> {
        let mut ext = extensions.get_mut::<Self, EguiExtension>().ok_or_else(|| {
            "Missing EguiExtension. You may need to add 'EguiConfig' to notan.".to_string()
        })?;

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
            ext.paint_and_update_textures(device, meshes, &self.textures_delta, target)?;
        }

        Ok(())
    }
}

impl Plugin for EguiPlugin {
    fn event(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        event: &Event,
    ) -> Result<AppFlow, String> {
        let mac_cmd = if cfg!(target_os = "macos") || cfg!(target_arch = "wasm32") {
            app.keyboard.logo()
        } else {
            false
        };

        let command_modifier = mac_cmd || app.keyboard.ctrl();

        let modifiers = egui::Modifiers {
            alt: app.keyboard.alt(),
            ctrl: app.keyboard.ctrl(),
            shift: app.keyboard.shift(),
            mac_cmd,
            command: command_modifier,
        };

        let mut is_touch_end = false;

        match event {
            Event::Exit => {}
            Event::WindowMove { .. } => {}
            Event::WindowResize { .. } => {
                self.ctx.request_repaint();
            }
            Event::ScreenAspectChange { .. } => {
                self.ctx.request_repaint();
            }
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
                    });

                    if self.latest_evt_was_touch {
                        self.add_event(egui::Event::PointerGone);
                    }
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

            Event::Copy => self.add_event(egui::Event::Copy),
            Event::Cut => self.add_event(egui::Event::Cut),
            Event::Paste(text) => self.add_event(egui::Event::Paste(text.into())),

            #[cfg(feature = "drop_files")]
            Event::DragEnter { path, mime, .. } => {
                self.raw_input.hovered_files.push(egui::HoveredFile {
                    path: path.clone(),
                    mime: mime.clone(),
                });
            }

            #[cfg(feature = "drop_files")]
            Event::DragLeft => {
                self.raw_input.hovered_files.clear();
            }

            #[cfg(feature = "drop_files")]
            Event::Drop(file) => {
                self.raw_input.hovered_files.clear();
                self.raw_input.dropped_files.push(egui::DroppedFile {
                    path: file.path.clone(),
                    ..Default::default()
                });
            }
            Event::TouchStart { id, x, y } => self.add_event(egui::Event::Touch {
                device_id: egui::TouchDeviceId(0),
                id: egui::TouchId(*id),
                phase: egui::TouchPhase::Start,
                pos: (*x, *y).into(),
                force: 0.0,
            }),
            Event::TouchMove { id, x, y } => self.add_event(egui::Event::Touch {
                device_id: egui::TouchDeviceId(0),
                id: egui::TouchId(*id),
                phase: egui::TouchPhase::Move,
                pos: (*x, *y).into(),
                force: 0.0,
            }),
            Event::TouchEnd { id, x, y } => {
                self.add_event(egui::Event::Touch {
                    device_id: egui::TouchDeviceId(0),
                    id: egui::TouchId(*id),
                    phase: egui::TouchPhase::End,
                    pos: (*x, *y).into(),
                    force: 0.0,
                });

                is_touch_end = true;
            }
            Event::TouchCancel { id, x, y } => {
                self.add_event(egui::Event::Touch {
                    device_id: egui::TouchDeviceId(0),
                    id: egui::TouchId(*id),
                    phase: egui::TouchPhase::Cancel,
                    pos: (*x, *y).into(),
                    force: 0.0,
                });
                is_touch_end = true;
            }
        }

        self.latest_evt_was_touch = is_touch_end;

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

    fn post_frame(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        _gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        if let Some(platform_output) = self.platform_output.take() {
            let egui::PlatformOutput {
                cursor_icon,
                open_url,

                copied_text,
                ..
            } = platform_output;

            {
                let win = app.window();
                win.set_cursor(translate_cursor(cursor_icon));
                if self.needs_repaint && win.lazy_loop() {
                    win.request_frame();
                }
            }

            #[cfg(not(feature = "links"))]
            let _ = open_url;

            #[cfg(feature = "links")]
            if let Some(OpenUrl { url, new_tab }) = open_url {
                if new_tab {
                    app.open_link_new_tab(&url);
                } else {
                    app.open_link(&url);
                }
            }

            if !copied_text.is_empty() {
                app.backend.set_clipboard_text(&copied_text);
            }
        }

        self.needs_repaint = false;
        Ok(AppFlow::Next)
    }
}

fn translate_cursor(cursor: CursorIcon) -> notan_app::CursorIcon {
    match cursor {
        CursorIcon::Default => NCursorIcon::Default,
        CursorIcon::None => NCursorIcon::None,
        CursorIcon::ContextMenu => NCursorIcon::ContextMenu,
        CursorIcon::Help => NCursorIcon::Help,
        CursorIcon::PointingHand => NCursorIcon::PointingHand,
        CursorIcon::Progress => NCursorIcon::Progress,
        CursorIcon::Wait => NCursorIcon::Wait,
        CursorIcon::Cell => NCursorIcon::Cell,
        CursorIcon::Crosshair => NCursorIcon::Crosshair,
        CursorIcon::Text => NCursorIcon::Text,
        CursorIcon::VerticalText => NCursorIcon::VerticalText,
        CursorIcon::Alias => NCursorIcon::Alias,
        CursorIcon::Copy => NCursorIcon::Copy,
        CursorIcon::Move => NCursorIcon::Move,
        CursorIcon::NoDrop => NCursorIcon::NoDrop,
        CursorIcon::NotAllowed => NCursorIcon::NotAllowed,
        CursorIcon::Grab => NCursorIcon::Grab,
        CursorIcon::Grabbing => NCursorIcon::Grabbing,
        CursorIcon::AllScroll => NCursorIcon::AllScroll,
        CursorIcon::ResizeHorizontal => NCursorIcon::ResizeHorizontal,
        CursorIcon::ResizeNeSw => NCursorIcon::ResizeNeSw,
        CursorIcon::ResizeNwSe => NCursorIcon::ResizeNwSe,
        CursorIcon::ResizeVertical => NCursorIcon::ResizeVertical,
        CursorIcon::ZoomIn => NCursorIcon::ZoomIn,
        CursorIcon::ZoomOut => NCursorIcon::ZoomOut,
        CursorIcon::ResizeEast => NCursorIcon::ResizeEast,
        CursorIcon::ResizeSouthEast => NCursorIcon::ResizeSouthEast,
        CursorIcon::ResizeSouth => NCursorIcon::ResizeSouth,
        CursorIcon::ResizeSouthWest => NCursorIcon::ResizeSouthWest,
        CursorIcon::ResizeWest => NCursorIcon::ResizeWest,
        CursorIcon::ResizeNorthWest => NCursorIcon::ResizeNorthWest,
        CursorIcon::ResizeNorth => NCursorIcon::ResizeNorth,
        CursorIcon::ResizeNorthEast => NCursorIcon::ResizeNorthEast,
        CursorIcon::ResizeColumn => NCursorIcon::ResizeColumn,
        CursorIcon::ResizeRow => NCursorIcon::ResizeRow,
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

pub trait EguiPluginSugar {
    fn egui(&mut self, run_ui: impl FnOnce(&egui::Context)) -> Output;
}

impl EguiPluginSugar for Plugins {
    fn egui(&mut self, run_ui: impl FnOnce(&Context)) -> Output {
        let mut ext = self.get_mut::<EguiPlugin>().unwrap();
        ext.run(run_ui)
    }
}
