#![cfg(feature = "save_file")]

use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
pub fn save_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<(), String> {
    use std::io::Write;

    let mut f = std::fs::File::create(path).map_err(|e| e.to_string())?;

    f.write_all(bytes).map_err(|e| e.to_string())
}

#[cfg(target_arch = "wasm32")]
pub fn save_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<(), String> {
    use js_sys::{Array, Uint8Array};
    use wasm_bindgen::JsCast;
    use web_sys::Url;

    let mime = mime_guess::from_path(&path)
        .first_raw()
        .unwrap_or("")
        .to_string();

    let mut u8_buff = Uint8Array::new_with_length(bytes.len() as _);
    u8_buff.copy_from(&bytes);

    let mut array = Array::new();
    array.push(&u8_buff.buffer());

    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
        &array,
        &web_sys::BlobPropertyBag::new().type_(&mime),
    )
    .map_err(|_| format!("Cannot create a blob from file"))?;

    let url = web_sys::Url::create_object_url_with_blob(&blob)
        .map_err(|_| format!("Cannot create a blob from file"))?;

    let mut a = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("a")
        .unwrap()
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .unwrap();

    let p = path.as_ref();

    a.set_href(&url);
    a.set_download(p.file_name().unwrap().to_str().unwrap());
    a.click();

    Url::revoke_object_url(&url);

    Ok(())
}
