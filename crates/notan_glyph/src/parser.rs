use crate::{Font, GlyphPlugin};
use notan_app::assets::AssetLoader;
use notan_app::Plugins;

pub fn create_font_parser() -> AssetLoader {
    AssetLoader::new()
        .use_parser(parse_font)
        .extensions(&["ttf"])
}

fn parse_font(id: &str, data: Vec<u8>, plugins: &mut Plugins) -> Result<Font, String> {
    let mut plugin = plugins
        .get_mut::<GlyphPlugin>()
        .ok_or_else(|| "Glyph plugin not found".to_string())?;
    let font = plugin.create_font(&data)?;
    notan_log::debug!("Asset '{}' parsed as Font", id);
    Ok(font)
}
