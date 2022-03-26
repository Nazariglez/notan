use crate::assets::AssetLoader;
use crate::graphics::Graphics;
use notan_graphics::Texture;

pub fn create_texture_parser() -> AssetLoader {
    AssetLoader::new()
        .use_parser(parse_image)
        .extensions(&["png", "jpg", "jpeg"])
}

fn parse_image(id: &str, data: Vec<u8>, gfx: &mut Graphics) -> Result<Texture, String> {
    let texture = gfx.create_texture().from_image(&data).build()?;
    log::debug!("Asset '{}' parsed as Texture", id);
    Ok(texture)
}
