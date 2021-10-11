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
        notan_log::error!("{:?}", e);
    }

    if let Err(e) = canvas
        .style()
        .set_property("height", &format!("{}px", height))
    {
        notan_log::error!("{:?}", e);
    }

    if let Err(e) = canvas.set_attribute("notan-width", &width.to_string()) {
        notan_log::error!("{:?}", e);
    }

    if let Err(e) = canvas.set_attribute("notan-height", &height.to_string()) {
        notan_log::error!("{:?}", e);
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
