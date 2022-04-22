/// Represent the audio implementation backend
pub trait AudioBackend {
    fn create_source(&mut self, info: &AudioSourceInfo) -> Result<u64, String>;
    fn play(&mut self, source: u64, repeat: bool) -> Result<(), String>;
    // fn stop(&mut self, source: u64);
}

pub struct AudioManager {
    backend: Box<dyn AudioBackend>,
    // drop?
}

impl AudioManager {
    pub fn new(backend: Box<dyn AudioBackend>) -> Result<Self, String> {
        Ok(Self { backend })
    }

    pub fn create_audio(&mut self, bytes: &[u8]) {
        self.backend
            .create_source(&AudioSourceInfo {
                bytes: bytes.to_vec(),
                typ: AudioFileType::Vorbis,
            })
            .unwrap();
    }

    pub fn play(&mut self, id: u64) {
        self.backend.play(id, false).unwrap();
    }

    pub fn stop(&mut self, id: u64) {
        // self.backend.stop(id);
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash)]
pub enum AudioFileType {
    Mp3,
    Vorbis,
    Flac,
    Wav,
}

pub struct AudioSourceInfo {
    pub bytes: Vec<u8>,
    pub typ: AudioFileType,
}

pub struct AudioSource;
pub struct AudioSink(u64);
