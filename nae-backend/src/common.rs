use futures::future::{poll_fn, result};
use futures::{future, Async, Future};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;
#[cfg(target_arch = "wasm32")]
use web_sys::{XmlHttpRequest, XmlHttpRequestResponseType};

pub(crate) trait ToNaeValue {
    type Kind;

    fn to_nae(&self) -> Self::Kind;
}

//TODO this should be done in a async way, keeping in mind that this should work on mobile devices and desktops
/// Read the content of a file and return a future with the content
#[cfg(not(target_arch = "wasm32"))]
pub fn load_file(path: &str) -> impl Future<Item = Vec<u8>, Error = String> {
    future::result(load_from_disk(path)).map_err(|e| e.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn load_from_disk(path: impl AsRef<Path>) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = Vec::new();
    File::open(path)?.read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(target_arch = "wasm32")]
fn xhr_req(url: &str) -> Result<XmlHttpRequest, String> {
    let xhr = XmlHttpRequest::new().map_err(|e| e.as_string().unwrap())?;

    xhr.set_response_type(XmlHttpRequestResponseType::Arraybuffer);
    xhr.open("GET", url).map_err(|e| e.as_string().unwrap())?;
    xhr.send().map_err(|e| e.as_string().unwrap())?;

    Ok(xhr)
}

#[cfg(target_arch = "wasm32")]
pub fn load_file(path: &str) -> impl Future<Item = Vec<u8>, Error = String> {
    result(xhr_req(path)).and_then(|xhr| {
        // Code ported from quicksilver https://github.com/ryanisaacg/quicksilver/blob/master/src/file.rs#L30
        poll_fn(move || {
            let status = xhr.status().unwrap() / 100;
            let done = xhr.ready_state() == 4;
            match (status, done) {
                (2, true) => Ok(Async::Ready(xhr.response().unwrap())),
                (2, _) => Ok(Async::NotReady),
                (0, _) => Ok(Async::NotReady),
                _ => Err("Error loading file.".to_string()), //todo add path to know which file is failing. (borrow error here?)
            }
        })
        .and_then(|data| {
            let js_arr: Uint8Array = Uint8Array::new(&data);
            let mut arr = vec![];
            let mut cb = |a, _b, _c| {
                arr.push(a);
            };
            js_arr.for_each(&mut cb);
            Ok(arr)
            //TODO why this panic?
            //   let mut arr = vec![];
            //   js_arr.copy_to(arr.as_mut_slice());
            //   Ok(arr)
        })
    })
}
