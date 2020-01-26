use super::System;
use nae_core::window::*;
use nae_core::{log, BaseApp, BuilderOpts, Event, KeyCode, MouseButton};
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

fn get_or_create_canvas(win: &web_sys::Window) -> Result<HtmlCanvasElement, String> {
    let doc = win.document().ok_or("Can't access document dom object ")?;
    let canvas = match doc.get_element_by_id("nae_canvas") {
        Some(c) => c,
        None => {
            let c = doc
                .create_element("canvas")
                .map_err(|e| format!("{:?}", e))?;

            let body = doc
                .body()
                .ok_or("body doesn't exists on document.".to_string())?;
            body.append_child(&c).map_err(|e| format!("{:?}", e))?;

            c.set_id("nae_canvas");
            c
        }
    };
    canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|e| format!("{:?}", e))
}

impl Window {
    pub(crate) fn new(opts: &BuilderOpts) -> Result<Self, String> {
        let win = web_sys::window().ok_or(String::from("Can't access window dom object."))?;
        let mut canvas = get_or_create_canvas(&win)?;
        canvas.set_width(opts.width as u32);
        canvas.set_height(opts.height as u32);

        let ctx_menu_cb =
            canvas_add_event_listener(&canvas, "contextmenu", |e: web_sys::Event| {
                e.prevent_default();
            })?;

        Ok(Self {
            title: opts.title.to_string(),
            canvas,
            width: opts.width,
            height: opts.height,
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

pub(crate) struct KeyboardContext {
    up_cb: Option<Closure<FnMut(web_sys::KeyboardEvent)>>,
    down_cb: Option<Closure<FnMut(web_sys::KeyboardEvent)>>,
}

impl KeyboardContext {
    fn new() -> Self {
        Self {
            up_cb: None,
            down_cb: None,
        }
    }
}

fn enable_keyboard_events(
    events: Rc<RefCell<VecDeque<Event>>>,
    canvas: &HtmlCanvasElement,
    ctx: &mut KeyboardContext,
) -> Result<(), String> {
    let events_copy = events.clone();
    let canvas_clone = canvas.clone();
    ctx.down_cb = Some(window_add_event_listener(
        "keydown",
        move |e: web_sys::KeyboardEvent| {
            let mut events = events_copy.borrow_mut();

            if let Some(k) = keyboard_code(&e.code()) {
                let down_evt = Event::KeyDown { key: k };
                events.push_back(down_evt);
            }

            let key_char = e.key();
            if key_char.len() <= 2 {
                //special keys like enter has "Enter" as key
                if let Some(c) = key_char.chars().next() {
                    events.push_back(Event::ReceivedCharacter(c));
                }
            }
        },
    )?);

    let events_copy = events.clone();
    let canvas_clone = canvas.clone();
    ctx.up_cb = Some(window_add_event_listener(
        "keyup",
        move |e: web_sys::KeyboardEvent| {
            let mut events = events_copy.borrow_mut();

            if let Some(k) = keyboard_code(&e.code()) {
                let down_evt = Event::KeyUp { key: k };
                events.push_back(down_evt);
            }
        },
    )?);

    Ok(())
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

    let mut keyboard_ctx = KeyboardContext::new();
    enable_keyboard_events(
        events.clone(),
        &app.system().window.canvas,
        &mut keyboard_ctx,
    );

    //Store the ref to the mouse context to avoid drop the closures, another option could be use forget but seems more clean.
    app.system().mouse_ctx = Some(mouse_ctx);
    app.system().keyboard_ctx = Some(keyboard_ctx);

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

//Code from winit
pub fn keyboard_code(code: &str) -> Option<KeyCode> {
    Some(match code {
        "Digit1" => KeyCode::Key1,
        "Digit2" => KeyCode::Key2,
        "Digit3" => KeyCode::Key3,
        "Digit4" => KeyCode::Key4,
        "Digit5" => KeyCode::Key5,
        "Digit6" => KeyCode::Key6,
        "Digit7" => KeyCode::Key7,
        "Digit8" => KeyCode::Key8,
        "Digit9" => KeyCode::Key9,
        "Digit0" => KeyCode::Key0,
        "KeyA" => KeyCode::A,
        "KeyB" => KeyCode::B,
        "KeyC" => KeyCode::C,
        "KeyD" => KeyCode::D,
        "KeyE" => KeyCode::E,
        "KeyF" => KeyCode::F,
        "KeyG" => KeyCode::G,
        "KeyH" => KeyCode::H,
        "KeyI" => KeyCode::I,
        "KeyJ" => KeyCode::J,
        "KeyK" => KeyCode::K,
        "KeyL" => KeyCode::L,
        "KeyM" => KeyCode::M,
        "KeyN" => KeyCode::N,
        "KeyO" => KeyCode::O,
        "KeyP" => KeyCode::P,
        "KeyQ" => KeyCode::Q,
        "KeyR" => KeyCode::R,
        "KeyS" => KeyCode::S,
        "KeyT" => KeyCode::T,
        "KeyU" => KeyCode::U,
        "KeyV" => KeyCode::V,
        "KeyW" => KeyCode::W,
        "KeyX" => KeyCode::X,
        "KeyY" => KeyCode::Y,
        "KeyZ" => KeyCode::Z,
        "Escape" => KeyCode::Escape,
        "F1" => KeyCode::F1,
        "F2" => KeyCode::F2,
        "F3" => KeyCode::F3,
        "F4" => KeyCode::F4,
        "F5" => KeyCode::F5,
        "F6" => KeyCode::F6,
        "F7" => KeyCode::F7,
        "F8" => KeyCode::F8,
        "F9" => KeyCode::F9,
        "F10" => KeyCode::F10,
        "F11" => KeyCode::F11,
        "F12" => KeyCode::F12,
        "F13" => KeyCode::F13,
        "F14" => KeyCode::F14,
        "F15" => KeyCode::F15,
        "F16" => KeyCode::F16,
        "F17" => KeyCode::F17,
        "F18" => KeyCode::F18,
        "F19" => KeyCode::F19,
        "F20" => KeyCode::F20,
        "F21" => KeyCode::F21,
        "F22" => KeyCode::F22,
        "F23" => KeyCode::F23,
        "F24" => KeyCode::F24,
        "PrintScreen" => KeyCode::Snapshot,
        "ScrollLock" => KeyCode::Scroll,
        "Pause" => KeyCode::Pause,
        "Insert" => KeyCode::Insert,
        "Home" => KeyCode::Home,
        "Delete" => KeyCode::Delete,
        "End" => KeyCode::End,
        "PageDown" => KeyCode::PageDown,
        "PageUp" => KeyCode::PageUp,
        "ArrowLeft" => KeyCode::Left,
        "ArrowUp" => KeyCode::Up,
        "ArrowRight" => KeyCode::Right,
        "ArrowDown" => KeyCode::Down,
        "Backspace" => KeyCode::Back,
        "Enter" => KeyCode::Return,
        "Space" => KeyCode::Space,
        "Compose" => KeyCode::Compose,
        "Caret" => KeyCode::Caret,
        "NumLock" => KeyCode::Numlock,
        "Numpad0" => KeyCode::Numpad0,
        "Numpad1" => KeyCode::Numpad1,
        "Numpad2" => KeyCode::Numpad2,
        "Numpad3" => KeyCode::Numpad3,
        "Numpad4" => KeyCode::Numpad4,
        "Numpad5" => KeyCode::Numpad5,
        "Numpad6" => KeyCode::Numpad6,
        "Numpad7" => KeyCode::Numpad7,
        "Numpad8" => KeyCode::Numpad8,
        "Numpad9" => KeyCode::Numpad9,
        "AbntC1" => KeyCode::AbntC1,
        "AbntC2" => KeyCode::AbntC2,
        "NumpadAdd" => KeyCode::Add,
        "Quote" => KeyCode::Apostrophe,
        "Apps" => KeyCode::Apps,
        "At" => KeyCode::At,
        "Ax" => KeyCode::Ax,
        "Backslash" => KeyCode::Backslash,
        "Calculator" => KeyCode::Calculator,
        "Capital" => KeyCode::Capital,
        "Semicolon" => KeyCode::Semicolon,
        "Comma" => KeyCode::Comma,
        "Convert" => KeyCode::Convert,
        "NumpadDecimal" => KeyCode::Decimal,
        "NumpadDivide" => KeyCode::Divide,
        "Equal" => KeyCode::Equals,
        "Backquote" => KeyCode::Grave,
        "Kana" => KeyCode::Kana,
        "Kanji" => KeyCode::Kanji,
        "AltLeft" => KeyCode::LAlt,
        "BracketLeft" => KeyCode::LBracket,
        "ControlLeft" => KeyCode::LControl,
        "ShiftLeft" => KeyCode::LShift,
        "MetaLeft" => KeyCode::LWin,
        "Mail" => KeyCode::Mail,
        "MediaSelect" => KeyCode::MediaSelect,
        "MediaStop" => KeyCode::MediaStop,
        "Minus" => KeyCode::Minus,
        "NumpadMultiply" => KeyCode::Multiply,
        "Mute" => KeyCode::Mute,
        "LaunchMyComputer" => KeyCode::MyComputer,
        "NavigateForward" => KeyCode::NavigateForward,
        "NavigateBackward" => KeyCode::NavigateBackward,
        "NextTrack" => KeyCode::NextTrack,
        "NoConvert" => KeyCode::NoConvert,
        "NumpadComma" => KeyCode::NumpadComma,
        "NumpadEnter" => KeyCode::NumpadEnter,
        "NumpadEquals" => KeyCode::NumpadEquals,
        "OEM102" => KeyCode::OEM102,
        "Period" => KeyCode::Period,
        "PlayPause" => KeyCode::PlayPause,
        "Power" => KeyCode::Power,
        "PrevTrack" => KeyCode::PrevTrack,
        "AltRight" => KeyCode::RAlt,
        "BracketRight" => KeyCode::RBracket,
        "ControlRight" => KeyCode::RControl,
        "ShiftRight" => KeyCode::RShift,
        "MetaRight" => KeyCode::RWin,
        "Slash" => KeyCode::Slash,
        "Sleep" => KeyCode::Sleep,
        "Stop" => KeyCode::Stop,
        "NumpadSubtract" => KeyCode::Subtract,
        "Sysrq" => KeyCode::Sysrq,
        "Tab" => KeyCode::Tab,
        "Underline" => KeyCode::Underline,
        "Unlabeled" => KeyCode::Unlabeled,
        "AudioVolumeDown" => KeyCode::VolumeDown,
        "AudioVolumeUp" => KeyCode::VolumeUp,
        "Wake" => KeyCode::Wake,
        "WebBack" => KeyCode::WebBack,
        "WebFavorites" => KeyCode::WebFavorites,
        "WebForward" => KeyCode::WebForward,
        "WebHome" => KeyCode::WebHome,
        "WebRefresh" => KeyCode::WebRefresh,
        "WebSearch" => KeyCode::WebSearch,
        "WebStop" => KeyCode::WebStop,
        "Yen" => KeyCode::Yen,
        _ => return None,
    })
}
