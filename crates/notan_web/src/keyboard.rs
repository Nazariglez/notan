use crate::utils::window_add_event_listener;
use crate::window::WebWindowBackend;
use notan_core::events::Event;
use notan_core::keyboard::KeyCode;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

#[derive(Default)]
pub struct KeyboardCallbacks {
    on_up: Option<Closure<dyn FnMut(KeyboardEvent)>>,
    on_down: Option<Closure<dyn FnMut(KeyboardEvent)>>,
}

pub fn enable_keyboard(
    win: &mut WebWindowBackend,
    fullscreen_dispatcher: Rc<RefCell<dyn Fn()>>,
) -> Result<(), String> {
    let add_evt_down = win.add_event_fn();
    let add_evt_up = win.add_event_fn();
    let callbacks = &mut win.keyboard_callbacks;
    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_down = Some(window_add_event_listener(
        "keydown",
        move |e: KeyboardEvent| {
            (*fullscreen.borrow_mut())();
            if let Some(key) = keyboard_code(&e.code()) {
                add_evt_down(Event::KeyDown { key });
            }

            let char = e.key();
            if char.len() <= 2 {
                if let Some(char) = char.chars().next() {
                    add_evt_down(Event::ReceivedCharacter(char));
                }
            }
        },
    )?);

    let fullscreen = fullscreen_dispatcher.clone();
    callbacks.on_up = Some(window_add_event_listener(
        "keyup",
        move |e: KeyboardEvent| {
            (*fullscreen.borrow_mut())();
            if let Some(key) = keyboard_code(&e.code()) {
                add_evt_up(Event::KeyUp { key });
            }
        },
    )?);

    Ok(())
}

// Code from winit
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
