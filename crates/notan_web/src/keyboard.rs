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
        "Again" => KeyCode::Again,
        "AltLeft" => KeyCode::AltLeft,
        "AltRight" => KeyCode::AltRight,
        "ArrowDown" => KeyCode::ArrowDown,
        "ArrowLeft" => KeyCode::ArrowLeft,
        "ArrowRight" => KeyCode::ArrowRight,
        "ArrowUp" => KeyCode::ArrowUp,
        "AudioVolumeDown" => KeyCode::AudioVolumeDown,
        "AudioVolumeMute" => KeyCode::AudioVolumeMute,
        "AudioVolumeUp" => KeyCode::AudioVolumeUp,
        "Backquote" => KeyCode::Backquote,
        "Backslash" => KeyCode::Backslash,
        "Backspace" => KeyCode::Backspace,
        "BracketLeft" => KeyCode::BracketLeft,
        "BracketRight" => KeyCode::BracketRight,
        "BrowserBack" => KeyCode::BrowserBack,
        "BrowserFavorites" => KeyCode::BrowserFavorites,
        "BrowserForward" => KeyCode::BrowserForward,
        "BrowserHome" => KeyCode::BrowserHome,
        "BrowserRefresh" => KeyCode::BrowserRefresh,
        "BrowserSearch" => KeyCode::BrowserSearch,
        "BrowserStop" => KeyCode::BrowserStop,
        "CapsLock" => KeyCode::CapsLock,
        "Comma" => KeyCode::Comma,
        "ContextMenu" => KeyCode::ContextMenu,
        "ControlLeft" => KeyCode::ControlLeft,
        "ControlRight" => KeyCode::ControlRight,
        "Convert" => KeyCode::Convert,
        "Copy" => KeyCode::Copy,
        "Cut" => KeyCode::Cut,
        "Delete" => KeyCode::Delete,
        "Digit0" => KeyCode::Digit0,
        "Digit1" => KeyCode::Digit1,
        "Digit2" => KeyCode::Digit2,
        "Digit3" => KeyCode::Digit3,
        "Digit4" => KeyCode::Digit4,
        "Digit5" => KeyCode::Digit5,
        "Digit6" => KeyCode::Digit6,
        "Digit7" => KeyCode::Digit7,
        "Digit8" => KeyCode::Digit8,
        "Digit9" => KeyCode::Digit9,
        "Eject" => KeyCode::Eject,
        "End" => KeyCode::End,
        "Enter" => KeyCode::Enter,
        "Equal" => KeyCode::Equal,
        "Escape" => KeyCode::Escape,
        "F1" => KeyCode::F1,
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
        "F2" => KeyCode::F2,
        "F20" => KeyCode::F20,
        "F21" => KeyCode::F21,
        "F22" => KeyCode::F22,
        "F23" => KeyCode::F23,
        "F24" => KeyCode::F24,
        "F3" => KeyCode::F3,
        "F4" => KeyCode::F4,
        "F5" => KeyCode::F5,
        "F6" => KeyCode::F6,
        "F7" => KeyCode::F7,
        "F8" => KeyCode::F8,
        "F9" => KeyCode::F9,
        "Find" => KeyCode::Find,
        "Fn" => KeyCode::Fn,
        "Help" => KeyCode::Help,
        "Home" => KeyCode::Home,
        "Insert" => KeyCode::Insert,
        "IntlBackslash" => KeyCode::IntlBackslash,
        "IntlRo" => KeyCode::IntlRo,
        "IntlYen" => KeyCode::IntlYen,
        "KanaMode" => KeyCode::KanaMode,
        "KeyA" => KeyCode::KeyA,
        "KeyB" => KeyCode::KeyB,
        "KeyC" => KeyCode::KeyC,
        "KeyD" => KeyCode::KeyD,
        "KeyE" => KeyCode::KeyE,
        "KeyF" => KeyCode::KeyF,
        "KeyG" => KeyCode::KeyG,
        "KeyH" => KeyCode::KeyH,
        "KeyI" => KeyCode::KeyI,
        "KeyJ" => KeyCode::KeyJ,
        "KeyK" => KeyCode::KeyK,
        "KeyL" => KeyCode::KeyL,
        "KeyM" => KeyCode::KeyM,
        "KeyN" => KeyCode::KeyN,
        "KeyO" => KeyCode::KeyO,
        "KeyP" => KeyCode::KeyP,
        "KeyQ" => KeyCode::KeyQ,
        "KeyR" => KeyCode::KeyR,
        "KeyS" => KeyCode::KeyS,
        "KeyT" => KeyCode::KeyT,
        "KeyU" => KeyCode::KeyU,
        "KeyV" => KeyCode::KeyV,
        "KeyW" => KeyCode::KeyW,
        "KeyX" => KeyCode::KeyX,
        "KeyY" => KeyCode::KeyY,
        "KeyZ" => KeyCode::KeyZ,
        "Lang1" => KeyCode::Lang1,
        "Lang2" => KeyCode::Lang2,
        "Lang3" => KeyCode::Lang3,
        "Lang4" => KeyCode::Lang4,
        "Lang5" => KeyCode::Lang5,
        "LaunchApp1" => KeyCode::LaunchApp1,
        "LaunchApp2" => KeyCode::LaunchApp2,
        "LaunchMail" => KeyCode::LaunchMail,
        "MediaPlayPause" => KeyCode::MediaPlayPause,
        "MediaSelect" => KeyCode::MediaSelect,
        "MediaStop" => KeyCode::MediaStop,
        "MediaTrackNext" => KeyCode::MediaTrackNext,
        "MediaTrackPrevious" => KeyCode::MediaTrackPrevious,
        "MetaLeft" => KeyCode::Meta,
        "MetaRight" => KeyCode::Meta,
        "Minus" => KeyCode::Minus,
        "NonConvert" => KeyCode::NonConvert,
        "NumLock" => KeyCode::NumLock,
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
        "NumpadAdd" => KeyCode::NumpadAdd,
        "NumpadComma" => KeyCode::NumpadComma,
        "NumpadDecimal" => KeyCode::NumpadDecimal,
        "NumpadDivide" => KeyCode::NumpadDivide,
        "NumpadEnter" => KeyCode::NumpadEnter,
        "NumpadEqual" => KeyCode::NumpadEqual,
        "NumpadMultiply" => KeyCode::NumpadMultiply,
        "NumpadParenLeft" => KeyCode::NumpadParenLeft,
        "NumpadParenRight" => KeyCode::NumpadParenRight,
        "NumpadSubtract" => KeyCode::NumpadSubtract,
        "Open" => KeyCode::Open,
        "PageDown" => KeyCode::PageDown,
        "PageUp" => KeyCode::PageUp,
        "Paste" => KeyCode::Paste,
        "Pause" => KeyCode::Pause,
        "Period" => KeyCode::Period,
        "Power" => KeyCode::Power,
        "PrintScreen" => KeyCode::PrintScreen,
        "Props" => KeyCode::Props,
        "Quote" => KeyCode::Quote,
        "ScrollLock" => KeyCode::ScrollLock,
        "Select" => KeyCode::Select,
        "Semicolon" => KeyCode::Semicolon,
        "ShiftLeft" => KeyCode::ShiftLeft,
        "ShiftRight" => KeyCode::ShiftRight,
        "Slash" => KeyCode::Slash,
        "Sleep" => KeyCode::Sleep,
        "Space" => KeyCode::Space,
        "Tab" => KeyCode::Tab,
        "Undo" => KeyCode::Undo,
        "VolumeDown" => KeyCode::AudioVolumeDown,
        "VolumeMute" => KeyCode::AudioVolumeMute,
        "VolumeUp" => KeyCode::AudioVolumeUp,
        "WakeUp" => KeyCode::WakeUp,
        _ => return None,
    })
}
