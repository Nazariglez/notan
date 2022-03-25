use crate::utils::canvas_add_event_listener;
use crate::window::WebWindowBackend;
use notan_app::Event;
use wasm_bindgen::prelude::*;
use web_sys::DragEvent;

#[cfg(feature = "drop_files")]
use notan_app::DroppedFile;

#[derive(Default)]
pub struct FileCallbacks {
    on_drag_enter: Option<Closure<dyn FnMut(DragEvent)>>,
    on_drag_over: Option<Closure<dyn FnMut(DragEvent)>>,
    on_drag_leave: Option<Closure<dyn FnMut(DragEvent)>>,
    on_drop: Option<Closure<dyn FnMut(DragEvent)>>,
}

pub fn enable_files(win: &mut WebWindowBackend) -> Result<(), String> {
    let add_evt_drop = win.add_event_fn();
    let add_evt_enter = win.add_event_fn();
    let add_evt_leave = win.add_event_fn();
    let callbacks = &mut win.file_callbacks;
    let events = win.events.clone();

    callbacks.on_drop = Some({
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

                                add_evt_drop(Event::Drop(DroppedFile {
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
        canvas_add_event_listener(&win.canvas, "dragenter", move |e: DragEvent| {
            e.stop_propagation();
            e.prevent_default();

            if let Some(dt) = e.data_transfer() {
                let len = dt.items().length();
                (0..len).for_each(|i| {
                    if let Some(item) = dt.items().get(i) {
                        if item.kind() == "file" {
                            let mime = item.type_();
                            add_evt_enter(Event::DragEnter {
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
            add_evt_leave(Event::DragLeft);
        },
    )?);

    Ok(())
}
