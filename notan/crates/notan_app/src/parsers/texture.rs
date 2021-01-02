use crate::assets::Loader;
use crate::graphics::{Graphics, Texture, TextureInfo};

pub fn create_texture_parser() -> Loader {
    Loader::new()
        .use_parser(parse_image)
        .from_extensions(&["png"])
}

fn parse_image(id: &str, data: Vec<u8>, gfx: &mut Graphics) -> Result<Texture, String> {
    let info = TextureInfo::from_image(&data)?;
    let texture = gfx.create_texture(info)?;
    notan_log::debug!("Asset '{}' parsed as Texture", id);
    Ok(texture)
}
