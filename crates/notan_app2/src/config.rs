use crate::{runner, App};
use notan_core::events;
use notan_core::window::{NotanWindow, WindowAction, WindowAttributes, WindowEvent};
use notan_core::{AppBuilder, AppState, BuildConfig};

pub struct PlatformConfig {
    main_window: Option<WindowAttributes>,
    auto_redraw: bool,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            main_window: Some(Default::default()),
            auto_redraw: true,
        }
    }
}

impl PlatformConfig {
    pub fn with_windowless(mut self) -> Self {
        self.main_window = None;
        self
    }

    pub fn with_window(mut self, attrs: WindowAttributes) -> Self {
        self.main_window = Some(attrs);
        self
    }
}

impl<S: AppState> BuildConfig<S> for PlatformConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        let mut platform = App::new();

        // Initialize main windows if is not windowless mode
        if let Some(attrs) = self.main_window.take() {
            let id = platform.create_window(attrs)?;
            log::info!("Window '{:?}' created.", id);
        }

        // Call request_draw on each frame
        let builder = if self.auto_redraw {
            builder.on(|_: &events::UpdateEvent, platform: &mut App| {
                platform.windows_mut().for_each(|win| win.request_redraw())
            })
        } else {
            builder
        };

        // Read windows event to set main window and close app when all windows are closed
        let builder = builder.on(|evt: &WindowEvent, platform: &mut App| match evt.action {
            WindowAction::Close => {
                if platform.window_ids().is_empty() {
                    platform.exit();
                }
            }
            _ => {}
        });

        // let's add the windows plugin
        Ok(builder.add_plugin(platform).with_runner(runner))
    }
}
