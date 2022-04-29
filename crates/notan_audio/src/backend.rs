use crate::tracker::{ResourceId, ResourceTracker};
use std::sync::Arc;

/// Represent the audio implementation backend
pub trait AudioBackend {
    fn set_global_volume(&mut self, volume: f32);
    fn global_volume(&self) -> f32;
    fn create_source(&mut self, bytes: &[u8]) -> Result<u64, String>;
    fn play_sound(&mut self, source: u64, repeat: bool) -> Result<u64, String>;
    fn pause(&mut self, sound: u64);
    fn resume(&mut self, sound: u64);
    fn stop(&mut self, sound: u64);
    #[allow(clippy::wrong_self_convention)]
    fn is_stopped(&mut self, sound: u64) -> bool;
    #[allow(clippy::wrong_self_convention)]
    fn is_paused(&mut self, sound: u64) -> bool;
    fn set_volume(&mut self, sound: u64, volume: f32);
    fn volume(&self, sound: u64) -> f32;
    fn clean(&mut self, sources: &[u64], sounds: &[u64]);
    // fn remaining_time(&self, sound: u64) -> f32;
}

#[derive(Debug)]
struct SourceIdRef {
    id: u64,
    tracker: Arc<ResourceTracker>,
}

impl Drop for SourceIdRef {
    fn drop(&mut self) {
        self.tracker.push(ResourceId::Source(self.id));
    }
}

#[derive(Debug, Clone)]
pub struct AudioSource {
    pub(crate) id: u64,
    _id_ref: Arc<SourceIdRef>,
}

impl AudioSource {
    pub(crate) fn new(id: u64, tracker: Arc<ResourceTracker>) -> Self {
        let _id_ref = Arc::new(SourceIdRef { id, tracker });
        Self { id, _id_ref }
    }
}

impl PartialEq for AudioSource {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug)]
struct SoundIdRef {
    id: u64,
    tracker: Arc<ResourceTracker>,
}

impl Drop for SoundIdRef {
    fn drop(&mut self) {
        self.tracker.push(ResourceId::Sound(self.id));
    }
}

#[derive(Debug, Clone)]
pub struct Sound {
    pub(crate) id: u64,
    _id_ref: Arc<SoundIdRef>,
}

impl Sound {
    pub(crate) fn new(id: u64, tracker: Arc<ResourceTracker>) -> Self {
        let _id_ref = Arc::new(SoundIdRef { id, tracker });
        Self { id, _id_ref }
    }
}

impl PartialEq for Sound {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
