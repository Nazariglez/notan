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

/// Dummy audio backend used until the user interacts with the browser
/// This is due security policies of browsers who doesn't allow to
/// play video or sound until the user interacts directly with it
#[derive(Default)]
pub(crate) struct DummyAudioBackend {
    id_count: u64,
    volume: f32,
}

impl DummyAudioBackend {
    pub fn new() -> Self {
        let dummy: Self = Default::default();

        /// Only on debug mode display a warning that the audio context needs an user's interaction to work
        #[cfg(debug_assertions)]
        {
            log::warn!("DEBUG LOG: AudioContext cannot not be enabled until the user interact with the app. {:p}", &dummy);
        }

        dummy
    }
}

impl AudioBackend for DummyAudioBackend {
    fn set_global_volume(&mut self, volume: f32) {
        log::error!("AudioContext needs an user's interaction to work.");
        self.volume = volume;
    }

    fn global_volume(&self) -> f32 {
        self.volume
    }

    fn create_source(&mut self, _bytes: &[u8]) -> Result<u64, String> {
        log::error!(
            "AudioContext needs an user's interaction to work. {:p}",
            self
        );
        let id = self.id_count;
        self.id_count += 1;
        Ok(id)
    }

    fn play_sound(&mut self, _source: u64, _repeat: bool) -> Result<u64, String> {
        log::error!("AudioContext needs an user's interaction to work.");
        let id = self.id_count;
        self.id_count += 1;
        Ok(id)
    }

    fn pause(&mut self, _sound: u64) {
        log::error!("AudioContext needs an user's interaction to work.");
    }

    fn resume(&mut self, _sound: u64) {
        log::error!("AudioContext needs an user's interaction to work.");
    }

    fn stop(&mut self, _sound: u64) {
        log::error!("AudioContext needs an user's interaction to work.");
    }

    fn is_stopped(&mut self, _sound: u64) -> bool {
        false
    }

    fn is_paused(&mut self, _sound: u64) -> bool {
        false
    }

    fn set_volume(&mut self, _sound: u64, _volume: f32) {
        log::error!("AudioContext needs an user's interaction to work.");
    }

    fn volume(&self, _sound: u64) -> f32 {
        0.0
    }

    fn clean(&mut self, _sources: &[u64], _sounds: &[u64]) {
        log::error!("AudioContext needs an user's interaction to work.");
    }
}
