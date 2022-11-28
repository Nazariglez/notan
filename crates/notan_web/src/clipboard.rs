#![cfg(feature = "clipboard")]
#![cfg(web_sys_unstable_apis)]

use crate::utils::window_add_event_listener;
use crate::window::WebWindowBackend;
use notan_core::events::Event;
use wasm_bindgen::prelude::*;
use web_sys::ClipboardEvent;

#[derive(Default)]
pub struct ClipboardCallbacks {
    on_cut: Option<Closure<dyn FnMut(ClipboardEvent)>>,
    on_copy: Option<Closure<dyn FnMut(ClipboardEvent)>>,
    on_paste: Option<Closure<dyn FnMut(ClipboardEvent)>>,
}

pub fn enable_clipboard(win: &mut WebWindowBackend) -> Result<(), String> {
    let add_evt_copy = win.add_event_fn();
    let add_evt_cut = win.add_event_fn();
    let add_evt_paste = win.add_event_fn();
    let callbacks = &mut win.clipboard_callbacks;
    callbacks.on_copy = Some(window_add_event_listener(
        "copy",
        move |_: ClipboardEvent| {
            add_evt_copy(Event::Copy);
        },
    )?);

    callbacks.on_cut = Some(window_add_event_listener(
        "cut",
        move |_: ClipboardEvent| {
            add_evt_cut(Event::Cut);
        },
    )?);

    callbacks.on_paste = Some(window_add_event_listener(
        "paste",
        move |e: ClipboardEvent| {
            if let Some(data) = e.clipboard_data() {
                if let Ok(text) = data.get_data("text") {
                    let text = text.replace("\r\n", "\n");
                    if !text.is_empty() {
                        add_evt_paste(Event::Paste(text));
                    }
                }
            }
        },
    )?);

    Ok(())
}

pub fn set_clipboard_text(text: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(clipboard) = window.navigator().clipboard() {
            let promise = clipboard.write_text(text);
            let future = wasm_bindgen_futures::JsFuture::from(promise);
            let future = async move {
                if let Err(err) = future.await {
                    log::error!("failed to set text on clipboard: {:?}", err);
                }
            };
            wasm_bindgen_futures::spawn_local(future);
        }
    }
}
