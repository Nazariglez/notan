use crate::keyboard::{enable_keyboard, KeyboardCallbacks};
use crate::mouse::{enable_mouse, MouseCallbacks};
use crate::utils::{
    canvas_add_event_listener, get_notan_size, get_or_create_canvas, set_size_dpi,
    window_add_event_listener,
};
use notan_app::WindowConfig;
use notan_app::{Event, EventIterator, WindowBackend};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Event as WebEvent, HtmlCanvasElement, Window};

pub struct WebWindowBackend {
    pub canvas: HtmlCanvasElement,
    pub window: Window,
    pub document: Document,
    pub canvas_parent: Element,
    pub dpi: f64,

    pub(crate) antialias: bool,

    pub(crate) events: Rc<RefCell<EventIterator>>,

    fullscreen_requested: Rc<RefCell<Option<bool>>>,
    fullscreen_last_size: Rc<RefCell<Option<(i32, i32)>>>,
    fullscreen_callback_ref: Option<Closure<dyn FnMut(WebEvent)>>,

    min_size: Option<(i32, i32)>,
    max_size: Option<(i32, i32)>,
    resize_callback_ref: Option<Closure<dyn FnMut(WebEvent)>>,

    _context_menu_callback_ref: Closure<dyn FnMut(WebEvent)>,

    pub(crate) mouse_callbacks: MouseCallbacks,
    pub(crate) keyboard_callbacks: KeyboardCallbacks,

    config: WindowConfig,
}

impl WebWindowBackend {
    pub fn new(config: WindowConfig, events: Rc<RefCell<EventIterator>>) -> Result<Self, String> {
        let window =
            web_sys::window().ok_or_else(|| String::from("Can't access window dom object."))?;
        let document = window
            .document()
            .ok_or("Can't access document dom object ")?;

        let canvas = get_or_create_canvas(&document, "notan_canvas")?;

        let canvas_parent = canvas
            .parent_element()
            .ok_or("Can't find the canvas parent element.")?;

        let context_menu_callback_ref =
            canvas_add_event_listener(&canvas, "contextmenu", |e: WebEvent| {
                e.prevent_default();
            })?;

        let fullscreen_requested = Rc::new(RefCell::new(None));
        let fullscreen_last_size = Rc::new(RefCell::new(None));
        let fullscreen_callback_ref = None;

        let min_size = config.min_size;
        let max_size = config.max_size;
        let resize_callback_ref = None;

        let mouse_callbacks = Default::default();
        let keyboard_callbacks = Default::default();
        let antialias = config.multisampling != 0;

        let dpi = window.device_pixel_ratio();

        let win = Self {
            window,
            document,
            canvas,
            canvas_parent,
            mouse_callbacks,
            keyboard_callbacks,
            fullscreen_requested,
            fullscreen_last_size,
            fullscreen_callback_ref,
            events,
            min_size,
            max_size,
            resize_callback_ref,
            _context_menu_callback_ref: context_menu_callback_ref,
            config,
            antialias,
            dpi,
        };

        win.init()
    }

    pub(crate) fn open_url(&self, url: &str, new_tab: bool) -> Result<(), String> {
        let target = if new_tab { "_blank" } else { "_self" };

        self.window
            .open_with_url_and_target(url, target)
            .map_err(|err| format!("{:?}", err))?;

        Ok(())
    }

    #[inline]
    pub(crate) fn init(mut self) -> Result<Self, String> {
        if let Err(e) = self.canvas.set_attribute(
            "notan-auto-res",
            &self.config.canvas_auto_resolution.to_string(),
        ) {
            log::error!("{:?}", e);
        }

        let (ww, hh) = if self.config.maximized {
            (
                self.canvas_parent.client_width(),
                self.canvas_parent.client_height(),
            )
        } else {
            (self.config.width, self.config.height)
        };

        self.set_size(ww, hh);

        let fullscreen_dispatcher = fullscreen_dispatcher_callback(&mut self);

        enable_mouse(&mut self, fullscreen_dispatcher.clone())?;
        enable_keyboard(&mut self, fullscreen_dispatcher.clone())?;

        if self.config.resizable {
            enable_resize(&mut self)?;
        }

        enable_fullscreen(&mut self)?;
        if self.config.fullscreen {
            self.set_fullscreen(true);
        }

        Ok(self)
    }

