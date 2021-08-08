use crate::window::WinitWindowBackend;
use glutin::event::ElementState;
use glutin::event_loop::ControlFlow;
use notan_app::buffer::VertexAttr;
use notan_app::commands::Commands;
use notan_app::config::WindowConfig;
use notan_app::graphics::pipeline::PipelineOptions;
use notan_app::prelude::mouse::MouseButton;
use notan_app::{
    App, Backend, BackendSystem, DeviceBackend, Event, EventIterator, InitializeFn, LoadFileFn,
    ResourceId, TextureInfo, TextureUpdate, WindowBackend,
};
use winit::event::{Event as WEvent, WindowEvent};
use winit::event_loop::EventLoop;

pub struct WinitBackend {
    window: Option<WinitWindowBackend>,
    events: EventIterator,
    exit_requested: bool,
}

impl WinitBackend {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            events: EventIterator::new(),
            window: None,
            exit_requested: false,
        })
    }
}

impl Backend for WinitBackend {
    fn window(&mut self) -> &mut dyn WindowBackend {
        self.window.as_mut().unwrap()
    }

    fn events_iter(&mut self) -> EventIterator {
        self.events.take_events()
    }

    fn exit(&mut self) {
        self.exit_requested = true;
    }

    fn system_timestamp(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

impl BackendSystem for WinitBackend {
    fn initialize<S, R>(&mut self, window: WindowConfig) -> Result<Box<InitializeFn<S, R>>, String>
    where
        S: 'static,
        R: FnMut(&mut App, &mut S) -> Result<(), String> + 'static,
    {
        let event_loop = EventLoop::new();
        self.window = Some(WinitWindowBackend::new(window, &event_loop)?);

        Ok(Box::new(move |mut app: App, mut state: S, mut cb: R| {
            let (mut mouse_x, mut mouse_y) = (0, 0);

            event_loop.run(move |event, _win_target, control_flow| {
                let b = backend(&mut app);
                match event {
                    WEvent::WindowEvent { ref event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            app.exit();
                        }
                        WindowEvent::Resized(size) => {
                            b.events.push(Event::WindowResize {
                                width: size.width as _,
                                height: size.height as _,
                            });
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            let evt = match state {
                                // TODO
                                ElementState::Pressed => Event::MouseDown {
                                    button: MouseButton::Left,
                                    x: mouse_x,
                                    y: mouse_y,
                                },
                                _ => Event::MouseUp {
                                    button: MouseButton::Left,
                                    x: mouse_x,
                                    y: mouse_y,
                                },
                            };
                            b.events.push(evt);
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            mouse_x = position.x as _;
                            mouse_y = position.y as _;
                            b.events.push(Event::MouseMove {
                                x: mouse_x,
                                y: mouse_y,
                            });
                        }
                        _ => {}
                    },
                    WEvent::MainEventsCleared => {
                        b.window.as_mut().unwrap().window().request_redraw();
                    }
                    WEvent::RedrawRequested(_) => {
                        cb(&mut app, &mut state);
                        backend(&mut app).window.as_mut().unwrap().swap_buffers();
                    }
                    _ => {}
                }

                let exit_requested = backend(&mut app).exit_requested;
                if exit_requested {
                    *control_flow = ControlFlow::Exit;
                }
            });
        }))
    }

    fn get_graphics_backend(&self) -> Box<DeviceBackend> {
        let ctx = &self.window.as_ref().unwrap().gl_ctx;
        let backend =
            notan_glow::GlowBackend::new(|s| ctx.get_proc_address(s) as *const _).unwrap();
        Box::new(backend)
    }
}

fn backend(app: &mut App) -> &mut WinitBackend {
    app.backend.downcast_mut::<WinitBackend>().unwrap()
}
