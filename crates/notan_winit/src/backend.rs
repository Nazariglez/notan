use crate::window::WinitWindowBackend;
use crate::{keyboard, mouse, touch};
use glutin::event_loop::ControlFlow;
use notan_app::{FrameState, WindowConfig};

#[cfg(feature = "clipboard")]
use crate::clipboard;

#[cfg(feature = "drop_files")]
use notan_app::DroppedFile;

use notan_app::{
    App, Backend, BackendSystem, DeviceBackend, Event, EventIterator, InitializeFn, WindowBackend,
};
#[cfg(feature = "audio")]
use notan_audio::AudioBackend;
#[cfg(feature = "audio")]
use notan_oddio::OddioBackend;

#[cfg(feature = "audio")]
use std::cell::RefCell;
#[cfg(feature = "audio")]
use std::rc::Rc;

use glutin::event::{Event as WEvent, WindowEvent};
use glutin::event_loop::EventLoop;

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

    fn set_clipboard_text(&mut self, text: &str) {
        #[cfg(feature = "clipboard")]
        clipboard::set_clipboard_text(text);

        #[cfg(not(feature = "clipboard"))]
        {
            log::warn!(
                "Cannot set {} to clipboard without the feature 'clipboard' enabled.",
                text
            );
        }
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

    fn open_link(&self, url: &str, _new_tab: bool) {
        #[cfg(feature = "links")]
        {
            if let Err(err) = webbrowser::open(url) {
                log::error!("Error opening {}: {}", url, err);
            }
        }

        #[cfg(not(feature = "links"))]
        {
            log::warn!("Cannot {} link without the feature 'links' enabled.", url);
        }
    }
}

