use crate::utils::{
    canvas_add_event_listener, canvas_position_from_global, window_add_event_listener,
};
use crate::window::WebWindowBackend;
use notan_app::mouse::MouseButton;
use notan_app::Event;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Event as WebEvent, MouseEvent, WheelEvent};

#[derive(Default)]
pub struct MouseCallbacks {
    on_move: Option<Closure<FnMut(MouseEvent)>>,
    on_down: Option<Closure<FnMut(MouseEvent)>>,
    on_up: Option<Closure<FnMut(MouseEvent)>>,
    on_left_window: Option<Closure<FnMut(MouseEvent)>>,
    on_enter_window: Option<Closure<FnMut(MouseEvent)>>,
    on_wheel: Option<Closure<FnMut(WheelEvent)>>,
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
    let callbacks = &mut win.mouse_callbacks;

    let events = win.events.clone();
    let canvas = win.canvas.clone();
    callbacks.on_move = Some(canvas_add_event_listener(
        &canvas.clone(),
        "mousemove",
        move |e: MouseEvent| {
            let (x, y) = canvas_position_from_global(&canvas, e);
            events.borrow_mut().push(Event::MouseMove { x, y });
        },
    )?);

    let events = win.events.clone();
    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_down = Some(canvas_add_event_listener(
        &canvas.clone(),
        "mousedown",
        move |e: MouseEvent| {
            (*fullscreen.borrow())();
            let button = mouse_button_to_nae(e.button());
            let (x, y) = canvas_position_from_global(&canvas, e);
            events.borrow_mut().push(Event::MouseDown { button, x, y });
        },
    )?);

    let events = win.events.clone();
    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_up = Some(window_add_event_listener(
        "mouseup",
        move |e: MouseEvent| {
            (*fullscreen.borrow())();
            let button = mouse_button_to_nae(e.button());
            let (x, y) = canvas_position_from_global(&canvas, e);
            events.borrow_mut().push(Event::MouseUp { button, x, y });
        },
    )?);

    let events = win.events.clone();
    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_left_window = Some(canvas_add_event_listener(
        &canvas.clone(),
        "mouseout",
        move |e: MouseEvent| {
            (*fullscreen.borrow())();
            let (x, y) = canvas_position_from_global(&canvas, e);
            events.borrow_mut().push(Event::MouseLeft { x, y });
        },
    )?);

    let events = win.events.clone();
    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_enter_window = Some(canvas_add_event_listener(
        &canvas.clone(),
        "mouseover",
        move |e: MouseEvent| {
            (*fullscreen.borrow())();
            let (x, y) = canvas_position_from_global(&canvas, e);
            events.borrow_mut().push(Event::MouseEnter { x, y });
        },
    )?);

    let events = win.events.clone();
    let canvas = win.canvas.clone();
    callbacks.on_wheel = Some(canvas_add_event_listener(
        &canvas.clone(),
        "wheel",
        move |e: WheelEvent| {
            let delta_x = e.delta_x() as _;
            let delta_y = e.delta_y() as _;
            events
                .borrow_mut()
                .push(Event::MouseWheel { delta_x, delta_y });
        },
    )?);

    Ok(())
}
