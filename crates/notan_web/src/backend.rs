#[cfg(feature = "audio")]
use crate::audio::enable_webaudio;
use crate::utils::request_animation_frame;
use crate::window::WebWindowBackend;
use notan_app::{App, Backend, BackendSystem, EventIterator, InitializeFn, WindowBackend};
use notan_app::{FrameState, WindowConfig};
#[cfg(feature = "audio")]
use notan_audio::AudioBackend;
use notan_graphics::DeviceBackend;
#[cfg(feature = "audio")]
use notan_oddio::OddioBackend;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;

#[cfg(feature = "clipboard")]
use crate::clipboard;

pub struct WebBackend {
    window: Option<WebWindowBackend>,
    events: Rc<RefCell<EventIterator>>,
    exit_requested: bool,
}

impl WebBackend {
    pub fn new() -> Result<Self, String> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        let events = Rc::new(RefCell::new(EventIterator::new()));

        Ok(Self {
            window: None,
            events,
            exit_requested: false,
        })
    }
}

impl Backend for WebBackend {
    fn events_iter(&mut self) -> EventIterator {
        self.events.borrow_mut().take_events()
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
        let backend =
            notan_glow::GlowBackend::new(&win.canvas, win.antialias, win.transparent).unwrap();
        Box::new(backend)
    }

    #[cfg(feature = "audio")]
    fn get_audio_backend(&self) -> Rc<RefCell<dyn AudioBackend>> {
        let oddio = OddioBackend::new().unwrap();
        let backend = Rc::new(RefCell::new(oddio));

        let b = backend.clone();
        enable_webaudio(move || b.borrow_mut().enable().unwrap());

        backend as _
    }
}

fn backend(app: &mut App) -> &mut WebBackend {
    app.backend.downcast_mut::<WebBackend>().unwrap()
}
