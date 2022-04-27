use js_sys::eval;
use notan_audio::AudioBackend;

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
