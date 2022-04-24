use crate::decoder::decode_bytes;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::BufferSize;
use hashbrown::HashMap;
use log::error;
use notan_audio::{AudioBackend, AudioSource};
use oddio::{Cycle, Frames, FramesSignal, Gain, Handle, Mixer, Stop};
use std::io::Cursor;
use std::sync::Arc;
use symphonia::core::io::MediaSourceStream;

type AudioHandle = Handle<Stop<Gain<FramesSignal<[f32; 2]>>>>;

pub struct OddioBackend {
    source_id_count: u64,
    sound_id_count: u64,
    mixer_handle: Handle<Gain<Mixer<[f32; 2]>>>,
    stream: cpal::Stream,
    sources: HashMap<u64, Arc<Frames<[f32; 2]>>>,
    sounds: HashMap<u64, AudioHandle>,
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
        })
    }
}

impl AudioBackend for OddioBackend {
    fn create_source(&mut self, bytes: &[u8]) -> Result<u64, String> {
        let (mut samples, sample_rate) = decode_bytes(bytes.to_vec())?;
        let stereo = oddio::frame_stereo(&mut samples);
        let frames = oddio::Frames::from_slice(sample_rate, &stereo);

        let id = self.source_id_count;
        self.sources.insert(id, frames);

        self.sound_id_count += 1;

        Ok(id)
    }

    fn play_sound(&mut self, source: u64) -> Result<u64, String> {
        let frames = self
            .sources
            .get(&source)
            .ok_or_else(|| "Invalid audio source id.".to_string())?;

        let signal = oddio::Gain::new(FramesSignal::from(frames.clone()), 1.0);
        let handle = self.mixer_handle.control::<Mixer<_>, _>().play(signal);
        let id = self.sound_id_count;
        self.sounds.insert(id, handle);
        self.sound_id_count += 1;
        Ok(id)
    }

    fn play(&mut self, sound: u64) {}

    fn stop(&mut self, sound: u64) {}
}
