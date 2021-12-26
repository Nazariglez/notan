use crate::{CreateDraw, DrawExtension};
use notan_app::assets::AssetLoader;
use notan_app::{AppBuilder, AppState, BackendSystem, BuildConfig, Graphics};
use notan_text::*;

pub struct DrawConfig;
impl<S, B> BuildConfig<S, B> for DrawConfig
where
    S: AppState + 'static,
    B: BackendSystem,
{
    fn apply(self, builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        builder.add_graphic_ext(|gfx: &mut Graphics| {
            // Add text extension if necessary
            if gfx.extension::<Text, TextExtension>().is_none() {
                let text_ext = TextExtension::new(gfx).unwrap();
                gfx.add_extension(text_ext);
            }

            DrawExtension::new(gfx).unwrap()
        })
    }
}
