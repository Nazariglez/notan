use crate::{runner, App};
use notan_core::events;
use notan_core::window::{NotanWindow, WindowAction, WindowConfig, WindowEvent};
use notan_core::{AppBuilder, AppState, BuildConfig};

pub struct AppConfig {
    main_window: Option<WindowConfig>,
    auto_redraw: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            main_window: Some(Default::default()),
            auto_redraw: true,
        }
    }
}

impl AppConfig {
    pub fn with_windowless(mut self) -> Self {
        self.main_window = None;
        self
    }

    pub fn with_window(mut self, attrs: WindowConfig) -> Self {
        self.main_window = Some(attrs);
        self
    }
}

impl<S: AppState> BuildConfig<S> for AppConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        let mut app = App::new()?;

        // Initialize main windows if is not windowless mode
        if let Some(attrs) = self.main_window.take() {
            let id = app.create_window(attrs)?;
            log::info!("Window '{:?}' created.", id);
        }

        // Call request_draw on each frame
        let builder = if self.auto_redraw {
            builder.on(|_: &events::UpdateEvent, app: &mut App| {
                app.windows_mut().for_each(|win| win.request_redraw())
            })
        } else {
            builder
        };

        // Read windows event to set main window and close app when all windows are closed
        let builder = builder.on(|evt: &WindowEvent, app: &mut App| match evt.action {
            WindowAction::Close => {
                if app.window_ids().is_empty() {
                    app.exit();
                }
            }
            _ => {}
        });

        // let's add the windows plugin
        Ok(builder.add_plugin(app).with_runner(runner))
    }
}
