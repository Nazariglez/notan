use crate::utils::{canvas_add_event_listener, canvas_position_from_global};
use crate::window::WebWindowBackend;
use notan_app::mouse::MouseButton;
use notan_app::Event;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Event as WebEvent, MouseEvent};

#[derive(Default)]
pub struct MouseContext {
    move_callback_ref: Option<Closure<FnMut(MouseEvent)>>,
    down_callback_ref: Option<Closure<FnMut(MouseEvent)>>,
    up_callback_ref: Option<Closure<FnMut(MouseEvent)>>,
    left_callback_ref: Option<Closure<FnMut(MouseEvent)>>,
    enter_callback_ref: Option<Closure<FnMut(MouseEvent)>>,
    wheel_callback_ref: Option<Closure<FnMut(MouseEvent)>>,
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
    let ctx = &mut win.mouse_context;

    let events = win.events.clone();
    let canvas = win.canvas.clone();
    ctx.move_callback_ref = Some(canvas_add_event_listener(
        &canvas.clone(),
        "mousemove",
        move |e: MouseEvent| {
            let (x, y) = canvas_position_from_global(&canvas, e);
            events.borrow_mut().push(Event::MouseMove { x, y });
        },
    )?);

    let events = win.events.clone();
    let canvas = win.canvas.clone();
    ctx.down_callback_ref = Some(canvas_add_event_listener(
        &canvas.clone(),
        "mousedown",
        move |e: MouseEvent| {
            (*fullscreen_dispatcher.borrow())();
            let button = mouse_button_to_nae(e.button());
            let (x, y) = canvas_position_from_global(&canvas, e);
            events.borrow_mut().push(Event::MouseDown { button, x, y });
        },
    )?);

    //TODO

    Ok(())
}
