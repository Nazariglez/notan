use crate::utils::canvas_add_event_listener;
use crate::window::WebWindowBackend;
use futures_util::{FutureExt, TryFutureExt};
use notan_app::{DroppedFile, Event};
use std::cell::RefCell;
use std::future::Future;
use std::path::PathBuf;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::DragEvent;

#[derive(Default)]
pub struct FileCallbacks {
    on_drag_enter: Option<Closure<dyn FnMut(DragEvent)>>,
    on_drag_over: Option<Closure<dyn FnMut(DragEvent)>>,
    on_drag_leave: Option<Closure<dyn FnMut(DragEvent)>>,
    on_drop: Option<Closure<dyn FnMut(DragEvent)>>,
}

pub fn enable_files(win: &mut WebWindowBackend) -> Result<(), String> {
    let callbacks = &mut win.file_callbacks;
    let events = win.events.clone();

    callbacks.on_drop = Some({
        let events = events.clone();
        canvas_add_event_listener(&win.canvas, "drop", move |e: DragEvent| {
            e.stop_propagation();
            e.prevent_default();

            if let Some(dt) = e.data_transfer() {
                if let Some(files) = dt.files() {
                    let len = files.length();
                    if len > 0 {
                        (0..len).for_each(|i| {
                            if let Some(file) = files.item(i) {
                                let name = file.name();
                                let mime = file.type_();

                                events.borrow_mut().push(Event::Drop(DroppedFile {
                                    name,
                                    mime,
                                    file: Some(file),
                                    ..Default::default()
                                }))
                            }
                        });
                    }
                }
            }
        })?
    });

    callbacks.on_drag_over = Some(canvas_add_event_listener(
        &win.canvas,
        "dragover",
        |e: DragEvent| {
            e.stop_propagation();
            e.prevent_default();
        },
    )?);

    callbacks.on_drag_enter = Some({
        let events = events.clone();

        canvas_add_event_listener(&win.canvas, "dragenter", move |e: DragEvent| {
            e.stop_propagation();
            e.prevent_default();

            if let Some(dt) = e.data_transfer() {
                let len = dt.items().length();
                (0..len).for_each(|i| {
                    if let Some(item) = dt.items().get(i) {
                        if item.kind() == "file" {
                            let mime = item.type_();
                            events.borrow_mut().push(Event::DragEnter {
                                path: None,
                                name: None,
                                mime,
                            });
                        }
                    }
                })
            }
        })?
    });

    callbacks.on_drag_leave = Some(canvas_add_event_listener(
        &win.canvas,
        "dragleave",
        move |e: DragEvent| {
            e.stop_propagation();
            e.prevent_default();
            events.borrow_mut().push(Event::DragLeft);
        },
    )?);

    Ok(())
}
