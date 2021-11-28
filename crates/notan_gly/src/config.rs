use crate::GlyphExtension;
use notan_app::{AppBuilder, AppState, BackendSystem, BuildConfig, Graphics};

pub struct GlyConfig;
impl<S, B> BuildConfig<S, B> for GlyConfig
where
    S: AppState + 'static,
    B: BackendSystem,
{
    fn apply(self, builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        builder.add_graphic_ext(|gfx: &mut Graphics| GlyphExtension::new(gfx))
    }
}
