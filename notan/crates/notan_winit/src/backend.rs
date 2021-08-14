use crate::messages::*;
use crate::window::WinitWindowBackend;
use crossbeam_channel::{Receiver, Sender};
use notan_app::{App, Backend, InitializeFn, WindowBackend};
use winit::dpi::LogicalSize;
use winit::event::DeviceEvent::Button;
use winit::event::Event::DeviceEvent;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::macos::WindowExtMacOS;
use winit::window::{Fullscreen, Window, WindowBuilder};

pub struct WinitBackend {
    sender: Sender<BackendMessages>,
    exit_requested: bool,
    window: WinitWindowBackend,
}

impl WinitBackend {
    pub fn new() -> Result<Self, String> {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let size = (800, 600);

        let window = WinitWindowBackend {
            sender: sender.clone(),
            receiver,
            is_fullscreen: false,
            size,
        };

        Ok(Self {
            sender,
            exit_requested: false,

            window,
        })
    }
}

impl Backend for WinitBackend {
    type Impl = WinitBackend;
    type Window = WinitWindowBackend;

    fn get_impl(&mut self) -> &mut Self::Impl {
        self
    }

    fn initialize<B, S, R>(&mut self) -> Result<Box<InitializeFn<B, S, R>>, String>
    where
        B: Backend<Impl = Self::Impl> + 'static,
        S: 'static,
        R: FnMut(&mut App<B>, &mut S) + 'static,
    {
        Ok(Box::new(move |mut app: App<B>, mut state: S, mut cb: R| {
            let event_loop = EventLoop::new();

            let window = WindowBuilder::new()
                .with_title("yeah")
                .build(&event_loop)
                .map_err(|e| format!("{:?}", e))
                .unwrap();

            let backend = app.backend.get_impl();
            let receiver = backend.window.receiver.clone();

            std::thread::spawn(move || {
                use BackendMessages::*;
                while let Ok(evt) = receiver.recv() {
                    match evt {
                        FullscreenMode(enabled) => {
                            let mode = if enabled {
                                let monitor = window.current_monitor();
                                Some(Fullscreen::Borderless(monitor))
                            } else {
                                None
                            };

                            window.set_fullscreen(mode);
                        }
                        Size { width, height } => {
                            window.set_inner_size(LogicalSize::new(width, height));
                        }
                        Exit => {
                            break;
                        }
                    }
                }
            });

            event_loop.run(move |event, target, mut control| {
                *control = ControlFlow::Poll;

                let backend = app.backend.get_impl();

                match event {
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            // running = false;
                            *control = ControlFlow::Exit;
                            return;
                        }
                        _ => {}
                    },
                    Event::DeviceEvent { device_id, event } => match event {
                        Button { button, state } => {
                            println!("{:?} {:?}", button, state);
                            let (width, height) = backend.window.size();
                            if button == 0 {
                                backend.window.set_size(width + 10, height + 10);
                            //app.backend.get_impl().set_fullscreen(true);
                            } else {
                                backend.window.set_size(width - 10, height - 10);
                                //app.backend.get_impl().set_fullscreen(false);
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }

                cb(&mut app, &mut state);

                if app.backend.get_impl().exit_requested {
                    *control = ControlFlow::Exit;
                }
            });
            Ok(())
        }))
    }

    fn window(&mut self) -> &mut Self::Window {
        &mut self.window
    }

    fn exit(&mut self) {
        if self.sender.send(BackendMessages::Exit).is_ok() {
            self.exit_requested = true;
        }
    }
}
