use crate::backend::{AudioBackend, AudioSource, Sound};
use crate::tracker::{ResourceId, ResourceTracker};
use std::sync::{Arc, RwLock, RwLockReadGuard};

pub struct Audio {
    backend: Box<dyn AudioBackend>,
    resource_tracker: Arc<ResourceTracker>,
}

impl Audio {
    pub fn new(backend: Box<dyn AudioBackend>) -> Result<Self, String> {
        let resource_tracker = Arc::new(ResourceTracker::default());
        Ok(Self {
            backend,
            resource_tracker,
        })
    }

    #[inline]
    pub fn create_source(&mut self, bytes: &[u8]) -> Result<AudioSource, String> {
        let id = self.backend.create_source(bytes)?;
        Ok(AudioSource::new(id, self.resource_tracker.clone()))
    }

    #[inline]
    pub fn play_sound(&mut self, source: &AudioSource, repeat: bool) -> Sound {
        let id = self.backend.play_sound(source.id, repeat).unwrap();
        Sound::new(id, self.resource_tracker.clone())
    }

    #[inline]
    pub fn resume(&mut self, sound: &Sound) {
        self.backend.resume(sound.id);
    }

    #[inline]
    pub fn stop(&mut self, sound: &Sound) {
        self.backend.stop(sound.id);
    }

    #[inline]
    pub fn pause(&mut self, sound: &Sound) {
        self.backend.pause(sound.id);
    }

    #[inline]
    pub fn is_stopped(&mut self, sound: &Sound) -> bool {
        self.backend.is_stopped(sound.id)
    }

    #[inline]
    pub fn is_paused(&mut self, sound: &Sound) -> bool {
        self.backend.is_paused(sound.id)
    }

    #[inline]
    pub fn set_global_volume(&mut self, volume: f32) {
        self.backend.set_global_volume(clamp_volume(volume));
    }

    #[inline]
    pub fn global_volume(&mut self) -> f32 {
        self.backend.global_volume()
    }

    #[inline]
    pub fn set_volume(&mut self, sound: &Sound, volume: f32) {
        self.backend.set_volume(sound.id, clamp_volume(volume));
    }

    #[inline]
    pub fn volume(&self, sound: &Sound) -> f32 {
        self.backend.volume(sound.id)
    }

    #[inline]
    pub fn clean(&mut self) {
        let resources = self.resource_tracker.dropped.read();
        if resources.is_empty() {
            return;
        }

        let mut sources = vec![];
        let mut sounds = vec![];
        for res in resources.iter() {
            match res {
                ResourceId::Source(id) => sources.push(*id),
                ResourceId::Sound(id) => sounds.push(*id),
            }
        }

        // drop resources here to avoid deadlock calling clean
        drop(resources);

        self.backend.clean(&sources, &sounds);
        self.resource_tracker.clean();
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
