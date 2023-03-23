use notan_graphics::Texture;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Returns a HashMap containing a list of textures created from the Atlas data
pub fn create_textures_from_atlas(
    data: &[u8],
    base_texture: &Texture,
) -> Result<HashMap<String, Texture>, String> {
    let data = atlas_from_bytes(data)?;
    let mut textures = HashMap::new();
    data.frames.iter().for_each(|af| {
        textures.insert(
            af.filename.clone(),
            base_texture.with_frame(
                af.frame.x as _,
                af.frame.y as _,
                af.frame.w as _,
                af.frame.h as _,
            ),
        );
    });
    Ok(textures)
}

#[inline]
fn atlas_from_bytes(data: &[u8]) -> Result<AtlasRoot, String> {
    serde_json::from_slice(data).map_err(|e| e.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasRoot {
    frames: Vec<AtlasFrame>,
    meta: AtlasMeta,
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasFrame {
    filename: String,
    frame: AtlasRect,
    rotated: bool,
    trimmed: bool,
    #[serde(alias = "spriteSourceSize")]
    sprite_source_size: AtlasRect,
    #[serde(alias = "sourceSize")]
    source_size: AtlasSize,
    #[serde(default)]
    pivot: AtlasPoint,
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasMeta {
    app: String,
    version: String,
    image: String,
    format: String,
    size: AtlasSize,
    scale: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasPoint {
    x: f32,
    y: f32,
}

impl Default for AtlasPoint {
    fn default() -> Self {
        Self { x: 0.5, y: 0.5 }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasSize {
    w: i32,
    h: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasRect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}
