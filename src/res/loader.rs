use super::resource::*;
use crate::res::ResourceManager;
use futures::future::{poll_fn, result, Future};
use futures::Async;
use hashbrown::HashMap;
use js_sys::Uint8Array;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{XmlHttpRequest, XmlHttpRequestEventTarget, XmlHttpRequestResponseType};

fn xhr_req(url: &str) -> Result<XmlHttpRequest, String> {
    let mut xhr = XmlHttpRequest::new().map_err(|e| e.as_string().unwrap())?;

    xhr.set_response_type(XmlHttpRequestResponseType::Arraybuffer);
    xhr.open("GET", url).map_err(|e| e.as_string().unwrap())?;
    xhr.send().map_err(|e| e.as_string().unwrap())?;

    Ok(xhr)
}

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
                _ => Err(format!("Error loading file.")), //todo add path to know which file is failing. (borrow error here?)
            }
        })
        .and_then(|data| {
            let js_arr: Uint8Array = Uint8Array::new(&data);
            let mut arr = vec![];
            let mut cb = |a, b, c| {
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

//TODO loader to load resources in batch doesn't works...
pub struct Loader<'a> {
    to_load: HashMap<String, Rc<RefCell<Resource + 'a>>>,
    manager: &'a mut ResourceManager<'a>,
}

impl<'a> Loader<'a> {
    pub fn new(manager: &'a mut ResourceManager<'a>) -> Self {
        Self {
            to_load: HashMap::new(),
            manager: manager,
        }
    }

    pub fn add<A>(&mut self, file: &str) -> &mut Self
    where
        A: Resource + ResourceConstructor + Clone + 'a,
    {
        let asset = self.manager.load::<A>(file).unwrap();
        let asset = Rc::new(RefCell::new(asset));
        self.to_load.insert(file.to_string(), asset.clone());
        self
    }

    //    pub fn get<A>(&self, file: &str) -> Option<&mut A>
    //        where A: Resource + ResourceConstructor + 'a
    //    {
    //        Some(self.to_load.get(file).unwrap().borrow_mut())
    //    }
    //
    //    pub fn load<A>(files: Vec<&str>) -> Self
    //    where A: ResourceConstructor + Resource
    //    {
    //        self.to_load = files.iter()
    //            .map()
    //    }
}
