use crate::decoder::frames_from_bytes;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::BufferSize;
use hashbrown::HashMap;
use notan_audio::AudioBackend;
use oddio::{Cycle, Frames, FramesSignal, Gain, GainControl, Handle, Mixer, Stop, StopControl};
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use crate::webaudio::DummyAudioBackend;

type FrameHandle = Handle<Stop<Gain<FramesSignal<[f32; 2]>>>>;
type CycleHandle = Handle<Stop<Gain<Cycle<[f32; 2]>>>>;

struct AudioInfo {
    handle: AudioHandle,
    volume: f32,
}

enum AudioHandle {
    Frame(FrameHandle),
    Cycle(CycleHandle),
}

impl AudioHandle {
    fn as_stop(&mut self) -> StopControl {
        match self {
            AudioHandle::Frame(h) => h.control::<Stop<_>, _>(),
            AudioHandle::Cycle(h) => h.control::<Stop<_>, _>(),
        }
    }

    fn as_gain(&mut self) -> GainControl {
        match self {
            AudioHandle::Frame(h) => h.control::<Gain<_>, _>(),
            AudioHandle::Cycle(h) => h.control::<Gain<_>, _>(),
        }
    }
}

enum BackendImpl {
    Oddio(InnerBackend),

    #[cfg(target_arch = "wasm32")]
    Dummy(DummyAudioBackend),
}

pub struct OddioBackend {
    inner: BackendImpl,
}

impl OddioBackend {
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            inner: BackendImpl::Dummy(DummyAudioBackend::new()),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            inner: BackendImpl::Oddio(InnerBackend::new()?),
        })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn enable(&mut self) -> Result<(), String> {
        let inner = if let BackendImpl::Dummy(dummy) = &mut self.inner {
            let mut inner = InnerBackend::new()?;
            std::mem::swap(&mut inner.sources, &mut dummy.sources);
            inner.source_id_count = dummy.id_count;
            inner.set_global_volume(dummy.volume);
            Some(inner)
        } else {
            None
        };

        if let Some(inner) = inner {
            self.inner = BackendImpl::Oddio(inner);
        }

        Ok(())
    }
}

impl AudioBackend for OddioBackend {
    #[inline]
    fn set_global_volume(&mut self, volume: f32) {
        match &mut self.inner {
            BackendImpl::Oddio(inner) => inner.set_global_volume(volume),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.set_global_volume(volume),
        }
    }

    #[inline]
    fn global_volume(&self) -> f32 {
        match &self.inner {
            BackendImpl::Oddio(inner) => inner.global_volume(),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.global_volume(),
        }
    }

    #[inline]
    fn create_source(&mut self, bytes: &[u8]) -> Result<u64, String> {
        match &mut self.inner {
            BackendImpl::Oddio(inner) => inner.create_source(bytes),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.create_source(bytes),
        }
    }

    #[inline]
    fn play_sound(&mut self, source: u64, volume: f32, repeat: bool) -> Result<u64, String> {
        match &mut self.inner {
            BackendImpl::Oddio(inner) => inner.play_sound(source, volume, repeat),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.play_sound(source, volume, repeat),
        }
    }

    #[inline]
    fn pause(&mut self, sound: u64) {
        match &mut self.inner {
            BackendImpl::Oddio(inner) => inner.pause(sound),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.pause(sound),
        }
    }

    #[inline]
    fn resume(&mut self, sound: u64) {
        match &mut self.inner {
            BackendImpl::Oddio(inner) => inner.resume(sound),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.resume(sound),
        }
    }

    #[inline]
    fn stop(&mut self, sound: u64) {
        match &mut self.inner {
            BackendImpl::Oddio(inner) => inner.stop(sound),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.stop(sound),
        }
    }

    #[inline]
    fn is_stopped(&mut self, sound: u64) -> bool {
        match &mut self.inner {
            BackendImpl::Oddio(inner) => inner.is_stopped(sound),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.is_stopped(sound),
        }
    }

    #[inline]
    fn is_paused(&mut self, sound: u64) -> bool {
        match &mut self.inner {
            BackendImpl::Oddio(inner) => inner.is_paused(sound),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.is_paused(sound),
        }
    }

    #[inline]
    fn set_volume(&mut self, sound: u64, volume: f32) {
        match &mut self.inner {
            BackendImpl::Oddio(inner) => inner.set_volume(sound, volume),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.set_volume(sound, volume),
        }
    }

    #[inline]
    fn volume(&self, sound: u64) -> f32 {
        match &self.inner {
            BackendImpl::Oddio(inner) => inner.volume(sound),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.volume(sound),
        }
    }

    #[inline]
    fn clean(&mut self, sources: &[u64], sounds: &[u64]) {
        match &mut self.inner {
            BackendImpl::Oddio(inner) => inner.clean(sources, sounds),
            #[cfg(target_arch = "wasm32")]
            BackendImpl::Dummy(inner) => inner.clean(sources, sounds),
        }
    }
}

pub struct InnerBackend {
    source_id_count: u64,
    sound_id_count: u64,
    mixer_handle: Handle<Gain<Mixer<[f32; 2]>>>,
    _stream: cpal::Stream,
    sources: HashMap<u64, Arc<Frames<[f32; 2]>>>,
    sounds: HashMap<u64, AudioInfo>,
    volume: f32,
}

