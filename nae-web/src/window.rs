use super::System;
use nae_core::window::*;
use nae_core::{log, BaseApp, Event, MouseButton};
use std::cell::RefCell;
use std::collections::VecDeque;
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
    ctx_menu_cb: Closure<FnMut(web_sys::Event)>,
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

        let ctx_menu_cb =
            canvas_add_event_listener(&canvas, "contextmenu", |e: web_sys::Event| {
                e.prevent_default();
            })?;

        Ok(Self {
            title: title.to_string(),
            canvas,
            width,
            height,
            fullscreen: false,
            ctx_menu_cb,
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

pub(crate) struct MouseContext {
    up_cb: Option<Closure<FnMut(web_sys::MouseEvent)>>,
    down_cb: Option<Closure<FnMut(web_sys::MouseEvent)>>,
    move_cb: Option<Closure<FnMut(web_sys::MouseEvent)>>,
    left_cb: Option<Closure<FnMut(web_sys::MouseEvent)>>,
    enter_cb: Option<Closure<FnMut(web_sys::MouseEvent)>>,
    wheel_cb: Option<Closure<FnMut(web_sys::WheelEvent)>>,
}

impl MouseContext {
    fn new() -> Self {
        Self {
            up_cb: None,
            down_cb: None,
            move_cb: None,
            left_cb: None,
            enter_cb: None,
            wheel_cb: None,
        }
    }
}

fn enable_mouse_events(
    events: Rc<RefCell<VecDeque<Event>>>,
    canvas: &HtmlCanvasElement,
    ctx: &mut MouseContext,
) -> Result<(), String> {
    let events_copy = events.clone();
    let canvas_clone = canvas.clone();
    ctx.move_cb = Some(canvas_add_event_listener(canvas, "mousemove", move |e| {
        let (x, y) = canvas_position_from_global(&canvas_clone, e);
        events_copy
            .borrow_mut()
            .push_back(Event::MouseMove { x, y });
    })?);

    let events_copy = events.clone();
    let canvas_clone = canvas.clone();
    ctx.down_cb = Some(canvas_add_event_listener(
        canvas,
        "mousedown",
        move |e: web_sys::MouseEvent| {
            let btn = mouse_button_to_nae(e.button());
            let (x, y) = canvas_position_from_global(&canvas_clone, e);
            events_copy
                .borrow_mut()
                .push_back(Event::MouseDown { button: btn, x, y });
        },
    )?);

    let events_copy = events.clone();
    let canvas_clone = canvas.clone();
    ctx.up_cb = Some(window_add_event_listener(
        "mouseup",
        move |e: web_sys::MouseEvent| {
            let btn = mouse_button_to_nae(e.button());
            let (x, y) = canvas_position_from_global(&canvas_clone, e);
            events_copy
                .borrow_mut()
                .push_back(Event::MouseUp { button: btn, x, y });
        },
    )?);

    let events_copy = events.clone();
    let canvas_clone = canvas.clone();
    ctx.left_cb = Some(canvas_add_event_listener(
        canvas,
        "mouseout",
        move |e: web_sys::MouseEvent| {
            let (x, y) = canvas_position_from_global(&canvas_clone, e);
            events_copy
                .borrow_mut()
                .push_back(Event::MouseLeft { x, y });
        },
    )?);

    let events_copy = events.clone();
    let canvas_clone = canvas.clone();
    ctx.enter_cb = Some(canvas_add_event_listener(
        canvas,
        "mouseover",
        move |e: web_sys::MouseEvent| {
            let (x, y) = canvas_position_from_global(&canvas_clone, e);
            events_copy
                .borrow_mut()
                .push_back(Event::MouseEnter { x, y });
        },
    )?);

    let events_copy = events.clone();
    let canvas_clone = canvas.clone();
    ctx.wheel_cb = Some(canvas_add_event_listener(
        canvas,
        "wheel",
        move |e: web_sys::WheelEvent| {
            events_copy.borrow_mut().push_back(Event::MouseWheel {
                delta_x: e.delta_x() as _,
                delta_y: e.delta_y() as _,
            });
        },
    )?);

    Ok(())
}

fn mouse_button_to_nae(btn: i16) -> MouseButton {
    match btn {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        n => MouseButton::Other(n as u8),
    }
}

pub fn run<A, S, F, D>(mut app: A, mut state: S, mut update: F, mut draw: D)
where
    A: BaseApp<System = System> + 'static,
    S: 'static,
    F: FnMut(&mut A, &mut S) + 'static,
    D: FnMut(&mut A, &mut S) + 'static,
{
    let cb = Rc::new(RefCell::new(None));
    let cb_copy = cb.clone();

    let events = Rc::new(RefCell::new(VecDeque::new()));
    let mut mouse_ctx = MouseContext::new();
    enable_mouse_events(events.clone(), &app.system().window.canvas, &mut mouse_ctx);

    //Store the ref to the mouse context to avoid drop the closures, another option could be use forget but seems more clean.
    app.system().mouse_ctx = Some(mouse_ctx);

    let callback = Rc::new(RefCell::new(move |app: &mut A, state: &mut S| {
        let mut frame_evts = events.borrow_mut();
        while let Some(evt) = frame_evts.pop_front() {
            app.system().events.push(evt);
        }

        update(app, state);
        draw(app, state);
    }));

    *cb_copy.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut tick_handler = callback.borrow_mut();
        (&mut *tick_handler)(&mut app, &mut state);

        //Web always run at max speed using raf (setTimeout has drawbacks)
        let win = web_sys::window().unwrap();
        request_animation_frame(win, cb.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    let win = web_sys::window().unwrap();
    request_animation_frame(win, cb_copy.borrow().as_ref().unwrap());
}

fn canvas_add_event_listener<F, E>(
    canvas: &HtmlCanvasElement,
    name: &str,
    handler: F,
) -> Result<Closure<FnMut(E)>, String>
where
    E: wasm_bindgen::convert::FromWasmAbi + 'static,
    F: FnMut(E) + 'static,
{
    let mut handler = handler;
    let closure = Closure::wrap(Box::new(move |e: E| {
        handler(e);
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
        .map_err(|e| format!("Invalid event name: {}", name))?;
    Ok(closure)
}

fn window_add_event_listener<F, E>(name: &str, handler: F) -> Result<Closure<FnMut(E)>, String>
where
    E: wasm_bindgen::convert::FromWasmAbi + 'static,
    F: FnMut(E) + 'static,
{
    let win = web_sys::window().ok_or("global window doesn't exists".to_string())?;

    let mut handler = handler;
    let closure = Closure::wrap(Box::new(move |e: E| {
        handler(e);
    }) as Box<dyn FnMut(_)>);

    win.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
        .map_err(|_| format!("Invalid event name: {}", name))?;
    Ok(closure)
}

fn canvas_position_from_global(canvas: &HtmlCanvasElement, evt: web_sys::MouseEvent) -> (i32, i32) {
    let client_x = (1 + evt.x()) as f64;
    let client_y = (1 + evt.y()) as f64;
    let rect = canvas.get_bounding_client_rect();
    let x = (client_x - rect.left()) / (rect.right() - rect.left()) * (canvas.width() as f64);
    let y = (client_y - rect.top()) / (rect.bottom() - rect.top()) * (canvas.height() as f64);
    (x as i32, y as i32)
}
