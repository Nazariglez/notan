use oddio::Frames;
use std::io::Cursor;
use std::io::ErrorKind::UnexpectedEof;
use std::sync::Arc;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::Decoder;
use symphonia::core::errors::Error::IoError;
use symphonia::core::formats::{FormatReader, Packet};
use symphonia::core::io::MediaSourceStream;
use symphonia::default::{get_codecs, get_probe};

pub(crate) fn frames_from_bytes(bytes: &[u8]) -> Result<Arc<Frames<[f32; 2]>>, String> {
    let (mut samples, sample_rate) = decode_bytes(bytes.to_vec())?;
    let stereo = oddio::frame_stereo(&mut samples);
    Ok(Frames::from_slice(sample_rate, stereo))
}

fn decode_bytes(bytes: Vec<u8>) -> Result<(Vec<f32>, u32), String> {
    let cursor = Box::new(Cursor::new(bytes));
    let media = MediaSourceStream::new(cursor, Default::default());

    let mut format = get_probe()
        .format(
            &Default::default(),
            media,
            &Default::default(),
            &Default::default(),
        )
        .map_err(|e| format!("Cannot parse audio file: {}", e))?
        .format;

    let track = format
        .default_track()
        .ok_or_else(|| "Missing default track".to_string())?;

    let track_id = track.id;

    let mut decoder = get_codecs()
        .make(&track.codec_params, &Default::default())
        .map_err(|e| format!("Cannot get decoder: {}", e))?;

    let sample_rate = decoder
        .codec_params()
        .sample_rate
        .ok_or_else(|| "Cannot get sample rate".to_string())?;
    let samples = get_samples(&mut decoder, &mut format, track_id)?;
    Ok((samples, sample_rate))
}

fn get_samples(
    decoder: &mut Box<dyn Decoder>,
    format: &mut Box<dyn FormatReader>,
    track_id: u32,
) -> Result<Vec<f32>, String> {
    let mut samples = vec![];
    loop {
        match format.next_packet() {
            Ok(packet) => {
                if packet.track_id() != track_id {
                    continue;
                }

                decode_packet(&mut samples, decoder, packet)?;
            }
            Err(err) => {
                if let IoError(err) = err {
                    if err.kind() != UnexpectedEof {
                        let e = format!("Error decoding: {}", err);
                        log::error!("{}", e);
                        return Err(e);
                    } else {
                        break;
                    }
                }
            }
        };
    }

    Ok(samples)
}

fn decode_packet(
    samples: &mut Vec<f32>,
    decoder: &mut Box<dyn Decoder>,
    packet: Packet,
) -> Result<(), String> {
    match decoder.decode(&packet) {
        Ok(buffer_ref) => {
            let capacity = buffer_ref.capacity() as _;
            let mut sample_buffer = SampleBuffer::<f32>::new(capacity, *buffer_ref.spec());
            sample_buffer.copy_interleaved_ref(buffer_ref);
            sample_buffer
                .samples()
                .iter()
                .for_each(|sample| samples.push(*sample));

            Ok(())
        }
        Err(err) => Err(format!("Error decoding: {}", err)),
    }
}
