use std::io::Cursor;
use symphonia::core::io::MediaSourceStream;
use symphonia::default::get_probe;

pub(crate) fn decode_bytes(bytes: Vec<u8>) -> Result<(Vec<u32>, u32), String> {
    let cursor = Box::new(Cursor::new(bytes));
    let media = MediaSourceStream::new(cursor, Default::default());

    let format = get_probe()
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
}
