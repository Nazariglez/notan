#![cfg(feature = "clipboard")]

use glutin::event::VirtualKeyCode;
use glutin::event::{ElementState, WindowEvent};
use notan_core::events::Event;
use notan_input::keyboard::Keyboard;

pub fn process_events(event: &WindowEvent, keyboard: &Keyboard) -> Option<Event> {
    match event {
        WindowEvent::KeyboardInput { input, .. } => {
            if let Some(key) = input.virtual_keycode.as_ref() {
                if input.state == ElementState::Pressed {
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
            log::error!("failed to set_text on clipboard: {}", err);
        }
    }
}

fn get_clipboard_text() -> Option<String> {
    if let Some(mut clipboard) = init_arboard() {
        return match clipboard.get_text() {
            Ok(text) => Some(text),
            Err(err) => {
                log::error!("failed to get_text from clipboard: {}", err);
                None
            }
        };
    }

    None
}

fn is_cut(keyboard: &Keyboard, keycode: &VirtualKeyCode) -> bool {
    is_command_pressed(keyboard) && *keycode == VirtualKeyCode::X
        || (cfg!(target_os = "windows") && keyboard.shift() && *keycode == VirtualKeyCode::Delete)
}

fn is_copy(keyboard: &Keyboard, keycode: &VirtualKeyCode) -> bool {
    is_command_pressed(keyboard) && *keycode == VirtualKeyCode::C
        || (cfg!(target_os = "windows") && keyboard.ctrl() && *keycode == VirtualKeyCode::Insert)
}

fn is_paste(keyboard: &Keyboard, keycode: &VirtualKeyCode) -> bool {
    is_command_pressed(keyboard) && *keycode == VirtualKeyCode::V
        || (cfg!(target_os = "windows") && keyboard.shift() && *keycode == VirtualKeyCode::Insert)
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
            log::error!("failed to initialize clipboard: {}", err);
            None
        }
    }
}