    pub(crate) fn check_dpi(&mut self) {
        let dpi = self.window.device_pixel_ratio();
        if (dpi - self.dpi).abs() > f64::EPSILON {
            let (ww, hh) = get_notan_size(&self.canvas);
            self.dpi = dpi;
            self.events
                .borrow_mut()
                .push(Event::ScreenAspectChange { ratio: dpi });
            self.set_size(ww as _, hh as _);
        }
    }
}

impl WindowBackend for WebWindowBackend {
    fn set_size(&mut self, width: i32, height: i32) {
        set_size_dpi(&self.canvas, width as _, height as _);
        self.config.width = width;
        self.config.height = height;
    }

    fn size(&self) -> (i32, i32) {
        get_notan_size(&self.canvas)
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        *self.fullscreen_requested.borrow_mut() = Some(enabled);
    }

    fn is_fullscreen(&self) -> bool {
        self.document.fullscreen()
    }

    fn dpi(&self) -> f64 {
        if self.config.canvas_auto_resolution {
            self.dpi
        } else {
            1.0
        }
    }
}

unsafe impl Send for WebWindowBackend {}
unsafe impl Sync for WebWindowBackend {}

fn enable_fullscreen(win: &mut WebWindowBackend) -> Result<(), String> {
    if win.fullscreen_callback_ref.is_none() {
        let events = win.events.clone();
        let canvas = win.canvas.clone();
        let document = win.document.clone();
        let last_size = win.fullscreen_last_size.clone();
        win.fullscreen_callback_ref =
            Some(window_add_event_listener("fullscreenchange", move |_| {
                let (width, height) = if document.fullscreen() {
                    (canvas.client_width(), canvas.client_height())
                } else {
                    match *last_size.borrow() {
                        Some(size) => size,
                        _ => {
                            log::error!("Invalid fullscreen disabled size.");
                            (800, 600)
                        }
                    }
                };

                log::info!("callback -> {:?} {} {}", last_size, width, height);
                set_size_dpi(&canvas, width, height);
                events
                    .borrow_mut()
                    .push(Event::WindowResize { width, height });
            })?);
    }

    Ok(())
}

fn fullscreen_dispatcher_callback(win: &mut WebWindowBackend) -> Rc<RefCell<dyn Fn()>> {
    let fullscreen_requested = win.fullscreen_requested.clone();
    let canvas = win.canvas.clone();
    let doc = win.document.clone();
    let last_size = win.fullscreen_last_size.clone();
    Rc::new(RefCell::new(move || {
        if let Some(full) = fullscreen_requested.borrow_mut().take() {
            if full {
                let width = canvas.client_width();
                let height = canvas.client_height();
                *last_size.borrow_mut() = Some((width, height));
                if let Err(e) = canvas.request_fullscreen() {
                    log::error!("{:?}", e);
                }
            } else {
                doc.exit_fullscreen();
            }
        }
    }))
}

fn enable_resize(win: &mut WebWindowBackend) -> Result<(), String> {
    let events = win.events.clone();
    let canvas = win.canvas.clone();
    let parent = win.canvas_parent.clone();
    let min_size = win.min_size;
    let max_size = win.max_size;
    win.resize_callback_ref = Some(window_add_event_listener("resize", move |_| {
        let mut p_width = parent.client_width();
        let mut p_height = parent.client_height();

        if let Some((w, h)) = min_size {
            if p_width < w {
                p_width = w;
            }

            if p_height < h {
                p_height = h;
            }
        }

        if let Some((w, h)) = max_size {
            if p_width > w {
                p_width = w;
            }

            if p_height > h {
                p_height = h;
            }
        }

        set_size_dpi(&canvas, p_width as _, p_height as _);
        events.borrow_mut().push(Event::WindowResize {
            width: p_width,
            height: p_height,
        });
    })?);
    Ok(())
}
