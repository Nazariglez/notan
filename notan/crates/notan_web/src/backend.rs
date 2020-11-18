use crate::utils::request_animation_frame;
use crate::window::WebWindowBackend;
use notan_app::config::WindowConfig;
use notan_app::{App, Backend, BackendSystem, EventIterator, InitializeFn, WindowBackend};
use notan_log as log;
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;

pub struct WebBackend {
    window: Option<WebWindowBackend>,
    events: Rc<RefCell<EventIterator>>,
    exit_requested: bool,
}

impl WebBackend {
    pub fn new() -> Result<Self, String> {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
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

    fn window(&mut self) -> &mut dyn WindowBackend {
        self.window.as_mut().unwrap()
    }

    fn exit(&mut self) {
        self.exit_requested = true;
    }
}

impl BackendSystem for WebBackend {
    fn initialize<S, R>(&mut self, window: WindowConfig) -> Result<Box<InitializeFn<S, R>>, String>
    where
        S: 'static,
        R: FnMut(&mut App, &mut S) -> Result<(), String> + 'static,
    {
        self.window = Some(WebWindowBackend::new(window, self.events.clone())?);

        Ok(Box::new(move |mut app: App, mut state: S, mut cb: R| {
            let callback = Rc::new(RefCell::new(None));
            let inner_callback = callback.clone();

            backend(&mut app).window.as_mut().unwrap().init()?;

            *callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                if let Err(e) = cb(&mut app, &mut state) {
                    log::error!("{}", e);
                    return;
                }

                let backend = backend(&mut app);
                if !backend.exit_requested {
                    request_animation_frame(
                        &backend.window.as_ref().unwrap().window,
                        inner_callback.borrow().as_ref().unwrap(),
                    );
                }
            }) as Box<dyn FnMut()>));

            let window = web_sys::window().unwrap();
            request_animation_frame(&window, callback.borrow().as_ref().unwrap());
            Ok(())
        }))
    }
}

unsafe impl Send for WebBackend {}
unsafe impl Sync for WebBackend {}

fn backend(app: &mut App) -> &mut WebBackend {
    app.backend.downcast_mut::<WebBackend>().unwrap()
}
