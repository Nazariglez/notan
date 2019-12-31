use super::System;
use nae_core::window::*;
use nae_core::{log, BaseApp};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub struct Window {
    pub(crate) canvas: HtmlCanvasElement,
    title: String,
    width: i32,
    height: i32,
    fullscreen: bool,
}

impl Window {
    pub(crate) fn new(title: &str, width: i32, height: i32) -> Result<Self, String> {
        let win = web_sys::window().ok_or(String::from("Can't access window dom object."))?;
        let canvas = win
            .document()
            .ok_or("Can't access document dom object ")?
            .get_element_by_id("nae_canvas")
            .ok_or("Can't get the element HtmlCanvasElement#nae_canvas")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|e| e.to_string())?;

        Ok(Self {
            title: title.to_string(),
            canvas,
            width,
            height,
            fullscreen: false,
        })
    }
}

impl BaseWindow for Window {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn dpi(&self) -> f32 {
        1.0
    }
}

fn request_animation_frame(win: web_sys::Window, f: &Closure<dyn FnMut()>) {
    win.request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn run<A, F>(app: A, callback: F)
where
    A: BaseApp<System = System> + 'static,
    F: FnMut(&mut A) + 'static,
{
    let mut app = app;
    let cb = Rc::new(RefCell::new(None));
    let cb_copy = cb.clone();
    let callback = Rc::new(RefCell::new(callback));

    *cb_copy.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut tick_handler = callback.borrow_mut();
        (&mut *tick_handler)(&mut app);

        //Web always run at max speed using raf (setTimeout has drawbacks)
        let win = web_sys::window().unwrap();
        request_animation_frame(win, cb.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    let win = web_sys::window().unwrap();
    request_animation_frame(win, cb_copy.borrow().as_ref().unwrap());
}
