use crate::audio::{fix_webaudio_if_necessary, DummyAudioBackend};
use crate::utils::{request_animation_frame, window_add_event_listener};
use crate::window::WebWindowBackend;
use notan_app::{App, Backend, BackendSystem, EventIterator, InitializeFn, WindowBackend};
use notan_app::{FrameState, WindowConfig};
use notan_audio::AudioBackend;
use notan_graphics::DeviceBackend;
use notan_oddio::OddioBackend;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use web_sys::MouseEvent;

pub struct WebBackend {
    window: Option<WebWindowBackend>,
    events: Rc<RefCell<EventIterator>>,
    exit_requested: bool,
    audio: Rc<RefCell<Box<dyn AudioBackend>>>,
}

impl WebBackend {
    pub fn new() -> Result<Self, String> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        let events = Rc::new(RefCell::new(EventIterator::new()));
        let audio: Rc<RefCell<Box<dyn AudioBackend>>> =
            Rc::new(RefCell::new(Box::new(DummyAudioBackend::new())));

        Ok(Self {
            window: None,
            events,
            exit_requested: false,
            audio,
        })
    }
}

impl Backend for WebBackend {
    fn events_iter(&mut self) -> EventIterator {
        self.events.borrow_mut().take_events()
    }

    fn window(&mut self) -> &mut dyn WindowBackend {
        self.window.as_mut().unwrap()
    }

    fn exit(&mut self) {
        self.exit_requested = true;
    }

    fn system_timestamp(&self) -> u64 {
        js_sys::Date::now() as u64
    }

    fn open_link(&self, url: &str, new_tab: bool) {
        if let Err(err) = self.window.as_ref().unwrap().open_url(url, new_tab) {
            log::error!("{}", err);
        }
    }
}

impl BackendSystem for WebBackend {
    fn initialize<S, R>(&mut self, window: WindowConfig) -> Result<Box<InitializeFn<S, R>>, String>
    where
        S: 'static,
        R: FnMut(&mut App, &mut S) -> Result<FrameState, String> + 'static,
    {
        let callback = Rc::new(RefCell::new(None));
        let win = WebWindowBackend::new(window, self.events.clone(), callback.clone())?;
        self.window = Some(win);

        Ok(Box::new(move |mut app: App, mut state: S, mut cb: R| {
            let inner_callback = callback.clone();

            *callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                let backend = backend(&mut app);
                if !backend.exit_requested {
                    let win = backend.window.as_mut().unwrap();
                    win.check_dpi();

                    if win.lazy_loop() {
                        *win.frame_requested.borrow_mut() = false;
                    } else {
                        request_animation_frame(
                            &win.window,
                            inner_callback.borrow().as_ref().unwrap(),
                        );
                    }
                }

                if let Err(e) = cb(&mut app, &mut state) {
                    log::error!("{}", e);
                }
            }) as Box<dyn FnMut()>));

            let window = web_sys::window().unwrap();
            request_animation_frame(&window, callback.borrow().as_ref().unwrap());
            Ok(())
        }))
    }

    fn get_graphics_backend(&self) -> Box<dyn DeviceBackend> {
        let win = self.window.as_ref().unwrap();
        let backend = notan_glow::GlowBackend::new(&win.canvas, win.antialias).unwrap();
        Box::new(backend)
    }

    fn get_audio_backend(&self) -> Rc<RefCell<Box<dyn AudioBackend>>> {
        let audio = self.audio.clone();
        let c = window_add_event_listener("click", move |e: MouseEvent| {
            *audio.borrow_mut() = Box::new(OddioBackend::new().unwrap());
        });
        std::mem::forget(c);

        self.audio.clone()
    }
}

fn backend(app: &mut App) -> &mut WebBackend {
    app.backend.downcast_mut::<WebBackend>().unwrap()
}
