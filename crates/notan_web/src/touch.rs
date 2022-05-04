use crate::utils::{canvas_add_event_listener, canvas_position_from_global, canvas_position_from_touch, window_add_event_listener};
use crate::window::WebWindowBackend;
use notan_core::events::Event;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{PointerEvent};

#[derive(Default)]
pub struct TouchCallbacks {
    on_start: Option<Closure<dyn FnMut(TouchEvent)>>,
    on_end: Option<Closure<dyn FnMut(TouchEvent)>>,
    on_move: Option<Closure<dyn FnMut(TouchEvent)>>,
    on_cancelled: Option<Closure<dyn FnMut(TouchEvent)>>,
}

pub fn enable_touch(
    win: &mut WebWindowBackend,
    fullscreen_dispatcher: Rc<RefCell<dyn Fn()>>,
) -> Result<(), String> {
    // we need to clone here to avoid a borrow checker issue
    let add_evt_move = win.add_event_fn();
    let add_evt_start = win.add_event_fn();
    let add_evt_end = win.add_event_fn();
    let add_evt_cancelled = win.add_event_fn();

    let callbacks = &mut win.touch_callbacks;
    let canvas = win.canvas.clone();

    callbacks.on_move = Some(canvas_add_event_listener(
        &win.canvas,
        "pointermove",
        move |e: PointerEvent| {
            if e.pointer_type() == "touch" {
                let id = e.pointer_id();
                let (x, y) = canvas_position_from_touch(&canvas, e);
                add_evt_move(Event::TouchMove { id, x, y });
            }
        },
    )?);

    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_down = Some(canvas_add_event_listener(
        &win.canvas,
        "mousedown",
        move |e: MouseEvent| {
            (*fullscreen.borrow())();
            let button = mouse_button_to_nae(e.button());
            let (x, y) = canvas_position_from_global(&canvas, e);
            add_evt_down(Event::MouseDown { button, x, y });
        },
    )?);

    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_up = Some(window_add_event_listener(
        "mouseup",
        move |e: MouseEvent| {
            (*fullscreen.borrow())();
            let button = mouse_button_to_nae(e.button());
            let (x, y) = canvas_position_from_global(&canvas, e);
            add_evt_up(Event::MouseUp { button, x, y });
        },
    )?);

    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_left_window = Some(canvas_add_event_listener(
        &win.canvas,
        "mouseout",
        move |e: MouseEvent| {
            (*fullscreen.borrow())();
            let (x, y) = canvas_position_from_global(&canvas, e);
            add_evt_left_window(Event::MouseLeft { x, y });
        },
    )?);

    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_enter_window = Some(canvas_add_event_listener(
        &win.canvas,
        "mouseover",
        move |e: MouseEvent| {
            (*fullscreen.borrow())();
            let (x, y) = canvas_position_from_global(&canvas, e);
            add_evt_over(Event::MouseEnter { x, y });
        },
    )?);

    callbacks.on_wheel = Some(canvas_add_event_listener(
        &win.canvas,
        "wheel",
        move |e: WheelEvent| {
            let delta_x = e.delta_x() as _;
            let delta_y = e.delta_y() as _;
            add_evt_wheel(Event::MouseWheel { delta_x, delta_y });
        },
    )?);

    Ok(())
}
