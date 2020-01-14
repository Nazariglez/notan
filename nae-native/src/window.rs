use super::System;
use glutin::{dpi::LogicalSize, ContextBuilder, PossiblyCurrent, WindowedContext};
use nae_core::window::BaseWindow;
use nae_core::{BaseApp, BaseSystem, Event, KeyCode, MouseButton};
use nae_glow::Context2d;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};
use winit::event::{ElementState, Event as WinitEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub struct Window {
    pub(crate) win: WindowedContext<PossiblyCurrent>,
    title: String,
    width: i32,
    height: i32,
    fullscreen: bool,
    dpi: f32,
}

impl Window {
    pub(crate) fn new(
        title: &str,
        width: i32,
        height: i32,
        event_loop: &EventLoop<()>,
    ) -> Result<Self, String> {
        let win_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width as f64, height as f64));

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

        Ok(Self {
            width,
            height,
            title: title.to_string(),
            fullscreen: false,
            win,
            dpi,
        })
    }
}

impl BaseWindow for Window {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
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

pub fn run<A, S, F, D>(mut app: A, mut state: S, mut update: F, mut draw: D)
where
    A: BaseApp<System = System> + 'static,
    S: 'static,
    F: FnMut(&mut A, &mut S) + 'static,
    D: FnMut(&mut A, &mut S) + 'static,
{
    let mut event_loop = app.system().event_loop.take().unwrap();
    let mut running = true;
    let (mut last_mouse_x, mut last_mouse_y) = (0, 0);
    let mut last_key_code: Option<Event> = None;

    event_loop.run(move |event, target, mut control| {
        if !running {
            return;
        }
        match event {
            WinitEvent::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    running = false;
                    *control = ControlFlow::Exit;
                    return;
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => {
                    println!("scale_factor: {} {:?}", scale_factor, new_inner_size);
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
                    let was_pressed = match input.state {
                        ElementState::Pressed => true,
                        _ => false,
                    };
                    let code = input.scancode;
                    let key = input.virtual_keycode;
                    //                    println!("{:?} {:?} {:?}", was_pressed, code, key);

                    if last_key_code.is_some() {
                        println!("sent last: {:?}", last_key_code);
                        last_key_code = None;
                    }

                    last_key_code = Some(Event::KeyDown {
                        key: nae_core::KeyCode::R,
                        character: '\0',
                    });
                }
                WindowEvent::ReceivedCharacter(c) => {
                    println!("char: {} {}", c, *c as u32);
                    if let Some(Event::KeyDown { key, character }) = &mut last_key_code {
                        *character = *c;
                    }
                    println!("sent last: {:?}", last_key_code);
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
}

trait ToNaeValue {
    type Kind;

    fn to_nae(&self) -> Self::Kind;
}

use winit::event::MouseButton as WinitMB;

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
