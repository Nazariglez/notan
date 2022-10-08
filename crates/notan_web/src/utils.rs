use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, Window};

pub fn set_size_dpi(canvas: &HtmlCanvasElement, width: i32, height: i32) {
    let auto_res = canvas
        .get_attribute("notan-auto-res")
        .unwrap_or_else(|| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);
    let dpi = if auto_res {
        web_sys::window().unwrap().device_pixel_ratio() as f32
    } else {
        1.0
    };

    let ww = width as f32 * dpi;
    let hh = height as f32 * dpi;

    canvas.set_width(ww as _);
    canvas.set_height(hh as _);

    if let Err(e) = canvas
        .style()
        .set_property("width", &format!("{}px", width))
    {
        log::error!("{:?}", e);
    }

    if let Err(e) = canvas
        .style()
        .set_property("height", &format!("{}px", height))
    {
        log::error!("{:?}", e);
    }

    if let Err(e) = canvas.set_attribute("notan-width", &width.to_string()) {
        log::error!("{:?}", e);
    }

    if let Err(e) = canvas.set_attribute("notan-height", &height.to_string()) {
        log::error!("{:?}", e);
    }
}

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

    let canvas_element = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|e| format!("{:?}", e))?;

    if let Err(e) = canvas_element.style().set_property("touch-action", "none") {
        log::error!("Cannot set touch-action: none {:?}", e);
    }

    Ok(canvas_element)
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

pub fn document_add_event_listener<F, E>(
    name: &str,
    handler: F,
) -> Result<Closure<dyn FnMut(E)>, String>
where
    E: wasm_bindgen::convert::FromWasmAbi + 'static,
    F: FnMut(E) + 'static,
{
    let doc = web_sys::window()
        .ok_or_else(|| "global window doesn't exists".to_string())?
        .document()
        .ok_or("Can't access document dom object ")?;

    let mut handler = handler;
    let closure = Closure::wrap(Box::new(move |e: E| {
        handler(e);
    }) as Box<dyn FnMut(_)>);

    doc.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
        .map_err(|_| format!("Invalid event name: {}", name))?;
    Ok(closure)
}

#[cfg(feature = "audio")]
pub fn window_remove_event_listener<E>(
    name: &str,
    closure: &Closure<dyn FnMut(E)>,
) -> Result<(), String>
where
    E: wasm_bindgen::convert::FromWasmAbi + 'static,
{
    let win = web_sys::window().ok_or_else(|| "global window doesn't exists".to_string())?;
    win.remove_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
        .map_err(|_| format!("Invalid event name: {}", name))?;

    Ok(())
}

pub fn canvas_position_from_global(
    canvas: &HtmlCanvasElement,
    evt: web_sys::MouseEvent,
) -> (i32, i32) {
    let (x, y) = canvas_pos(canvas, evt.client_x(), evt.client_y());
    (x as _, y as _)
}

pub fn canvas_position_from_touch(
    canvas: &HtmlCanvasElement,
    evt: web_sys::PointerEvent,
) -> (f32, f32) {
    canvas_pos(canvas, evt.client_x(), evt.client_y())
}

fn canvas_pos(canvas: &HtmlCanvasElement, client_x: i32, client_y: i32) -> (f32, f32) {
    let client_x = client_x as f32;
    let client_y = client_y as f32;
    let rect = canvas.get_bounding_client_rect();
    let x = client_x - rect.left() as f32;
    let y = client_y - rect.top() as f32;
    (x, y)
}

pub fn canvas_visible(canvas: &HtmlCanvasElement, visible: bool) {
    if let Err(e) = canvas
        .style()
        .set_property("display", if visible { "block" } else { "none" })
    {
        log::error!("{:?}", e);
    }
}

pub fn canvas_mouse_passthrough(canvas: &HtmlCanvasElement, passthrough: bool) {
    if let Err(e) = canvas
        .style()
        .set_property("pointer-events", if passthrough { "none" } else { "auto" })
    {
        log::error!("{:?}", e);
    }
}

pub fn get_notan_size(canvas: &HtmlCanvasElement) -> (i32, i32) {
    let width = canvas
        .get_attribute("notan-width")
        .unwrap_or_else(|| "0".to_string())
        .parse::<i32>()
        .unwrap_or(0);

    let height = canvas
        .get_attribute("notan-height")
        .unwrap_or_else(|| "0".to_string())
        .parse::<i32>()
        .unwrap_or(0);

    (width, height)
}
