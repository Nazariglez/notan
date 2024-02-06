use crate::gfx::Gfx;
use crate::GfxAttributes;
use notan_app2::App;
use notan_core::window::{WindowAction, WindowEvent};
use notan_core::{AppBuilder, BuildConfig, AppState};

#[derive(Default)]
pub struct GfxConfig {
    attrs: GfxAttributes,
}

impl GfxConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Use VSync mode if possible
    pub fn with_vsync(mut self, enable: bool) -> Self {
        self.attrs.vsync = enable;
        self
    }
}

impl<S: AppState + 'static> BuildConfig<S> for GfxConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        let builder = builder.on(on_window_event);

        let attrs = self.attrs;
        builder.add_plugin_with(move |app: &mut App| {
            let mut gfx = Gfx::new(attrs)?;
            if let Some(win) = app.window() {
                gfx.init_surface(win)?;
            }

            Ok(gfx)
        })
    }
}

fn on_window_event(evt: &WindowEvent, gfx: &mut Gfx, app: &mut App) {
    match evt.action {
        // when a new window is created let's initialize the surface for it
        WindowAction::Init => {
            gfx.init_surface(app.window_by_id(evt.id).unwrap()).unwrap();
        }
        WindowAction::Moved { .. } => {}
        WindowAction::Resized {
            width,
            height,
            scale_factor,
        } => {
            let w = (width as f64 * scale_factor) as u32;
            let h = (height as f64 * scale_factor) as u32;
            gfx.resize(evt.id, w, h).unwrap();
        }
        WindowAction::Minimized => {}
        WindowAction::Maximized => {}
        WindowAction::FocusGained => {}
        WindowAction::FocusLost => {}
        WindowAction::Close => {}
    }
}
