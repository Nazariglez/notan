#![cfg(feature = "audio")]

use crate::assets::AssetLoader;
use crate::App;
use notan_audio::{Audio, AudioSource};

pub fn create_audio_parser() -> AssetLoader {
    AssetLoader::new()
        .use_parser(parse_audio)
        .extensions(&["mp3", "ogg", "wav", "flac"])
}

fn parse_audio(id: &str, data: Vec<u8>, app: &mut App) -> Result<AudioSource, String> {
    let source = app.audio.create_source(&data)?;
    log::debug!("Asset '{}' parsed as AudioSource", id);
    Ok(source)
}
