use std::sync::Arc;

// TODO DROP HELPER

/// Represent the audio implementation backend
pub trait AudioBackend {
    fn set_global_volume(&mut self, volume: f32);
    fn global_volume(&self) -> f32;
    fn create_source(&mut self, bytes: &[u8]) -> Result<u64, String>;
    fn play_sound(&mut self, source: u64, repeat: bool) -> Result<u64, String>;
    fn pause(&mut self, sound: u64);
    fn resume(&mut self, sound: u64);
    fn stop(&mut self, sound: u64);
    fn is_stopped(&mut self, sound: u64) -> bool;
    fn is_paused(&mut self, sound: u64) -> bool;
    fn set_volume(&mut self, sound: u64, volume: f32);
    fn volume(&self, sound: u64) -> f32;
    // fn clean(&mut self, sources: &[u64], sounds: &[u64]); // todo
    // fn remaining_time(&self, sound: u64) -> f32;
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

    pub fn resume(&mut self, sound: &SoundId) {
        self.backend.resume(sound.id);
    }

    pub fn stop(&mut self, sound: &SoundId) {
        self.backend.stop(sound.id);
    }

    pub fn pause(&mut self, sound: &SoundId) {
        self.backend.pause(sound.id);
    }

    pub fn is_stopped(&mut self, sound: &SoundId) -> bool {
        self.backend.is_stopped(sound.id)
    }

    pub fn is_paused(&mut self, sound: &SoundId) -> bool {
        self.backend.is_paused(sound.id)
    }

    pub fn set_global_volume(&mut self, volume: f32) {
        self.backend.set_global_volume(clamp_volume(volume));
    }

    pub fn global_volume(&mut self) -> f32 {
        self.backend.global_volume()
    }

    pub fn set_volume(&mut self, sound: &SoundId, volume: f32) {
        self.backend.set_volume(sound.id, clamp_volume(volume));
    }

    pub fn volume(&self, sound: &SoundId) -> f32 {
        self.backend.volume(sound.id)
    }
}

fn clamp_volume(volume: f32) -> f32 {
    if volume < 0.0 {
        0.0
    } else if volume > 1.0 {
        1.0
    } else {
        volume
    }
}
