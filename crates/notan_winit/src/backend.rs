use crate::window::WinitWindowBackend;
use crate::{keyboard, mouse, touch};
use notan_app::{FrameState, WindowConfig};
use winit::application::ApplicationHandler;
use winit::event_loop::ControlFlow;

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

use glutin::display::GlDisplay;
#[cfg(feature = "audio")]
use std::cell::RefCell;
use std::ffi::CString;
#[cfg(feature = "audio")]
use std::rc::Rc;

use winit::event::WindowEvent;
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

struct AppHandler<R, S>
where
    R: FnMut(&mut App, &mut S) -> Result<FrameState, String> + 'static,
    S: 'static,
{
    app: App,
    dpi_scale: f64,
    mouse_x: i32,
    mouse_y: i32,
    request_redraw: bool,
    state: S,
    cb: R,
}
fn add_event(b: &mut WinitBackend, request_redraw: &mut bool, evt: Event) {
    b.events.push(evt);
    *request_redraw = true;
}

impl<R, S> ApplicationHandler for AppHandler<R, S>
where
    R: FnMut(&mut App, &mut S) -> Result<FrameState, String> + 'static,
    S: 'static,
{
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let b = backend(&mut self.app.backend);

        // Await for the next event to run the loop again
        let is_lazy = b.window.as_ref().is_some_and(|w| w.lazy);

        if let Some(evt) =
            mouse::process_events(&event, &mut self.mouse_x, &mut self.mouse_y, self.dpi_scale)
        {
            add_event(b, &mut self.request_redraw, evt);
        }

        if let Some(evt) = keyboard::process_events(&event) {
            add_event(b, &mut self.request_redraw, evt);
        }

        keyboard::process_char_events(&event, |e| add_event(b, &mut self.request_redraw, e));

        if let Some(evt) = touch::process_events(&event, self.dpi_scale) {
            add_event(b, &mut self.request_redraw, evt);
        }

        #[cfg(feature = "clipboard")]
        if let Some(evt) = clipboard::process_events(&event, &self.app.keyboard) {
            add_event(b, &mut self.request_redraw, evt);
        }

        match event {
            WindowEvent::Touch(t) => {
                println!("{t:?}");
            }
            WindowEvent::CloseRequested => {
                self.app.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(win) = &mut b.window {
                    win.resize(size.width, size.height);
                }

                let logical_size = size.to_logical::<f64>(self.dpi_scale);
                add_event(
                    b,
                    &mut self.request_redraw,
                    Event::WindowResize {
                        width: logical_size.width as _,
                        height: logical_size.height as _,
                    },
                );
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                if let Some(win) = &mut b.window {
                    //win.resize(size.width, size.height);
                    self.dpi_scale = scale_factor;
                    win.scale_factor = self.dpi_scale;
                }

                //let logical_size = size.to_logical::<f64>(dpi_scale);

                add_event(
                    b,
                    &mut self.request_redraw,
                    Event::ScreenAspectChange {
                        ratio: self.dpi_scale,
                    },
                );
            }
            #[cfg(feature = "drop_files")]
            WindowEvent::HoveredFile(path) => {
                let name = path
                    .file_name()
                    .map_or_else(|| "".to_string(), |n| n.to_string_lossy().to_string());

                let mime = mime_guess::from_path(&path)
                    .first_raw()
                    .unwrap_or("")
                    .to_string();

                add_event(
                    b,
                    &mut self.request_redraw,
                    Event::DragEnter {
                        path: Some(path.clone()),
                        name: Some(name),
                        mime,
                    },
                );
            }
            #[cfg(feature = "drop_files")]
            WindowEvent::HoveredFileCancelled => {
                add_event(b, &mut self.request_redraw, Event::DragLeft);
            }
            #[cfg(feature = "drop_files")]
            WindowEvent::DroppedFile(path) => {
                let name = path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_else(|| "".to_string());

                let mime = mime_guess::from_path(&path)
                    .first_raw()
                    .unwrap_or("")
                    .to_string();

                add_event(
                    b,
                    &mut self.request_redraw,
                    Event::Drop(DroppedFile {
                        path: Some(path.clone()),
                        name,
                        mime,
                    }),
                );
            }
            WindowEvent::RedrawRequested => {
                self.request_redraw = false;
                if let Some(w) = &mut b.window {
                    w.frame_requested = false;
                }

                match (self.cb)(&mut self.app, &mut self.state) {
                    Ok(FrameState::End) => {
                        backend(&mut self.app.backend)
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

            _ => {}
        }

        if backend(&mut self.app.backend).exit_requested {
            event_loop.exit();
            return;
        }
        let control_flow = {
            if self.request_redraw {
                // If something needs to be drawn keep polling events
                ControlFlow::Poll
            } else if is_lazy {
                // If is in lazy mode and nothing needs to be drawn just wait
                ControlFlow::Wait
            } else {
                // by default keep polling events
                ControlFlow::Poll
            }
        };
        event_loop.set_control_flow(control_flow);
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let Some(evt) = mouse::process_device_events(&event) {
            let b = backend(&mut self.app.backend);
            add_event(b, &mut self.request_redraw, evt);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let b = backend(&mut self.app.backend);
        let is_lazy = b.window.as_ref().is_some_and(|w| w.lazy);
        let needs_redraw =
            !is_lazy || self.request_redraw || b.window.as_ref().is_some_and(|w| w.frame_requested);
        if needs_redraw {
            if let Some(win) = &mut b.window {
                win.window().request_redraw();
            }
        }
    }
}

impl BackendSystem for WinitBackend {
    fn initialize<S, R>(&mut self, window: WindowConfig) -> Result<Box<InitializeFn<S, R>>, String>
    where
        S: 'static,
        R: FnMut(&mut App, &mut S) -> Result<FrameState, String> + 'static,
    {
        let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
        let win = WinitWindowBackend::new(window, &event_loop)?;
        let dpi_scale = win
            .window()
            .current_monitor()
            .as_ref()
            .map_or(1.0, |m| m.scale_factor());
        self.window = Some(win);

        Ok(Box::new(move |app: App, state: S, cb: R| {
            let mut handler = AppHandler {
                app,
                dpi_scale,
                mouse_x: 0,
                mouse_y: 0,
                request_redraw: false,
                state,
                cb,
            };
            event_loop.run_app(&mut handler).map_err(|e| e.to_string())
        }))
    }

    fn get_graphics_backend(&self) -> Box<dyn DeviceBackend> {
        let ctx = &self.window.as_ref().unwrap().gl_manager.display;
        let backend = notan_glow::GlowBackend::new(|s| {
            let symbol = CString::new(s).unwrap();
            ctx.get_proc_address(symbol.as_c_str()).cast()
        })
        .unwrap();
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
