use crate::common::ToNaeValue;
use glutin::{dpi::LogicalSize, ContextBuilder, PossiblyCurrent, WindowedContext};
use nae_core::window::BaseWindow;
use nae_core::{BaseApp, BaseSystem, BuilderOpts, Event, EventIterator, KeyCode, MouseButton};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};
use winit::event::MouseButton as WinitMB;
use winit::event::{ElementState, Event as WinitEvent, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::monitor::MonitorHandle;
use winit::window::Fullscreen::Borderless;
use winit::window::WindowBuilder;

pub struct System {
    window: Window,
    draw: nae_gfx::Draw,
    events: EventIterator,
    event_loop: Option<EventLoop<()>>,
}

impl BaseSystem for System {
    type Kind = Self;
    type Graphics = nae_gfx::Graphics;
    type Draw = nae_gfx::Draw;

    fn new(mut opts: BuilderOpts) -> Result<Self, String> {
        let event_loop = EventLoop::new();
        let win = Window::new(&opts, &event_loop)?;
        let draw = nae_gfx::Draw::new(&win.win)?;
        Ok(Self {
            window: win,
            event_loop: Some(event_loop),
            events: EventIterator::new(),
            draw,
        })
    }

    fn gfx(&mut self) -> &mut Self::Graphics {
        &mut self.draw.gfx
    }

    fn draw(&mut self) -> &mut Self::Draw {
        &mut self.draw
    }

    fn events(&mut self) -> &mut EventIterator {
        &mut self.events
    }

    fn width(&self) -> f32 {
        self.window.width() as _
    }

    fn height(&self) -> f32 {
        self.window.height() as _
    }

    fn dpi(&self) -> f32 {
        self.window.dpi()
    }

    fn set_fullscreen(&mut self, full: bool) {
        self.window.set_fullscreen(full);
    }

    fn fullscreen(&self) -> bool {
        self.window.fullscreen()
    }
}

pub struct Window {
    win: WindowedContext<PossiblyCurrent>,
    title: String,
    fullscreen: bool,
    dpi: f32,
}

impl Window {
    fn new(opts: &BuilderOpts, event_loop: &EventLoop<()>) -> Result<Self, String> {
        let mut win_builder = WindowBuilder::new()
            .with_title(&opts.title)
            .with_resizable(opts.resizable)
            .with_maximized(opts.maximized)
            .with_inner_size(LogicalSize::new(opts.width as f64, opts.height as f64));

        if let Some((w, h)) = opts.max_size {
            win_builder = win_builder.with_max_inner_size(LogicalSize::new(w, h));
        }

        if let Some((w, h)) = opts.min_size {
            win_builder = win_builder.with_min_inner_size(LogicalSize::new(w, h));
        }

        let win_ctx = ContextBuilder::new()
            .with_vsync(true)
            .with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 3),
                opengles_version: (2, 0),
            })
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(8)
            .build_windowed(win_builder, event_loop)
            .map_err(|e| format!("{}", e))?;

        let win = unsafe { win_ctx.make_current().unwrap() };
        let dpi = win.window().scale_factor() as f32;

        let mut fullscreen = false;
        if opts.fullscreen {
            let monitor = win.window().current_monitor();
            win.window().set_fullscreen(Some(Borderless(monitor)));
            fullscreen = true;
        }

        Ok(Self {
            title: opts.title.to_string(),
            fullscreen,
            win,
            dpi,
        })
    }

    fn set_fullscreen(&mut self, full: bool) {
        self.fullscreen = full;
        if full {
            let monitor = self.win.window().current_monitor();
            self.win.window().set_fullscreen(Some(Borderless(monitor)));
        } else {
            self.win.window().set_fullscreen(None);
        }
    }
}

impl BaseWindow for Window {
    fn width(&self) -> i32 {
        (self.win.window().inner_size().width as f32 / self.dpi) as _
    }

    fn height(&self) -> i32 {
        (self.win.window().inner_size().height as f32 / self.dpi) as _
    }

    fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn dpi(&self) -> f32 {
        self.dpi
    }
}

