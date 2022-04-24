use crate::decoder::decode_bytes;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::BufferSize;
use hashbrown::HashMap;
use log::error;
use notan_audio::{AudioBackend, AudioSource};
use oddio::{Cycle, FilterHaving, Frames, FramesSignal, Gain, Handle, Mixer, Stop};
use std::io::Cursor;
use std::sync::Arc;
use symphonia::core::io::MediaSourceStream;

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

pub struct OddioBackend {
    source_id_count: u64,
    sound_id_count: u64,
    mixer_handle: Handle<Gain<Mixer<[f32; 2]>>>,
    stream: cpal::Stream,
    sources: HashMap<u64, Arc<Frames<[f32; 2]>>>,
    sounds: HashMap<u64, AudioInfo>,
    volume: f32,
}

impl OddioBackend {
    pub fn new() -> Result<Self, String> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| "No output device available")?;

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

        let (mut mixer_handle, mixer) = oddio::split(oddio::Gain::new(oddio::Mixer::new(), 1.0));

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _| {
                    let frames = oddio::frame_stereo(data);
                    oddio::run(&mixer, sample_rate.0, frames);
                },
                |err| {
                    log::error!("{}", err);
                },
            )
            .map_err(|e| format!("{:?}", e))?;

        stream.play().map_err(|e| format!("{:?}", e))?;

        Ok(Self {
            source_id_count: 0,
            sound_id_count: 0,
            mixer_handle,
            stream,
            sources: Default::default(),
            sounds: Default::default(),
            volume: 1.0,
        })
    }
}

impl AudioBackend for OddioBackend {
    fn set_global_volume(&mut self, volume: f32) {
        let v = 1.0 - volume;

        let mut gain = self.mixer_handle.control::<Gain<_>, _>();
        gain.set_gain(v * 60.0 * -1.0);
        self.volume = volume;
        println!("{} {} {}", volume, v, v * 60.0 * -1.0);
    }

    fn global_volume(&self) -> f32 {
        self.volume
    }

    fn create_source(&mut self, bytes: &[u8]) -> Result<u64, String> {
        let (mut samples, sample_rate) = decode_bytes(bytes.to_vec())?;
        let stereo = oddio::frame_stereo(&mut samples);
        let frames = oddio::Frames::from_slice(sample_rate, &stereo);

        let id = self.source_id_count;
        self.sources.insert(id, frames);

        self.sound_id_count += 1;

        Ok(id)
    }

    fn play_sound(&mut self, source: u64, repeat: bool) -> Result<u64, String> {
        let frames = self
            .sources
            .get(&source)
            .ok_or_else(|| "Invalid audio source id.".to_string())?;

        let handle = if repeat {
            let signal = oddio::Gain::new(Cycle::new(frames.clone()), 1.0);
            let handle = self.mixer_handle.control::<Mixer<_>, _>().play(signal);
            AudioHandle::Cycle(handle)
        } else {
            let signal = oddio::Gain::new(FramesSignal::from(frames.clone()), 1.0);
            let handle = self.mixer_handle.control::<Mixer<_>, _>().play(signal);
            AudioHandle::Frame(handle)
        };

        let id = self.sound_id_count;
        self.sounds.insert(
            id,
            AudioInfo {
                handle,
                volume: 1.0,
            },
        );
        self.sound_id_count += 1;
        Ok(id)
    }

    fn pause(&mut self, sound: u64) {
        match self.sounds.get_mut(&sound) {
            None => log::warn!("Cannot pause sound, invalid id: {}", sound),
            Some(s) => match &mut s.handle {
                AudioHandle::Frame(h) => h.control::<Stop<_>, _>().pause(),
                AudioHandle::Cycle(h) => h.control::<Stop<_>, _>().pause(),
            },
        }
    }

    fn resume(&mut self, sound: u64) {
        match self.sounds.get_mut(&sound) {
            None => log::warn!("Cannot resume sound, invalid id: {}", sound),
            Some(s) => match &mut s.handle {
                AudioHandle::Frame(h) => h.control::<Stop<_>, _>().resume(),
                AudioHandle::Cycle(h) => h.control::<Stop<_>, _>().resume(),
            },
        }
    }

    fn stop(&mut self, sound: u64) {
        match self.sounds.get_mut(&sound) {
            None => log::warn!("Cannot stop sound, invalid id: {}", sound),
            Some(s) => match &mut s.handle {
                AudioHandle::Frame(h) => h.control::<Stop<_>, _>().stop(),
                AudioHandle::Cycle(h) => h.control::<Stop<_>, _>().stop(),
            },
        }
    }

    fn is_stopped(&mut self, sound: u64) -> bool {
        match self.sounds.get_mut(&sound) {
            None => false,
            Some(s) => match &mut s.handle {
                AudioHandle::Frame(h) => h.control::<Stop<_>, _>().is_stopped(),
                AudioHandle::Cycle(h) => h.control::<Stop<_>, _>().is_stopped(),
            },
        }
    }

    fn is_paused(&mut self, sound: u64) -> bool {
        match self.sounds.get_mut(&sound) {
            None => false,
            Some(s) => match &mut s.handle {
                AudioHandle::Frame(h) => h.control::<Stop<_>, _>().is_paused(),
                AudioHandle::Cycle(h) => h.control::<Stop<_>, _>().is_paused(),
            },
        }
    }

    fn set_volume(&mut self, sound: u64, volume: f32) {
        let v = 1.0 - volume;

        match self.sounds.get_mut(&sound) {
            None => log::warn!("Cannot set volume for sound: {}", sound),
            Some(s) => {
                s.volume = volume;
                match &mut s.handle {
                    AudioHandle::Frame(h) => h.control::<Gain<_>, _>().set_gain(v * 60.0 * -1.0),
                    AudioHandle::Cycle(h) => h.control::<Gain<_>, _>().set_gain(v * 60.0 * -1.0),
                }
            }
        }
    }

    fn volume(&self, sound: u64) -> f32 {
        match self.sounds.get(&sound) {
            None => 0.0,
            Some(s) => s.volume,
        }
    }
}
