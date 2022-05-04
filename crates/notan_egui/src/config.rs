use crate::{EguiExtension, EguiPlugin};
use notan_app::{AppBuilder, AppState, BackendSystem, BuildConfig, Graphics};

pub struct EguiConfig;
impl<S, B> BuildConfig<S, B> for EguiConfig
where
    S: AppState + 'static,
    B: BackendSystem,
{
    fn apply(&self, builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        builder
            .add_plugin(EguiPlugin::default())
            .add_graphic_ext(move |gfx: &mut Graphics| EguiExtension::new(gfx).unwrap())
    }
}
