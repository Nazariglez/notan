use crate::{CreateDraw, DrawExtension};
use notan_app::assets::AssetLoader;
use notan_app::{AppBuilder, AppState, BackendSystem, BuildConfig, Graphics};
use notan_glyph::Font;

pub struct DrawConfig;
impl<S, B> BuildConfig<S, B> for DrawConfig
where
    S: AppState + 'static,
    B: BackendSystem,
{
    fn apply(self, builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        builder
            .add_graphic_ext(|gfx: &mut Graphics| DrawExtension::new(gfx).unwrap())
            .add_loader(
                AssetLoader::new()
                    .use_parser(parse_font)
                    .extensions(&["ttf"]),
            )
    }
}

fn parse_font(id: &str, data: Vec<u8>, gfx: &mut Graphics) -> Result<Font, String> {
    let font = gfx.create_font(&data)?;
    notan_log::debug!("Asset '{}' parsed as Draw Font", id);
    Ok(font)
}
