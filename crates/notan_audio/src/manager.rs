use crate::backend::{AudioBackend, AudioSource, Sound};
use crate::tracker::{ResourceId, ResourceTracker};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

pub struct Audio {
    backend: Rc<RefCell<dyn AudioBackend>>,
    resource_tracker: Arc<ResourceTracker>,
}

impl Audio {
    pub fn new(backend: Rc<RefCell<dyn AudioBackend>>) -> Result<Self, String> {
        let resource_tracker = Arc::new(ResourceTracker::default());
        Ok(Self {
            backend,
            resource_tracker,
        })
    }

    #[inline]
    pub fn create_source(&mut self, bytes: &[u8]) -> Result<AudioSource, String> {
        let id = self.backend.borrow_mut().create_source(bytes)?;
        Ok(AudioSource::new(id, self.resource_tracker.clone()))
    }

    #[inline]
    pub fn play_sound(&mut self, source: &AudioSource, repeat: bool) -> Sound {
        let id = self
            .backend
            .borrow_mut()
            .play_sound(source.id, repeat)
            .unwrap();
        Sound::new(id, self.resource_tracker.clone())
    }

    #[inline]
    pub fn resume(&mut self, sound: &Sound) {
        self.backend.borrow_mut().resume(sound.id);
    }

    #[inline]
    pub fn stop(&mut self, sound: &Sound) {
        self.backend.borrow_mut().stop(sound.id);
    }

    #[inline]
    pub fn pause(&mut self, sound: &Sound) {
        self.backend.borrow_mut().pause(sound.id);
    }

    #[inline]
    pub fn is_stopped(&self, sound: &Sound) -> bool {
        self.backend.borrow_mut().is_stopped(sound.id)
    }

    #[inline]
    pub fn is_paused(&self, sound: &Sound) -> bool {
        self.backend.borrow_mut().is_paused(sound.id)
    }

    #[inline]
    pub fn set_global_volume(&mut self, volume: f32) {
        self.backend
            .borrow_mut()
            .set_global_volume(clamp_volume(volume));
    }

    #[inline]
    pub fn global_volume(&self) -> f32 {
        self.backend.borrow().global_volume()
    }

    #[inline]
    pub fn set_volume(&mut self, sound: &Sound, volume: f32) {
        self.backend
            .borrow_mut()
            .set_volume(sound.id, clamp_volume(volume));
    }

    #[inline]
    pub fn volume(&self, sound: &Sound) -> f32 {
        self.backend.borrow().volume(sound.id)
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

        self.backend.borrow_mut().clean(&sources, &sounds);
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