impl BackendSystem for WinitBackend {
    fn initialize<S, R>(&mut self, window: WindowConfig) -> Result<Box<InitializeFn<S, R>>, String>
    where
        S: 'static,
        R: FnMut(&mut App, &mut S) -> Result<FrameState, String> + 'static,
    {
        let event_loop = EventLoop::new();
        let win = WinitWindowBackend::new(window, &event_loop)?;
        let mut dpi_scale = win
            .window()
            .current_monitor()
            .as_ref()
            .map_or(1.0, |m| m.scale_factor());
        self.window = Some(win);

        Ok(Box::new(move |mut app: App, mut state: S, mut cb: R| {
            let (mut mouse_x, mut mouse_y) = (0, 0);
            let mut request_redraw = false;

            let add_event = move |b: &mut WinitBackend, request_redraw: &mut bool, evt: Event| {
                b.events.push(evt);
                *request_redraw = true;
            };

            event_loop.run(move |event, _win_target, control_flow| {
                let b = backend(&mut app.backend);

                // Await for the next event to run the loop again
                let is_lazy = b.window.as_ref().unwrap().lazy;
                if is_lazy {
                    *control_flow = ControlFlow::Wait;
                }

                match event {
                    WEvent::WindowEvent { ref event, .. } => {
                        if let Some(evt) =
                            mouse::process_events(event, &mut mouse_x, &mut mouse_y, dpi_scale)
                        {
                            add_event(b, &mut request_redraw, evt);
                        }

                        if let Some(evt) = keyboard::process_events(event) {
                            add_event(b, &mut request_redraw, evt);
                        }

                        if let Some(evt) = touch::process_events(event, dpi_scale) {
                            add_event(b, &mut request_redraw, evt);
                        }

                        #[cfg(feature = "clipboard")]
                        if let Some(evt) = clipboard::process_events(event, &app.keyboard) {
                            add_event(b, &mut request_redraw, evt);
                        }

                        match event {
                            WindowEvent::Touch(t) => {
                                println!("{:?}", t);
                            }
                            WindowEvent::CloseRequested => {
                                app.exit();
                            }
                            WindowEvent::Resized(size) => {
                                b.window.as_mut().unwrap().gl_ctx.resize(*size);

                                let logical_size = size.to_logical::<f64>(dpi_scale);
                                add_event(
                                    b,
                                    &mut request_redraw,
                                    Event::WindowResize {
                                        width: logical_size.width as _,
                                        height: logical_size.height as _,
                                    },
                                );
                            }
                            WindowEvent::ScaleFactorChanged {
                                scale_factor,
                                new_inner_size: size,
                            } => {
                                b.window.as_mut().unwrap().gl_ctx.resize(**size);
                                let win = b.window.as_mut().unwrap();
                                dpi_scale = *scale_factor;
                                win.scale_factor = dpi_scale;

                                let logical_size = size.to_logical::<f64>(dpi_scale);

                                add_event(
                                    b,
                                    &mut request_redraw,
                                    Event::ScreenAspectChange { ratio: dpi_scale },
                                );
                                add_event(
                                    b,
                                    &mut request_redraw,
                                    Event::WindowResize {
                                        width: logical_size.width as _,
                                        height: logical_size.height as _,
                                    },
                                );
                            }
                            WindowEvent::ReceivedCharacter(c) => {
                                add_event(b, &mut request_redraw, Event::ReceivedCharacter(*c));
                            }

                            #[cfg(feature = "drop_files")]
                            WindowEvent::HoveredFile(path) => {
                                let name = path.file_name().map_or_else(
                                    || "".to_string(),
                                    |n| n.to_string_lossy().to_string(),
                                );

                                let mime = mime_guess::from_path(path)
                                    .first_raw()
                                    .unwrap_or("")
                                    .to_string();

                                add_event(
                                    b,
                                    &mut request_redraw,
                                    Event::DragEnter {
                                        path: Some(path.clone()),
                                        name: Some(name),
                                        mime,
                                    },
                                );
                            }
                            #[cfg(feature = "drop_files")]
                            WindowEvent::HoveredFileCancelled => {
                                add_event(b, &mut request_redraw, Event::DragLeft);
                            }
                            #[cfg(feature = "drop_files")]
                            WindowEvent::DroppedFile(path) => {
                                let name = path
                                    .file_name()
                                    .map(|name| name.to_string_lossy().to_string())
                                    .unwrap_or_else(|| "".to_string());

                                let mime = mime_guess::from_path(path)
                                    .first_raw()
                                    .unwrap_or("")
                                    .to_string();

                                add_event(
                                    b,
                                    &mut request_redraw,
                                    Event::Drop(DroppedFile {
                                        path: Some(path.clone()),
                                        name,
                                        mime,
                                    }),
                                );
                            }

                            _ => {}
                        }
                    }
                    WEvent::MainEventsCleared => {
                        let needs_redraw = !is_lazy || request_redraw;
                        if needs_redraw {
                            b.window.as_mut().unwrap().window().request_redraw();
                        }
                    }
                    WEvent::RedrawRequested(_) => {
                        match cb(&mut app, &mut state) {
                            Ok(FrameState::End) => {
                                backend(&mut app.backend)
                                    .window
                                    .as_mut()
                                    .unwrap()
                                    .swap_buffers();
                            }
                            Ok(FrameState::Skip) => {
                                // log::debug!("Frame skipped");
                                // no-op
                            }
                            Err(e) => {
                                log::error!("{}", e);
                            }
                        }
                    }
                    WEvent::RedrawEventsCleared => {
                        request_redraw = false;
                    }
                    _ => {}
                }

                let b = backend(&mut app.backend);

                // Close the loop if the user want to exit
                let exit_requested = b.exit_requested;
                if exit_requested {
                    *control_flow = ControlFlow::Exit;
                }
            });
        }))
    }

    fn get_graphics_backend(&self) -> Box<dyn DeviceBackend> {
        let ctx = &self.window.as_ref().unwrap().gl_ctx;
        let backend =
            notan_glow::GlowBackend::new(|s| ctx.get_proc_address(s) as *const _).unwrap();
        Box::new(backend)
    }

    #[cfg(feature = "audio")]
    fn get_audio_backend(&self) -> Rc<RefCell<dyn AudioBackend>> {
        let backend = OddioBackend::new().unwrap();
        Rc::new(RefCell::new(backend))
    }
}

fn backend(backend: &mut Box<dyn Backend>) -> &mut WinitBackend {
    backend.downcast_mut::<WinitBackend>().unwrap()
}
