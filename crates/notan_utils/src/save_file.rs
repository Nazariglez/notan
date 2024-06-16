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

    let u8_buff = Uint8Array::new_with_length(bytes.len() as _);
    u8_buff.copy_from(bytes);

    let array = Array::new();
    array.push(&u8_buff.buffer());

    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
        &array,
        web_sys::BlobPropertyBag::new().type_(&mime),
    )
    .map_err(|_| "Cannot create a blob from file".to_string())?;

    let url = web_sys::Url::create_object_url_with_blob(&blob)
        .map_err(|_| "Cannot create a blob from file".to_string())?;

    let a = web_sys::window()
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

    if let Err(e) = Url::revoke_object_url(&url) {
        log::error!("{:?}", e);
    }

    Ok(())
}
