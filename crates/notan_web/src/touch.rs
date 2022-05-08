use crate::utils::{canvas_add_event_listener, canvas_position_from_touch};
use crate::window::WebWindowBackend;
use notan_core::events::Event;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::PointerEvent;

#[derive(Default)]
pub struct PointerCallbacks {
    on_start: Option<Closure<dyn FnMut(PointerEvent)>>,
    on_end: Option<Closure<dyn FnMut(PointerEvent)>>,
    on_move: Option<Closure<dyn FnMut(PointerEvent)>>,
    on_cancel: Option<Closure<dyn FnMut(PointerEvent)>>,
}

pub fn enable_touch(
    win: &mut WebWindowBackend,
    fullscreen_dispatcher: Rc<RefCell<dyn Fn()>>,
) -> Result<(), String> {
    // we need to clone here to avoid a borrow checker issue
    let add_evt_move = win.add_event_fn();
    let add_evt_start = win.add_event_fn();
    let add_evt_end = win.add_event_fn();
    let add_evt_cancel = win.add_event_fn();

    let callbacks = &mut win.touch_callbacks;
    let canvas = win.canvas.clone();

    callbacks.on_move = Some(canvas_add_event_listener(
        &win.canvas,
        "pointermove",
        move |e: PointerEvent| {
            if e.pointer_type() == "touch" {
                e.stop_propagation();
                e.prevent_default();
                let id = e.pointer_id() as _;
                let (x, y) = canvas_position_from_touch(&canvas, e);
                add_evt_move(Event::TouchMove { id, x, y });
            }
        },
    )?);

    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_start = Some(canvas_add_event_listener(
        &win.canvas,
        "pointerdown",
        move |e: PointerEvent| {
            (*fullscreen.borrow())();
            e.stop_propagation();
            e.prevent_default();
            let id = e.pointer_id() as _;
            let (x, y) = canvas_position_from_touch(&canvas, e);
            add_evt_start(Event::TouchStart { id, x, y });
        },
    )?);

    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_end = Some(canvas_add_event_listener(
        &win.canvas,
        "pointerup",
        move |e: PointerEvent| {
            (*fullscreen.borrow())();
            e.stop_propagation();
            e.prevent_default();
            let id = e.pointer_id() as _;
            let (x, y) = canvas_position_from_touch(&canvas, e);
            add_evt_end(Event::TouchEnd { id, x, y });
        },
    )?);

    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_cancel = Some(canvas_add_event_listener(
        &win.canvas,
        "pointercancel",
        move |e: PointerEvent| {
            (*fullscreen.borrow())();
            e.stop_propagation();
            e.prevent_default();
            let id = e.pointer_id() as _;
            let (x, y) = canvas_position_from_touch(&canvas, e);
            add_evt_cancel(Event::TouchCancel { id, x, y });
        },
    )?);

    Ok(())
}