pub fn run<A, S, F, D>(mut app: A, mut state: S, mut update: F, mut draw: D) -> Result<(), String>
where
    A: BaseApp<System = System> + 'static,
    S: 'static,
    F: FnMut(&mut A, &mut S) + 'static,
    D: FnMut(&mut A, &mut S) + 'static,
{
    let mut event_loop = app.system().event_loop.take().unwrap();
    let mut running = true;
    let (mut last_mouse_x, mut last_mouse_y) = (0, 0);

    event_loop.run(move |event, target, mut control| {
        if !running {
            return;
        }
        match event {
            WinitEvent::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(size) => {
                    app.system().window.win.resize(*size);
                    let dpi = app.system().window.dpi;
                    let ww = (size.width as f32 / dpi) as _;
                    let hh = (size.height as f32 / dpi) as _;

                    app.system().events.push(Event::WindowResize {
                        width: ww,
                        height: hh,
                    });
                }
                WindowEvent::CloseRequested => {
                    running = false;
                    *control = ControlFlow::Exit;
                    return;
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => {
                    app.system().events.push(Event::ScreenAspectChange {
                        ratio: *scale_factor as _,
                    });
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    let evt = match state {
                        ElementState::Pressed => Event::MouseDown {
                            button: button.to_nae(),
                            x: last_mouse_x,
                            y: last_mouse_y,
                        },
                        _ => Event::MouseUp {
                            button: button.to_nae(),
                            x: last_mouse_x,
                            y: last_mouse_y,
                        },
                    };
                    app.system().events.push(evt);
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let evt = match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, y) => Event::MouseWheel {
                            delta_x: *x,
                            delta_y: *y,
                        },
                        winit::event::MouseScrollDelta::PixelDelta(
                            winit::dpi::LogicalPosition { x, y },
                        ) => {
                            let delta_x = if *x > 0.0 {
                                (*x / 10.0).max(0.1)
                            } else {
                                (*x / 10.0).min(-0.1)
                            } as f32;

                            let delta_y = if *y > 0.0 {
                                (*y / 10.0).max(0.1)
                            } else {
                                (*y / 10.0).min(-0.1)
                            } as f32;
                            Event::MouseWheel { delta_x, delta_y }
                        }
                    };
                    app.system().events.push(evt);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    last_mouse_x = position.x;
                    last_mouse_y = position.y;
                    app.system().events.push(Event::MouseMove {
                        x: last_mouse_x,
                        y: last_mouse_y,
                    });
                }
                WindowEvent::CursorEntered { .. } => {
                    app.system().events.push(Event::MouseEnter {
                        x: last_mouse_x,
                        y: last_mouse_y,
                    });
                }
                WindowEvent::CursorLeft { .. } => {
                    app.system().events.push(Event::MouseLeft {
                        x: last_mouse_x,
                        y: last_mouse_y,
                    });
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    let key = input.virtual_keycode.to_nae();
                    let evt = match input.state {
                        ElementState::Pressed => Event::KeyDown { key },
                        _ => Event::KeyUp { key },
                    };
                    app.system().events.push(evt);
                }
                WindowEvent::ReceivedCharacter(c) => {
                    app.system().events.push(Event::ReceivedCharacter(*c));
                }
                _ => {}
            },
            WinitEvent::MainEventsCleared => {
                update(&mut app, &mut state);
                app.system().window.win.window().request_redraw();
            }
            WinitEvent::RedrawRequested(_) => {
                draw(&mut app, &mut state);
                app.system().window.win.swap_buffers();
            }
            _ => {}
        }

        let mut time = Instant::now();
        time = time + Duration::from_secs_f32(1.0 / 60.0);
        *control = ControlFlow::WaitUntil(time);
        //            *control = ControlFlow::Poll;
    });

    Ok(())
}

impl ToNaeValue for WinitMB {
    type Kind = MouseButton;

    fn to_nae(&self) -> Self::Kind {
        match &self {
            WinitMB::Left => MouseButton::Left,
            WinitMB::Middle => MouseButton::Middle,
            WinitMB::Right => MouseButton::Right,
            WinitMB::Other(n) => MouseButton::Other(*n),
        }
    }
}

impl ToNaeValue for Option<VirtualKeyCode> {
    type Kind = KeyCode;

