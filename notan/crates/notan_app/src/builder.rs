use crate::assets::{AssetLoader, AssetManager};
use crate::config::*;
use crate::handlers::{AppCallback, AppHandler, EventCallback, EventHandler};
use crate::plugins::*;
use crate::{App, Backend, BackendSystem};
use notan_log as log;

//TODO read this https://floooh.github.io/2017/05/15/oryol-spirv.html

/// Configurations used at build time
pub trait BuildConfig<S, B>
where
    B: Backend,
{
    /// Applies the configuration on the builder
    fn apply(self, builder: AppBuilder<S, B>) -> AppBuilder<S, B>;
}

/// The builder is charge of create and configure the application
pub struct AppBuilder<S, B> {
    state_callback: fn(&mut App, &mut AssetManager) -> S,
    // state: S,
    backend: B,

    plugins: Plugins,
    assets: AssetManager,

    init_callback: Option<AppCallback<S>>,
    update_callback: Option<AppCallback<S>>,
    draw_callback: Option<AppCallback<S>>,
    event_callback: Option<EventCallback<S>>,

    pub window: WindowConfig,
}

impl<S, B> AppBuilder<S, B>
where
    S: 'static,
    B: BackendSystem + 'static,
{
    /// Creates a new instance of the builder
    pub fn new(state_callback: fn(&mut App, &mut AssetManager) -> S, backend: B) -> Self {
        AppBuilder {
            state_callback,
            backend,
            plugins: Plugins::new(),
            assets: AssetManager::new(),
            init_callback: None,
            update_callback: None,
            draw_callback: None,
            event_callback: None,
            window: Default::default(),
        }
    }

    /// Applies a configuration
    pub fn set_config<C>(mut self, config: C) -> Self
    where
        C: BuildConfig<S, B>,
    {
        config.apply(self)
    }

    /// Sets a callback used before the application loop starts running
    pub fn initialize<H, Params>(mut self, handler: H) -> Self
    where
        H: AppHandler<S, Params>,
    {
        self.init_callback = Some(handler.callback());
        self
    }

    /// Sets a callback used on each frame
    pub fn update<H, Params>(mut self, handler: H) -> Self
    where
        H: AppHandler<S, Params>,
    {
        self.update_callback = Some(handler.callback());
        self
    }

    /// Sets a callback to be used on each event
    pub fn event<H, Params>(mut self, handler: H) -> Self
    where
        H: EventHandler<S, Params>,
    {
        self.event_callback = Some(handler.callback());
        self
    }

    /// Sets a plugin that can alter or control the app
    pub fn set_plugin<P: Plugin>(mut self, mut plugin: P) -> Self {
        plugin.build(&mut self);
        self.plugins.set(plugin);
        self
    }

    /// Adds a new [AssetLoader]
    pub fn add_loader<L>(mut self) -> Self
    where
        L: AssetLoader + Default + 'static,
    {
        self.assets.add_loader::<L>();
        self
    }

    /// Creates and run the application
    pub fn build(mut self) -> Result<(), String> {
        let AppBuilder {
            mut backend,
            state_callback,
            mut plugins,
            mut assets,

            init_callback,
            update_callback,
            draw_callback,
            event_callback,
            window,
            ..
        } = self;

        let load_file = backend.get_file_loader();
        let initialize = backend.initialize(window)?;

        let mut app = App::new(Box::new(backend));
        let mut state = (state_callback)(&mut app, &mut assets);

        plugins.init(&mut app).map(|flow| match flow {
            AppFlow::Next => Ok(()),
            _ => Err(format!(
                "Aborted application loop because a plugin returns on the init method AppFlow::{:?} instead of AppFlow::Next",
                flow
            )),
        })?;

        if let Some(cb) = &init_callback {
            cb.exec(&mut app, &mut assets, &mut plugins, &mut state);
        }

        if let Err(e) = initialize(app, state, move |mut app, mut state| {
            // Manage pre frame events
            match plugins.pre_frame(&mut app)? {
                AppFlow::SkipFrame => return Ok(()),
                _ => {}
            }

            assets.tick(&mut app);
            app.tick(); //todo delta here?

            // Manage each event
            for evt in app.backend.events_iter() {
                match plugins.event(&mut app, &evt)? {
                    AppFlow::Skip => {}
                    AppFlow::Next => {
                        if let Some(cb) = &event_callback {
                            cb.exec(&mut app, &mut assets, &mut plugins, &mut state, evt);
                        }
                    }
                    AppFlow::SkipFrame => return Ok(()),
                }
            }

            // Manage update callback
            match plugins.update(&mut app)? {
                AppFlow::Skip => {}
                AppFlow::Next => {
                    if let Some(cb) = &update_callback {
                        cb.exec(&mut app, &mut assets, &mut plugins, &mut state);
                    }
                }
                AppFlow::SkipFrame => return Ok(()),
            }

            // Manage draw callback
            match plugins.draw(&mut app)? {
                AppFlow::Skip => {}
                AppFlow::Next => {
                    if let Some(cb) = &draw_callback {
                        cb.exec(&mut app, &mut assets, &mut plugins, &mut state);
                    }
                }
                AppFlow::SkipFrame => return Ok(()),
            }

            app.mouse.clear();
            app.keyboard.clear();

            // Manage post frame event
            match plugins.post_frame(&mut app)? {
                _ => {}
            }

            Ok(())
        }) {
            log::error!("{}", e);
        }

        Ok(())
    }
}
