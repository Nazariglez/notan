use crate::utils::{
    canvas_add_event_listener, canvas_position_from_global, document_add_event_listener,
    window_add_event_listener,
};
use crate::window::WebWindowBackend;
use notan_core::events::Event;
use notan_core::mouse::MouseButton;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, MouseEvent, WheelEvent};

#[derive(Default)]
pub struct MouseCallbacks {
    on_move: Option<Closure<dyn FnMut(MouseEvent)>>,
    on_down: Option<Closure<dyn FnMut(MouseEvent)>>,
    on_up: Option<Closure<dyn FnMut(MouseEvent)>>,
    on_left_window: Option<Closure<dyn FnMut(MouseEvent)>>,
    on_enter_window: Option<Closure<dyn FnMut(MouseEvent)>>,
    on_wheel: Option<Closure<dyn FnMut(WheelEvent)>>,
    on_pointer_lock_change: Option<Closure<dyn FnMut(web_sys::Event)>>,
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

    let last_x_ref = Rc::new(RefCell::new(0));
    let last_y_ref = Rc::new(RefCell::new(0));

    let callbacks = &mut win.mouse_callbacks;
    let canvas = win.canvas.clone();

    let captured = win.captured.clone();
    let last_x = last_x_ref.clone();
    let last_y = last_y_ref.clone();
    callbacks.on_move = Some(canvas_add_event_listener(
        &win.canvas,
        "mousemove",
        move |e: MouseEvent| {
            e.stop_propagation();
            e.prevent_default();
            let (x, y) = get_x_y(
                &canvas,
                e,
                *captured.borrow(),
                &mut last_x.borrow_mut(),
                &mut last_y.borrow_mut(),
            );
            add_evt_move(Event::MouseMove { x, y });
        },
    )?);

    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    let captured = win.captured.clone();
    let last_x = last_x_ref.clone();
    let last_y = last_y_ref.clone();
    callbacks.on_down = Some(canvas_add_event_listener(
        &win.canvas,
        "mousedown",
        move |e: MouseEvent| {
            (*fullscreen.borrow())();
            e.stop_propagation();
            e.prevent_default();
            let button = mouse_button_to_nae(e.button());
            let (x, y) = get_x_y(
                &canvas,
                e,
                *captured.borrow(),
                &mut last_x.borrow_mut(),
                &mut last_y.borrow_mut(),
            );
            add_evt_down(Event::MouseDown { button, x, y });
        },
    )?);

    let canvas = win.canvas.clone();
    let fullscreen = fullscreen_dispatcher.clone();
    let captured = win.captured.clone();
    let last_x = last_x_ref;
    let last_y = last_y_ref;
    callbacks.on_up = Some(window_add_event_listener(
        "mouseup",
        move |e: MouseEvent| {
            (*fullscreen.borrow())();
            e.stop_propagation();
            e.prevent_default();
            let button = mouse_button_to_nae(e.button());
            let (x, y) = get_x_y(
                &canvas,
                e,
                *captured.borrow(),
                &mut last_x.borrow_mut(),
                &mut last_y.borrow_mut(),
            );
            add_evt_up(Event::MouseUp { button, x, y });
        },
    )?);

    let canvas = win.canvas.clone();
    callbacks.on_left_window = Some(canvas_add_event_listener(
        &win.canvas,
        "mouseout",
        move |e: MouseEvent| {
            e.stop_propagation();
            e.prevent_default();
            let (x, y) = canvas_position_from_global(&canvas, e);
            add_evt_left_window(Event::MouseLeft { x, y });
        },
    )?);

    let canvas = win.canvas.clone();
    callbacks.on_enter_window = Some(canvas_add_event_listener(
        &win.canvas,
        "mouseover",
        move |e: MouseEvent| {
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
            let delta_x = -e.delta_x() as _;
            let delta_y = -e.delta_y() as _;
            add_evt_wheel(Event::MouseWheel { delta_x, delta_y });
            e.stop_propagation();
            e.prevent_default();
        },
    )?);

    let captured = win.captured.clone();
    let doc = win.document.clone();
    let canvas = win.canvas.clone();
    callbacks.on_pointer_lock_change = Some(document_add_event_listener(
        "pointerlockchange",
        move |_| match doc.pointer_lock_element() {
            Some(el) => {
                *captured.borrow_mut() = el.id() == canvas.id();
            }
            _ => {
                *captured.borrow_mut() = false;
            }
        },
    )?);

    Ok(())
}

fn get_x_y(
    canvas: &HtmlCanvasElement,
    e: MouseEvent,
    captured: bool,
    last_x: &mut i32,
    last_y: &mut i32,
) -> (i32, i32) {
    let (x, y) = if captured {
        (*last_x + e.movement_x(), *last_y + e.movement_y())
    } else {
        canvas_position_from_global(canvas, e)
    };
    *last_x = x;
    *last_y = y;
    (x, y)
}
