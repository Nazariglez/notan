use crate::draw2d::Draw2D;
use notan_app2::App;
use notan_core::{AppBuilder, AppState, BuildConfig};
use notan_gfx::Gfx;

// TODO text with subpixel, custom shaders, etc...
#[derive(Default)]
pub struct DrawConfig {}

impl DrawConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: AppState + 'static> BuildConfig<S> for DrawConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        builder.add_plugin_with(move |app: &mut App, gfx: &mut Gfx| {
            let draw = Draw2D::new(gfx)?;
            Ok(draw)
        })
    }
}
