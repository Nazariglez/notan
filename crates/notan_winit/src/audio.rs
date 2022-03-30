use notan_audio::{AudioBackend, AudioFileType, AudioSourceInfo};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::collections::HashMap;
use std::io::Cursor;

pub struct RodioBackend {
    id_count: u64,
    decoders: HashMap<u64, (AudioFileType, Cursor<Vec<u8>>)>,
    _stream: OutputStream,
    handle: OutputStreamHandle,
    sink: Vec<Sink>,
}

impl RodioBackend {
    pub(crate) fn new() -> Result<Self, String> {
        let (_stream, handle) = rodio::OutputStream::try_default().map_err(|e| e.to_string())?;

        Ok(Self {
            _stream,
            handle,
            id_count: 0,
            decoders: HashMap::new(),
            sink: vec![],
        })
    }
}

impl AudioBackend for RodioBackend {
    fn create_source(&mut self, info: &AudioSourceInfo) -> Result<u64, String> {
        let cursor = std::io::Cursor::new(info.bytes.clone());

        let id = self.id_count;
        self.decoders.insert(id, (info.typ, cursor));

        self.id_count += 1;

        Ok(id)
    }

    fn play(&mut self, source: u64, repeat: bool) -> Result<(), String> {
        let (typ, cursor) = self.decoders.get(&source).ok_or("nop".to_string())?;

        let decoder = match typ {
            AudioFileType::Mp3 => Decoder::new_mp3(cursor.clone()),
            AudioFileType::Vorbis => Decoder::new_vorbis(cursor.clone()),
            AudioFileType::Flac => Decoder::new_flac(cursor.clone()),
            AudioFileType::Wav => Decoder::new_wav(cursor.clone()),
        }
        .map_err(|e| e.to_string())?;

        let sink = Sink::try_new(&self.handle).map_err(|e| e.to_string())?;
        sink.append(decoder);

        self.sink.push(sink);

        Ok(())
    }
}
