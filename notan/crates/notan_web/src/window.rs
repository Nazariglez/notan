use crate::keyboard::{enable_keyboard, KeyboardCallbacks};
use crate::mouse::{enable_mouse, MouseCallbacks};
use crate::utils::{canvas_add_event_listener, get_or_create_canvas, window_add_event_listener};
use notan_app::{Event, EventIterator, WindowBackend};
use notan_log as log;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Event as WebEvent, HtmlCanvasElement, Window};

pub struct WebWindowBackend {
    pub canvas: HtmlCanvasElement,
    pub window: Window,
    pub document: Document,
    pub canvas_parent: Element,

    pub(crate) events: Rc<RefCell<EventIterator>>,

    fullscreen_requested: Rc<RefCell<Option<bool>>>,
    fullscreen_last_size: Rc<RefCell<Option<(i32, i32)>>>,
    fullscreen_callback_ref: Option<Closure<FnMut(WebEvent)>>,

    min_size: Option<(i32, i32)>,
    max_size: Option<(i32, i32)>,
    resize_callback_ref: Option<Closure<FnMut(WebEvent)>>,

    context_menu_callback_ref: Closure<FnMut(WebEvent)>,

    pub(crate) mouse_callbacks: MouseCallbacks,
    pub(crate) keyboard_callbacks: KeyboardCallbacks,
}

impl WebWindowBackend {
    pub fn new(events: Rc<RefCell<EventIterator>>) -> Result<Self, String> {
        let window = web_sys::window().ok_or(String::from("Can't access window dom object."))?;
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

        let min_size = None; //From options
        let max_size = None; //From options
        let resize_callback_ref = None;

        let mouse_callbacks = Default::default();
        let keyboard_callbacks = Default::default();

        Ok(Self {
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
            context_menu_callback_ref,
        })
    }

    pub(crate) fn enable_events(&mut self) -> Result<(), String> {
        let fullscreen_dispatcher = fullscreen_dispatcher_callback(self);

        enable_mouse(self, fullscreen_dispatcher.clone())?;
        enable_keyboard(self, fullscreen_dispatcher.clone())?;

        //if is_resizable {
        enable_resize(self)?;
        // }

        enable_fullscreen(self)?;

        Ok(())
    }
}

impl WindowBackend for WebWindowBackend {
    fn set_size(&mut self, width: i32, height: i32) {
        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
    }

    fn size(&self) -> (i32, i32) {
        (self.canvas.client_width(), self.canvas.client_height())
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        *self.fullscreen_requested.borrow_mut() = Some(enabled);
    }

    fn is_fullscreen(&self) -> bool {
        self.document.fullscreen()
    }
}

unsafe impl Send for WebWindowBackend {}
unsafe impl Sync for WebWindowBackend {}

fn enable_fullscreen(win: &mut WebWindowBackend) -> Result<(), String> {
    if win.fullscreen_callback_ref.is_none() {
        let events = win.events.clone();
        let canvas = win.canvas.clone();
        let parent = win.canvas_parent.clone();
        let document = win.document.clone();
        let last_size = win.fullscreen_last_size.clone();
        win.fullscreen_callback_ref = Some(window_add_event_listener(
            "fullscreenchange",
            move |e: WebEvent| {
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

                canvas.set_width(width as _);
                canvas.set_height(height as _);
                events
                    .borrow_mut()
                    .push(Event::WindowResize { width, height });
            },
        )?);
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
    let min_size = win.min_size.clone();
    let max_size = win.max_size.clone();
    win.resize_callback_ref = Some(window_add_event_listener("resize", move |e: WebEvent| {
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

        canvas.set_width(p_width as _);
        canvas.set_height(p_height as _);
        events.borrow_mut().push(Event::WindowResize {
            width: p_width,
            height: p_height,
        });
    })?);
    Ok(())
}
