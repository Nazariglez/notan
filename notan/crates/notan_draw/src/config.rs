use crate::DrawExtension;
use notan_app::{AppBuilder, AppState, BackendSystem, BuildConfig, Graphics};

pub struct DrawConfig;
impl<S, B> BuildConfig<S, B> for DrawConfig
where
    S: AppState + 'static,
    B: BackendSystem,
{
    fn apply(self, builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        builder.add_graphic_ext(|gfx: &mut Graphics| DrawExtension::new(gfx).unwrap())
    }
}
