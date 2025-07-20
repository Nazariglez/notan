#![cfg(feature = "clipboard")]

use notan_core::events::Event;
use notan_input::keyboard::Keyboard;
use winit::event::{ElementState, WindowEvent};
use winit::keyboard::{KeyCode as WinitKeyCode, PhysicalKey};

pub fn process_events(event: &WindowEvent, keyboard: &Keyboard) -> Option<Event> {
    match event {
        WindowEvent::KeyboardInput { event, .. } => {
            if let PhysicalKey::Code(ref key) = event.physical_key {
                if event.state == ElementState::Pressed {
                    if is_cut(keyboard, key) {
                        return Some(Event::Cut);
                    } else if is_copy(keyboard, key) {
                        return Some(Event::Copy);
                    } else if is_paste(keyboard, key) {
                        if let Some(contents) = get_clipboard_text() {
                            let contents = contents.replace("\r\n", "\n");
                            if !contents.is_empty() {
                                return Some(Event::Paste(contents));
                            }
                        }
                    }
                }
            }

            None
        }
        _ => None,
    }
}

pub fn set_clipboard_text(text: &str) {
    if let Some(mut clipboard) = init_arboard() {
        if let Err(err) = clipboard.set_text(text) {
            log::error!("failed to set_text on clipboard: {err}");
        }
    }
}

fn get_clipboard_text() -> Option<String> {
    if let Some(mut clipboard) = init_arboard() {
        return match clipboard.get_text() {
            Ok(text) => Some(text),
            Err(err) => {
                log::error!("failed to get_text from clipboard: {err}");
                None
            }
        };
    }

    None
}

fn is_cut(keyboard: &Keyboard, keycode: &WinitKeyCode) -> bool {
    is_command_pressed(keyboard) && *keycode == WinitKeyCode::KeyX
        || (cfg!(target_os = "windows") && keyboard.shift() && *keycode == WinitKeyCode::Delete)
}

fn is_copy(keyboard: &Keyboard, keycode: &WinitKeyCode) -> bool {
    is_command_pressed(keyboard) && *keycode == WinitKeyCode::KeyC
        || (cfg!(target_os = "windows") && keyboard.ctrl() && *keycode == WinitKeyCode::Insert)
}

fn is_paste(keyboard: &Keyboard, keycode: &WinitKeyCode) -> bool {
    is_command_pressed(keyboard) && *keycode == WinitKeyCode::KeyV
        || (cfg!(target_os = "windows") && keyboard.shift() && *keycode == WinitKeyCode::Insert)
}

// returns true for ⌘ Command on mac and ctrl on others
fn is_command_pressed(keyboard: &Keyboard) -> bool {
    let mac_cmd = if cfg!(target_os = "macos") || cfg!(target_arch = "wasm32") {
        keyboard.logo()
    } else {
        false
    };

    mac_cmd || keyboard.ctrl()
}

fn init_arboard() -> Option<arboard::Clipboard> {
    match arboard::Clipboard::new() {
        Ok(clipboard) => Some(clipboard),
        Err(err) => {
            log::error!("failed to initialize clipboard: {err}");
            None
        }
    }
}
