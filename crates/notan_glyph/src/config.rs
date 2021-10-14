use crate::GlyphPlugin;
use notan_app::{AppBuilder, AppState, BackendSystem, BuildConfig, Graphics};

pub struct GlyphConfig;
impl<S, B> BuildConfig<S, B> for GlyphConfig
where
    S: AppState + 'static,
    B: BackendSystem,
{
    fn apply(self, builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        builder.add_plugin_with(|gfx: &mut Graphics| GlyphPlugin::new(gfx).unwrap())
    }
}