impl InnerBackend {
    pub fn new() -> Result<Self, String> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or("No output device available")?;

        let sample_rate = device
            .default_output_config()
            .map_err(|e| format!("{:?}", e))?
            .sample_rate();

        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate,
            buffer_size: BufferSize::Default,
        };

        log::debug!(
            "Audio Device {} with config {:?}",
            device.name().unwrap(),
            config
        );

        let gain = Gain::new(Mixer::new());
        let (mixer_handle, mixer) = oddio::split(gain);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _| {
                    let frames = oddio::frame_stereo(data);
                    oddio::run(&mixer, sample_rate.0, frames);
                },
                |err| {
                    log::error!("{:?}", err);
                },
            )
            .map_err(|e| format!("{:?}", e))?;

        stream.play().map_err(|e| format!("{:?}", e))?;

        Ok(Self {
            source_id_count: 0,
            sound_id_count: 0,
            mixer_handle,
            _stream: stream,
            sources: Default::default(),
            sounds: Default::default(),
            volume: 1.0,
        })
    }

    fn set_global_volume(&mut self, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        let mut gain = self.mixer_handle.control::<Gain<_>, _>();
        gain.set_gain(volume_as_gain(volume));
        self.volume = volume;
    }

    fn global_volume(&self) -> f32 {
        self.volume
    }

    fn create_source(&mut self, bytes: &[u8]) -> Result<u64, String> {
        let frames = frames_from_bytes(bytes)?;

        let id = self.source_id_count;
        self.sources.insert(id, frames);

        self.source_id_count += 1;

        Ok(id)
    }

    fn play_sound(&mut self, source: u64, volume: f32, repeat: bool) -> Result<u64, String> {
        let volume = volume.clamp(0.0, 1.0);
        let frames = self
            .sources
            .get(&source)
            .ok_or_else(|| "Invalid audio source id.".to_string())?;

        let handle = if repeat {
            let mut signal = Gain::new(Cycle::new(frames.clone()));
            signal.set_gain(volume_as_gain(volume));
            let handle = self.mixer_handle.control::<Mixer<_>, _>().play(signal);
            AudioHandle::Cycle(handle)
        } else {
            let mut signal = Gain::new(FramesSignal::from(frames.clone()));
            signal.set_gain(volume_as_gain(volume));
            let handle = self.mixer_handle.control::<Mixer<_>, _>().play(signal);
            AudioHandle::Frame(handle)
        };

        let id = self.sound_id_count;
        self.sounds.insert(id, AudioInfo { handle, volume });
        self.sound_id_count += 1;
        Ok(id)
    }

    fn pause(&mut self, sound: u64) {
        match self.sounds.get_mut(&sound) {
            None => log::warn!("Cannot pause sound, invalid id: {}", sound),
            Some(s) => s.handle.as_stop().pause(),
        }
    }

    fn resume(&mut self, sound: u64) {
        match self.sounds.get_mut(&sound) {
            None => log::warn!("Cannot resume sound, invalid id: {}", sound),
            Some(s) => s.handle.as_stop().resume(),
        }
    }

    fn stop(&mut self, sound: u64) {
        match self.sounds.get_mut(&sound) {
            None => log::warn!("Cannot stop sound, invalid id: {}", sound),
            Some(s) => s.handle.as_stop().stop(),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_stopped(&mut self, sound: u64) -> bool {
        match self.sounds.get_mut(&sound) {
            None => false,
            Some(s) => s.handle.as_stop().is_stopped(),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_paused(&mut self, sound: u64) -> bool {
        match self.sounds.get_mut(&sound) {
            None => false,
            Some(s) => s.handle.as_stop().is_paused(),
        }
    }

    fn set_volume(&mut self, sound: u64, volume: f32) {
        match self.sounds.get_mut(&sound) {
            None => log::warn!("Cannot set volume for sound: {}", sound),
            Some(s) => {
                s.volume = volume;
                s.handle.as_gain().set_gain(volume_as_gain(volume));
            }
        }
    }

    fn volume(&self, sound: u64) -> f32 {
        match self.sounds.get(&sound) {
            None => 0.0,
            Some(s) => s.volume,
        }
    }

    fn clean(&mut self, sources: &[u64], sounds: &[u64]) {
        sources.iter().for_each(|id| {
            self.sources.remove(id);
        });

        sounds.iter().for_each(|id| {
            self.sounds.remove(id);
        });

        log::debug!(
            "Audio resources cleaned: Sources({:?}) - Sounds({:?})",
            sources,
            sounds,
        );
    }
}

// convert [0.0 - 1.0] to [-100.0 - 0.0]
// with headphones I can hear -90, so I opted to to -100
fn volume_as_gain(volume: f32) -> f32 {
    let v = 1.0 - volume;
    v * 100.0 * -1.0
}

#[cfg(test)]
mod test {
    use super::volume_as_gain;

    #[test]
    fn test_volume_as_gain() {
        assert_eq!(volume_as_gain(0.0), -100.0);
        assert_eq!(volume_as_gain(0.5), -50.0);
        assert_eq!(volume_as_gain(1.0), 0.0);
    }
}
