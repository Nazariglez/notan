use crate::utils::{
    canvas_add_event_listener, canvas_position_from_global, window_add_event_listener,
};
use crate::window::WebWindowBackend;
use notan_core::events::Event;
use notan_core::mouse::MouseButton;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{MouseEvent, WheelEvent};

#[derive(Default)]
pub struct MouseCallbacks {
    on_move: Option<Closure<dyn FnMut(MouseEvent)>>,
    on_down: Option<Closure<dyn FnMut(MouseEvent)>>,
    on_up: Option<Closure<dyn FnMut(MouseEvent)>>,
    on_left_window: Option<Closure<dyn FnMut(MouseEvent)>>,
    on_enter_window: Option<Closure<dyn FnMut(MouseEvent)>>,
    on_wheel: Option<Closure<dyn FnMut(WheelEvent)>>,
}

fn mouse_button_to_nae(btn: i16) -> MouseButton {
    match btn {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        n => MouseButton::Other(n as u8),
    }
}

pub fn enable_mouse(
    win: &mut WebWindowBackend,
    fullscreen_dispatcher: Rc<RefCell<dyn Fn()>>,
) -> Result<(), String> {
    // we need to clone here to avoid a borrow checker issue
    let add_evt_move = win.add_event_fn();
    let add_evt_down = win.add_event_fn();
    let add_evt_up = win.add_event_fn();
    let add_evt_left_window = win.add_event_fn();
    let add_evt_over = win.add_event_fn();
    let add_evt_wheel = win.add_event_fn();

    let callbacks = &mut win.mouse_callbacks;
    let canvas = win.canvas.clone();

    callbacks.on_move = Some(canvas_add_event_listener(
        &win.canvas,
        "mousemove",
        move |e: MouseEvent| {
            e.stop_propagation();
            e.prevent_default();
            let (x, y) = canvas_position_from_global(&canvas, e);
            add_evt_move(Event::MouseMove { x, y });
        },
    )?);

    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_down = Some(canvas_add_event_listener(
        &win.canvas,
        "mousedown",
        move |e: MouseEvent| {
            (*fullscreen.borrow())();
            e.stop_propagation();
            e.prevent_default();
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
            e.stop_propagation();
            e.prevent_default();
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
            e.stop_propagation();
            e.prevent_default();
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
            e.stop_propagation();
            e.prevent_default();
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
            e.stop_propagation();
            e.prevent_default();
        },
    )?);

    Ok(())
}
