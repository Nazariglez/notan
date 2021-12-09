use crate::{Font, TextExtension, TT};
use notan_app::assets::AssetLoader;
use notan_app::{AppBuilder, AppState, BackendSystem, BuildConfig, Graphics};

pub struct TextConfig;
impl<S, B> BuildConfig<S, B> for TextConfig
where
    S: AppState + 'static,
    B: BackendSystem,
{
    fn apply(self, builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        builder
            .add_graphic_ext(|gfx: &mut Graphics| TextExtension::new(gfx).unwrap())
            .add_loader(
                AssetLoader::new()
                    .use_parser(parse_font)
                    .extensions(&["ttf"]),
            )
    }
}

fn parse_font(id: &str, data: Vec<u8>, gfx: &mut Graphics) -> Result<Font, String> {
    let font = gfx
        .extension_mut::<TT, TextExtension>()
        .ok_or_else(|| "TextExtension is not added to Graphics")?
        .create_font(&data)?;
    log::debug!("Asset '{}' parsed as TextExtension Font", font.id);
    Ok(font)
}
