use crate::window::WinitWindowBackend;
use crate::{keyboard, mouse};
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
        let win = WinitWindowBackend::new(window, &event_loop)?;
        let mut dpi_scale = win
            .window()
            .current_monitor()
            .as_ref()
            .unwrap()
            .scale_factor();
        self.window = Some(win);

        Ok(Box::new(move |mut app: App, mut state: S, mut cb: R| {
            let (mut mouse_x, mut mouse_y) = (0, 0);

            event_loop.run(move |event, _win_target, control_flow| {
                let b = backend(&mut app);
                match event {
                    WEvent::WindowEvent { ref event, .. } => {
                        if let Some(evt) =
                            mouse::process_events(event, &mut mouse_x, &mut mouse_y, dpi_scale)
                        {
                            b.events.push(evt);
                        }

                        if let Some(evt) = keyboard::process_events(event) {
                            b.events.push(evt);
                        }

                        match event {
                            WindowEvent::CloseRequested => {
                                app.exit();
                            }
                            WindowEvent::Resized(size) => {
                                b.window.as_mut().unwrap().gl_ctx.resize(*size);
                                b.events.push(Event::WindowResize {
                                    width: size.width as _,
                                    height: size.height as _,
                                });
                            }
                            WindowEvent::ScaleFactorChanged {
                                scale_factor,
                                new_inner_size: size,
                            } => {
                                println!(" -->> {} {:?} <<-- ", scale_factor, size);
                                b.window.as_mut().unwrap().gl_ctx.resize(**size);
                                dpi_scale = *scale_factor;
                                // b.events.push(Event::WindowResize {
                                //     width: size.width as _,
                                //     height: size.height as _,
                                // });
                                // TODO this is important for the "draw" module when using more than one display with different dpis
                            }
                            WindowEvent::ReceivedCharacter(c) => {
                                b.events.push(Event::ReceivedCharacter(*c));
                            }
                            _ => {}
                        }
                    }
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
