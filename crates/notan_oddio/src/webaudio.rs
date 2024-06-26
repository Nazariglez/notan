use hashbrown::HashMap;
use notan_audio::AudioBackend;
use oddio::Frames;
use std::sync::Arc;

use crate::decoder::frames_from_bytes;

const WARN_TEXT: &str =
    "AudioContext cannot be initiated until the user interacts with the webpage.";

/// Dummy audio backend used until the user interacts with the browser
/// This is due security policies of browsers who doesn't allow to
/// play video or sound until the user interacts directly with it
pub(crate) struct DummyAudioBackend {
    pub id_count: u64,
    pub volume: f32,
    pub sources: HashMap<u64, Arc<Frames<[f32; 2]>>>,
}

impl DummyAudioBackend {
    pub fn new() -> Self {
        // Only on debug mode display a warning that the audio context needs an user's interaction to work
        #[cfg(debug_assertions)]
        {
            log::warn!("DEBUG LOG: AudioContext cannot not be enabled until the user interact with the app.");
        }

        Self {
            id_count: 0,
            volume: 1.0,
            sources: Default::default(),
        }
    }
}

impl AudioBackend for DummyAudioBackend {
    fn set_global_volume(&mut self, volume: f32) {
        log::error!("{}", WARN_TEXT);
        self.volume = volume;
    }

    fn global_volume(&self) -> f32 {
        self.volume
    }

    fn create_source(&mut self, bytes: &[u8]) -> Result<u64, String> {
        let frames = frames_from_bytes(bytes)?;

        let id = self.id_count;
        self.sources.insert(id, frames);

        self.id_count += 1;

        Ok(id)
    }

    #[allow(unreachable_code)]
    fn play_sound(&mut self, _source: u64, _volume: f32, _repeat: bool) -> Result<u64, String> {
        log::error!("{}", WARN_TEXT);
        #[cfg(debug_assertions)]
        {
            panic!("AudioContext needs an user's interaction to work.");
        }
        let id = self.id_count;
        self.id_count += 1;
        Ok(id)
    }

    fn pause(&mut self, _sound: u64) {
        log::error!("{}", WARN_TEXT);
    }

    fn resume(&mut self, _sound: u64) {
        log::error!("{}", WARN_TEXT);
    }

    fn stop(&mut self, _sound: u64) {
        log::error!("{}", WARN_TEXT);
    }

    fn is_stopped(&mut self, _sound: u64) -> bool {
        false
    }

    fn is_paused(&mut self, _sound: u64) -> bool {
        false
    }

    fn set_volume(&mut self, _sound: u64, _volume: f32) {
        log::error!("{}", WARN_TEXT);
    }

    fn volume(&self, _sound: u64) -> f32 {
        0.0
    }

    fn clean(&mut self, _sources: &[u64], _sounds: &[u64]) {
        log::error!("{}", WARN_TEXT);
    }
}
