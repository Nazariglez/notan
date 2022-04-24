use std::sync::Arc;

/// Represent the audio implementation backend
pub trait AudioBackend {
    fn create_source(&mut self, bytes: &[u8]) -> Result<u64, String>;
    fn play_sound(&mut self, source: u64, repeat: bool) -> Result<u64, String>;
    fn play(&mut self, sound: u64);
    fn stop(&mut self, sound: u64);
}

pub struct AudioSource {
    id: u64,
}

pub struct SoundId {
    id: u64,
}

pub struct Audio {
    backend: Box<dyn AudioBackend>,
    // drop?
}

impl Audio {
    pub fn new(backend: Box<dyn AudioBackend>) -> Result<Self, String> {
        Ok(Self { backend })
    }

    pub fn create_source(&mut self, bytes: &[u8]) -> Result<AudioSource, String> {
        let id = self.backend.create_source(bytes)?;
        Ok(AudioSource { id })
    }

    pub fn play_sound(&mut self, source: &AudioSource, repeat: bool) -> SoundId {
        let id = self.backend.play_sound(source.id, repeat).unwrap();
        SoundId { id }
    }

    pub fn play(&mut self, sound: &SoundId) {
        self.backend.play(sound.id);
    }

    pub fn stop(&mut self, sound: &SoundId) {
        self.backend.stop(sound.id);
    }
}