    fn to_nae(&self) -> Self::Kind {
        match self {
            Some(k) => match k {
                VirtualKeyCode::Key1 => KeyCode::Key1,
                VirtualKeyCode::Key2 => KeyCode::Key2,
                VirtualKeyCode::Key3 => KeyCode::Key3,
                VirtualKeyCode::Key4 => KeyCode::Key4,
                VirtualKeyCode::Key5 => KeyCode::Key5,
                VirtualKeyCode::Key6 => KeyCode::Key6,
                VirtualKeyCode::Key7 => KeyCode::Key7,
                VirtualKeyCode::Key8 => KeyCode::Key8,
                VirtualKeyCode::Key9 => KeyCode::Key9,
                VirtualKeyCode::Key0 => KeyCode::Key0,
                VirtualKeyCode::A => KeyCode::A,
                VirtualKeyCode::B => KeyCode::B,
                VirtualKeyCode::C => KeyCode::C,
                VirtualKeyCode::D => KeyCode::D,
                VirtualKeyCode::E => KeyCode::E,
                VirtualKeyCode::F => KeyCode::F,
                VirtualKeyCode::G => KeyCode::G,
                VirtualKeyCode::H => KeyCode::H,
                VirtualKeyCode::I => KeyCode::I,
                VirtualKeyCode::J => KeyCode::J,
                VirtualKeyCode::K => KeyCode::K,
                VirtualKeyCode::L => KeyCode::L,
                VirtualKeyCode::M => KeyCode::M,
                VirtualKeyCode::N => KeyCode::N,
                VirtualKeyCode::O => KeyCode::O,
                VirtualKeyCode::P => KeyCode::P,
                VirtualKeyCode::Q => KeyCode::Q,
                VirtualKeyCode::R => KeyCode::R,
                VirtualKeyCode::S => KeyCode::S,
                VirtualKeyCode::T => KeyCode::T,
                VirtualKeyCode::U => KeyCode::U,
                VirtualKeyCode::V => KeyCode::V,
                VirtualKeyCode::W => KeyCode::W,
                VirtualKeyCode::X => KeyCode::X,
                VirtualKeyCode::Y => KeyCode::Y,
                VirtualKeyCode::Z => KeyCode::Z,
                VirtualKeyCode::Escape => KeyCode::Escape,
                VirtualKeyCode::F1 => KeyCode::F1,
                VirtualKeyCode::F2 => KeyCode::F2,
                VirtualKeyCode::F3 => KeyCode::F3,
                VirtualKeyCode::F4 => KeyCode::F4,
                VirtualKeyCode::F5 => KeyCode::F5,
                VirtualKeyCode::F6 => KeyCode::F6,
                VirtualKeyCode::F7 => KeyCode::F7,
                VirtualKeyCode::F8 => KeyCode::F8,
                VirtualKeyCode::F9 => KeyCode::F9,
                VirtualKeyCode::F10 => KeyCode::F10,
                VirtualKeyCode::F11 => KeyCode::F11,
                VirtualKeyCode::F12 => KeyCode::F12,
                VirtualKeyCode::F13 => KeyCode::F13,
                VirtualKeyCode::F14 => KeyCode::F14,
                VirtualKeyCode::F15 => KeyCode::F15,
                VirtualKeyCode::F16 => KeyCode::F16,
                VirtualKeyCode::F17 => KeyCode::F17,
                VirtualKeyCode::F18 => KeyCode::F18,
                VirtualKeyCode::F19 => KeyCode::F19,
                VirtualKeyCode::F20 => KeyCode::F20,
                VirtualKeyCode::F21 => KeyCode::F21,
                VirtualKeyCode::F22 => KeyCode::F22,
                VirtualKeyCode::F23 => KeyCode::F23,
                VirtualKeyCode::F24 => KeyCode::F24,
                VirtualKeyCode::Snapshot => KeyCode::Snapshot,
                VirtualKeyCode::Scroll => KeyCode::Scroll,
                VirtualKeyCode::Pause => KeyCode::Pause,
                VirtualKeyCode::Insert => KeyCode::Insert,
                VirtualKeyCode::Home => KeyCode::Home,
                VirtualKeyCode::Delete => KeyCode::Delete,
                VirtualKeyCode::End => KeyCode::End,
                VirtualKeyCode::PageDown => KeyCode::PageDown,
                VirtualKeyCode::PageUp => KeyCode::PageUp,
                VirtualKeyCode::Left => KeyCode::Left,
                VirtualKeyCode::Up => KeyCode::Up,
                VirtualKeyCode::Right => KeyCode::Right,
                VirtualKeyCode::Down => KeyCode::Down,
                VirtualKeyCode::Back => KeyCode::Back,
                VirtualKeyCode::Return => KeyCode::Return,
                VirtualKeyCode::Space => KeyCode::Space,
                VirtualKeyCode::Compose => KeyCode::Compose,
                VirtualKeyCode::Caret => KeyCode::Caret,
                VirtualKeyCode::Numlock => KeyCode::Numlock,
                VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
                VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
                VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
                VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
                VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
                VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
                VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
                VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
                VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
                VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
                VirtualKeyCode::AbntC1 => KeyCode::AbntC1,
                VirtualKeyCode::AbntC2 => KeyCode::AbntC2,
                VirtualKeyCode::Add => KeyCode::Add,
                VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
                VirtualKeyCode::Apps => KeyCode::Apps,
                VirtualKeyCode::At => KeyCode::At,
                VirtualKeyCode::Ax => KeyCode::Ax,
                VirtualKeyCode::Backslash => KeyCode::Backslash,
                VirtualKeyCode::Calculator => KeyCode::Calculator,
                VirtualKeyCode::Capital => KeyCode::Capital,
                VirtualKeyCode::Colon => KeyCode::Colon,
                VirtualKeyCode::Comma => KeyCode::Comma,
                VirtualKeyCode::Convert => KeyCode::Convert,
                VirtualKeyCode::Decimal => KeyCode::Decimal,
                VirtualKeyCode::Divide => KeyCode::Divide,
                VirtualKeyCode::Equals => KeyCode::Equals,
                VirtualKeyCode::Grave => KeyCode::Grave,
                VirtualKeyCode::Kana => KeyCode::Kana,
                VirtualKeyCode::Kanji => KeyCode::Kanji,
                VirtualKeyCode::LAlt => KeyCode::LAlt,
                VirtualKeyCode::LBracket => KeyCode::LBracket,
                VirtualKeyCode::LControl => KeyCode::LControl,
                VirtualKeyCode::LShift => KeyCode::LShift,
                VirtualKeyCode::LWin => KeyCode::LWin,
                VirtualKeyCode::Mail => KeyCode::Mail,
                VirtualKeyCode::MediaSelect => KeyCode::MediaSelect,
                VirtualKeyCode::MediaStop => KeyCode::MediaStop,
                VirtualKeyCode::Minus => KeyCode::Minus,
                VirtualKeyCode::Multiply => KeyCode::Multiply,
                VirtualKeyCode::Mute => KeyCode::Mute,
                VirtualKeyCode::MyComputer => KeyCode::MyComputer,
                VirtualKeyCode::NavigateForward => KeyCode::NavigateForward,
                VirtualKeyCode::NavigateBackward => KeyCode::NavigateBackward,
                VirtualKeyCode::NextTrack => KeyCode::NextTrack,
                VirtualKeyCode::NoConvert => KeyCode::NoConvert,
                VirtualKeyCode::NumpadComma => KeyCode::NumpadComma,
                VirtualKeyCode::NumpadEnter => KeyCode::NumpadEnter,
                VirtualKeyCode::NumpadEquals => KeyCode::NumpadEquals,
                VirtualKeyCode::OEM102 => KeyCode::OEM102,
                VirtualKeyCode::Period => KeyCode::Period,
                VirtualKeyCode::PlayPause => KeyCode::PlayPause,
                VirtualKeyCode::Power => KeyCode::Power,
                VirtualKeyCode::PrevTrack => KeyCode::PrevTrack,
                VirtualKeyCode::RAlt => KeyCode::RAlt,
                VirtualKeyCode::RBracket => KeyCode::RBracket,
                VirtualKeyCode::RControl => KeyCode::RControl,
                VirtualKeyCode::RShift => KeyCode::RShift,
                VirtualKeyCode::RWin => KeyCode::RWin,
                VirtualKeyCode::Semicolon => KeyCode::Semicolon,
                VirtualKeyCode::Slash => KeyCode::Slash,
                VirtualKeyCode::Sleep => KeyCode::Sleep,
                VirtualKeyCode::Stop => KeyCode::Stop,
                VirtualKeyCode::Subtract => KeyCode::Subtract,
                VirtualKeyCode::Sysrq => KeyCode::Sysrq,
                VirtualKeyCode::Tab => KeyCode::Tab,
                VirtualKeyCode::Underline => KeyCode::Underline,
                VirtualKeyCode::Unlabeled => KeyCode::Unlabeled,
                VirtualKeyCode::VolumeDown => KeyCode::VolumeDown,
                VirtualKeyCode::VolumeUp => KeyCode::VolumeUp,
                VirtualKeyCode::Wake => KeyCode::Wake,
                VirtualKeyCode::WebBack => KeyCode::WebBack,
                VirtualKeyCode::WebFavorites => KeyCode::WebFavorites,
                VirtualKeyCode::WebForward => KeyCode::WebForward,
                VirtualKeyCode::WebHome => KeyCode::WebHome,
                VirtualKeyCode::WebRefresh => KeyCode::WebRefresh,
                VirtualKeyCode::WebSearch => KeyCode::WebSearch,
                VirtualKeyCode::WebStop => KeyCode::WebStop,
                VirtualKeyCode::Yen => KeyCode::Yen,
                VirtualKeyCode::Copy => KeyCode::Copy,
                VirtualKeyCode::Paste => KeyCode::Paste,
                VirtualKeyCode::Cut => KeyCode::Cut,
            },
            _ => KeyCode::Unknown,
        }
    }
}
