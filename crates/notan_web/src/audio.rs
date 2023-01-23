#![cfg(feature = "audio")]
use crate::utils::window_remove_event_listener;

use js_sys::eval;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};

#[allow(unused)]
#[inline]
pub(crate) fn fix_webaudio_if_necessary() {
    // todo check if its chrome?
    webaudio_fix_eval();
}

fn webaudio_fix_eval() {
    // https://developer.chrome.com/blog/web-audio-autoplay/#moving-forward

    // language=javascript
    let code = r#"
        (function () {
            console.log("here...");
          // An array of all contexts to resume on the page
          const audioContextList = [];

          // An array of various user interaction events we should listen for
          const userInputEventNames = [
            'click',
            'contextmenu',
            'auxclick',
            'dblclick',
            'mousedown',
            'mouseup',
            'pointerup',
            'touchend',
            'keydown',
            'keyup'
          ];

          // A proxy object to intercept AudioContexts and
          // add them to the array for tracking and resuming later
          window.AudioContext = new Proxy(window.AudioContext, {
            construct(target, args) {
                console.log("audio context!!");
              const result = new target(...args);
              audioContextList.push(result);
              return result;
            }
          });

          // To resume all AudioContexts being tracked
          function resumeAllContexts(event) {
              console.log("resume?");
            let count = 0;

            audioContextList.forEach(context => {
              if (context.state !== 'running') {
                context.resume();
              } else {
                count++;
              }
            });

            // If all the AudioContexts have now resumed then we
            // unbind all the event listeners from the page to prevent
            // unnecessary resume attempts
            if (count == audioContextList.length) {
              userInputEventNames.forEach(eventName => {
                document.removeEventListener(eventName, resumeAllContexts);
              });
            }
          }

          // We bind the resume function for each user interaction
          // event on the page
          userInputEventNames.forEach(eventName => {
              console.log("bind", eventName);
            document.addEventListener(eventName, resumeAllContexts);
          });
        })();
    "#;

    let result = eval(code);
    if let Err(e) = result {
        log::error!("Error evaluating webaudio fix for chrome: {:?}", e);
    }
}

pub(crate) fn enable_webaudio<F: FnMut() + 'static>(mut handler: F) {
    let win = web_sys::window().unwrap();
    let closure_wrapper = Rc::new(RefCell::new(None));

    let event_list = [
        "click",
        "contextmenu",
        "auxclick",
        "dblclick",
        "mousedown",
        "mouseup",
        "pointerup",
        "touchend",
        "keydown",
        "keyup",
    ];

    let cw = closure_wrapper.clone();
    let closure = Closure::wrap(Box::new(move |_: JsValue| {
        log::debug!("Enabling WebAudio AudioContext");
        handler();
        let closure = cw.borrow();
        event_list.iter().for_each(|name| {
            window_remove_event_listener(name, closure.as_ref().unwrap()).unwrap();
        });
    }) as Box<dyn FnMut(_)>);

    event_list.iter().for_each(|name| {
        win.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
            .unwrap();
    });

    closure_wrapper.replace(Some(closure));
}
