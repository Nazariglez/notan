use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, Window};

pub fn request_animation_frame(win: &Window, f: &Closure<dyn FnMut()>) -> i32 {
    win.request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

pub fn get_or_create_canvas(doc: &Document, canvas_id: &str) -> Result<HtmlCanvasElement, String> {
    let canvas = match doc.get_element_by_id(canvas_id) {
        Some(c) => c,
        None => {
            let c = doc
                .create_element("canvas")
                .map_err(|e| format!("{:?}", e))?;

            let body = doc
                .body()
                .ok_or_else(|| "body doesn't exists on document.".to_string())?;
            body.append_child(&c).map_err(|e| format!("{:?}", e))?;

            c.set_id(canvas_id);
            c
        }
    };
    canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|e| format!("{:?}", e))
}

pub fn canvas_add_event_listener<F, E>(
    canvas: &HtmlCanvasElement,
    name: &str,
    handler: F,
) -> Result<Closure<dyn FnMut(E)>, String>
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
        .map_err(|_| format!("Invalid event name: {}", name))?;
    Ok(closure)
}

pub fn window_add_event_listener<F, E>(
    name: &str,
    handler: F,
) -> Result<Closure<dyn FnMut(E)>, String>
where
    E: wasm_bindgen::convert::FromWasmAbi + 'static,
    F: FnMut(E) + 'static,
{
    let win = web_sys::window().ok_or_else(|| "global window doesn't exists".to_string())?;

    let mut handler = handler;
    let closure = Closure::wrap(Box::new(move |e: E| {
        handler(e);
    }) as Box<dyn FnMut(_)>);

    win.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
        .map_err(|_| format!("Invalid event name: {}", name))?;
    Ok(closure)
}

pub fn canvas_position_from_global(
    canvas: &HtmlCanvasElement,
    evt: web_sys::MouseEvent,
) -> (i32, i32) {
    let client_x = evt.client_x() as f32;
    let client_y = evt.client_y() as f32;
    let rect = canvas.get_bounding_client_rect();
    let x = client_x - rect.left() as f32;
    let y = client_y - rect.top() as f32;
    (x as i32, y as i32)
}
