#![allow(clippy::type_complexity)]

use crate::assets::{AssetLoader, Assets};
use crate::config::*;
use crate::graphics::Graphics;
use crate::handlers::{
    AppCallback, AppHandler, DrawCallback, DrawHandler, EventCallback, EventHandler,
    ExtensionHandler, InitCallback, InitHandler, PluginHandler, SetupCallback,
};
use crate::parsers::*;
use crate::plugins::*;
use crate::{App, Backend, BackendSystem, FrameState, GfxExtension, GfxRenderer};
use indexmap::IndexMap;
#[cfg(feature = "audio")]
use notan_audio::Audio;
use notan_core::events::{Event, EventIterator};
use notan_core::mouse::MouseButton;
use notan_input::internals::{
    clear_keyboard, clear_mouse, process_keyboard_events, process_mouse_events,
    process_touch_events,
};

pub use crate::handlers::SetupHandler;

/// Configurations used at build time
pub trait BuildConfig<S, B>
where
    B: Backend,
{
    /// Applies the configuration on the builder
    fn apply(&self, builder: AppBuilder<S, B>) -> AppBuilder<S, B>;

    /// This config will be applied before the app is initiated not when is set
    fn late_evaluation(&self) -> bool {
        false
    }
}

/// The builder is charge of create and configure the application
pub struct AppBuilder<S, B> {
    setup_callback: SetupCallback<S>,
    backend: B,

    plugins: Plugins,
    assets: Assets,

    init_callback: Option<InitCallback<S>>,
    update_callback: Option<AppCallback<S>>,
    draw_callback: Option<DrawCallback<S>>,
    event_callback: Option<EventCallback<S>>,

    plugin_callbacks: Vec<Box<dyn FnOnce(&mut App, &mut Assets, &mut Graphics, &mut Plugins)>>,
    extension_callbacks: Vec<Box<dyn FnOnce(&mut App, &mut Assets, &mut Graphics, &mut Plugins)>>,

    late_config: Option<IndexMap<std::any::TypeId, Box<dyn BuildConfig<S, B>>>>,

    use_touch_as_mouse: bool,

    pub(crate) window: WindowConfig,
}

impl<S, B> AppBuilder<S, B>
where
    S: 'static,
    B: BackendSystem + 'static,
{
    /// Creates a new instance of the builder
    pub fn new<H, Params>(setup: H, backend: B) -> Self
    where
        H: SetupHandler<S, Params>,
    {
        let builder = AppBuilder {
            backend,
            plugins: Default::default(),
            assets: Assets::new(),
            setup_callback: setup.callback(),
            init_callback: None,
            update_callback: None,
            draw_callback: None,
            event_callback: None,
            plugin_callbacks: vec![],
            extension_callbacks: vec![],
            window: Default::default(),
            late_config: Some(Default::default()),
            use_touch_as_mouse: true,
        };

        builder.default_loaders()
    }

    #[allow(unreachable_code)]
    fn default_loaders(self) -> Self {
        #[cfg(feature = "audio")]
        {
            self.add_loader(create_texture_parser())
                .add_loader(create_audio_parser())
        }

        #[cfg(not(feature = "audio"))]
        {
            self.add_loader(create_texture_parser())
        }
    }

    /// Converts touch events as mouse events
    pub fn touch_as_mouse(mut self, enabled: bool) -> Self {
        self.use_touch_as_mouse = enabled;
        self
    }

    /// Applies a configuration
    pub fn add_config<C>(mut self, config: C) -> Self
    where
        C: BuildConfig<S, B> + 'static,
    {
        if config.late_evaluation() {
            if let Some(late_config) = &mut self.late_config {
                let typ = std::any::TypeId::of::<C>();
                late_config.insert(typ, Box::new(config));
            }

            self
        } else {
            config.apply(self)
        }
    }

    /// Sets a callback used before the application loop starts running
    pub fn initialize<H, Params>(mut self, handler: H) -> Self
    where
        H: InitHandler<S, Params>,
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

    /// Sets a callback executed after each update to draw
    pub fn draw<H, Params>(mut self, handler: H) -> Self
    where
        H: DrawHandler<S, Params>,
    {
        self.draw_callback = Some(handler.callback());
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
    pub fn add_plugin<P: Plugin + 'static>(mut self, mut plugin: P) -> Self {
        plugin.build(&mut self);
        self.plugins.add(plugin);
        self
    }

    /// Adds a plugin using parameters from the app
    pub fn add_plugin_with<P, H, Params>(mut self, handler: H) -> Self
    where
        P: Plugin + 'static,
        H: PluginHandler<P, Params> + 'static,
    {
        let cb =
            move |app: &mut App, assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins| {
                let p = handler.callback().exec(app, assets, gfx, plugins);
                plugins.add(p);
            };
        self.plugin_callbacks.push(Box::new(cb));
        self
    }

    /// Adds an extension using parameters from the app
    pub fn add_graphic_ext<R, E, H, Params>(mut self, handler: H) -> Self
    where
        R: GfxRenderer,
        E: GfxExtension<R> + 'static,
        H: ExtensionHandler<R, E, Params> + 'static,
    {
        let cb =
            move |app: &mut App, assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins| {
                let e = handler.callback().exec(app, assets, gfx, plugins);
                gfx.add_extension(e);
            };
        self.extension_callbacks.push(Box::new(cb));
        self
    }

    /// Adds a new [AssetLoader]
    pub fn add_loader(mut self, loader: AssetLoader) -> Self {
        self.assets.add_loader(loader);
        self
    }

    /// Creates and run the application
    pub fn build(self) -> Result<(), String> {
        let mut builder = self;
        if let Some(late_config) = builder.late_config.take() {
            for (_, config) in late_config {
                builder = config.apply(builder);
            }
        }

        let AppBuilder {
            mut backend,
            setup_callback,
            mut plugins,
            mut assets,

            init_callback,
            update_callback,
            draw_callback,
            event_callback,
            mut plugin_callbacks,
            mut extension_callbacks,
            window,
            use_touch_as_mouse,
            ..
        } = builder;

        let initialize = backend.initialize(window)?;

        let mut graphics = Graphics::new(backend.get_graphics_backend())?;

        #[cfg(feature = "audio")]
        let audio = Audio::new(backend.get_audio_backend())?;
        #[cfg(feature = "audio")]
        let mut app = App::new(Box::new(backend), audio);
        app.window().set_touch_as_mouse(use_touch_as_mouse);

        #[cfg(not(feature = "audio"))]
        let mut app = App::new(Box::new(backend));

        let (width, height) = app.window().size();
        let win_dpi = app.window().dpi();
        graphics.set_size(width, height);
        graphics.set_dpi(win_dpi);

        // add graphics extensions
        extension_callbacks.reverse();
        while let Some(cb) = extension_callbacks.pop() {
            cb(&mut app, &mut assets, &mut graphics, &mut plugins);
        }

        // add plugins
        plugin_callbacks.reverse();
        while let Some(cb) = plugin_callbacks.pop() {
            cb(&mut app, &mut assets, &mut graphics, &mut plugins);
        }

        // create the state
        let mut state = setup_callback.exec(&mut app, &mut assets, &mut graphics, &mut plugins);

        // init callback from plugins
        let _ = plugins.init(&mut app, &mut assets, &mut graphics).map(|flow| match flow {
            AppFlow::Next => Ok(()),
            _ => Err(format!(
                "Aborted application loop because a plugin returns on the init method AppFlow::{flow:?} instead of AppFlow::Next",
            )),
        })?;

        // app init life event
        if let Some(cb) = init_callback {
            cb.exec(&mut app, &mut assets, &mut plugins, &mut state);
        }

        let mut current_touch_id: Option<u64> = None;

        let mut first_loop = true;
        if let Err(e) = initialize(app, state, move |app, mut state| {
            // update system delta time and fps here
            app.system_timer.update();

            let win_size = app.window().size();
            if graphics.size() != win_size {
                let (width, height) = win_size;
                graphics.set_size(width, height);
            }

            let win_dpi = app.window().dpi();
            if (graphics.dpi() - win_dpi).abs() > f64::EPSILON {
                graphics.set_dpi(win_dpi);
            }

            // Manage pre frame events
            if let AppFlow::SkipFrame = plugins.pre_frame(app, &mut assets, &mut graphics)? {
                return Ok(FrameState::Skip);
            }

            // update delta time and fps here
            app.timer.update();

            assets.tick((app, &mut graphics, &mut plugins, &mut state))?;

            let delta = app.timer.delta_f32();

            let use_touch_as_mouse = app.window().touch_as_mouse();

            // Manage each event
            let mut events = app.backend.events_iter();
            while let Some(evt) = events.next() {
                if use_touch_as_mouse {
                    touch_as_mouse(&mut current_touch_id, &mut events, &evt);
                }

                process_keyboard_events(&mut app.keyboard, &evt, delta);
                process_mouse_events(&mut app.mouse, &evt, delta);
                process_touch_events(&mut app.touch, &evt, delta);

                match plugins.event(app, &mut assets, &evt)? {
                    AppFlow::Skip => {}
                    AppFlow::Next => {
                        if let Some(cb) = &event_callback {
                            cb.exec(app, &mut assets, &mut plugins, state, evt);
                        }
                    }
                    AppFlow::SkipFrame => return Ok(FrameState::Skip),
                }
            }

            // Manage update callback
            match plugins.update(app, &mut assets)? {
                AppFlow::Skip => {}
                AppFlow::Next => {
                    if let Some(cb) = &update_callback {
                        cb.exec(app, &mut assets, &mut plugins, state);
                    }
                }
                AppFlow::SkipFrame => return Ok(FrameState::Skip),
            }

            // Manage draw callback
            match plugins.draw(app, &mut assets, &mut graphics)? {
                AppFlow::Skip => {}
                AppFlow::Next => {
                    if let Some(cb) = &draw_callback {
                        cb.exec(app, &mut assets, &mut graphics, &mut plugins, state);
                    }
                }
                AppFlow::SkipFrame => return Ok(FrameState::Skip),
            }

            // call next frame in lazy mode if user is pressing mouse or keyboard
            if app.window().lazy_loop() {
                let mouse_down = !app.mouse.down.is_empty();
                let key_down = !app.keyboard.down.is_empty();
                if mouse_down || key_down {
                    app.window().request_frame();
                }
            }

            clear_mouse(&mut app.mouse);
            clear_keyboard(&mut app.keyboard);

            // Manage post frame event
            let _ = plugins.post_frame(app, &mut assets, &mut graphics)?;

            // Clean possible dropped resources on the backend
            graphics.clean();
            #[cfg(feature = "audio")]
            app.audio.clean();

            if app.closed {
                log::debug!("App Closed");
            }

            // Using lazy loop we need to draw 2 frames at the beginning to avoid
            // a blank window when the buffer is swapped
            if !app.closed && app.window().lazy_loop() && first_loop {
                first_loop = false;
                app.window().request_frame();
            }

            Ok(FrameState::End)
        }) {
            log::error!("{}", e);
        }

        Ok(())
    }
}

#[inline]
fn touch_as_mouse(current_touch_id: &mut Option<u64>, events: &mut EventIterator, evt: &Event) {
    match evt {
        Event::TouchStart { id, x, y } => {
            if current_touch_id.is_none() || current_touch_id.unwrap() == *id {
                *current_touch_id = Some(*id);
                events.push_front(Event::MouseDown {
                    button: MouseButton::Left,
                    x: *x as _,
                    y: *y as _,
                });
            }
        }
        Event::TouchMove { id, x, y } => {
            if let Some(last_id) = current_touch_id {
                if last_id == id {
                    events.push_front(Event::MouseMove {
                        x: *x as _,
                        y: *y as _,
                    });
                }
            }
        }
        Event::TouchEnd { id, x, y } => {
            if let Some(last_id) = current_touch_id {
                if last_id == id {
                    *current_touch_id = None;
                    events.push_front(Event::MouseUp {
                        button: MouseButton::Left,
                        x: *x as _,
                        y: *y as _,
                    });
                }
            }
        }
        Event::TouchCancel { id, x, y } => {
            if let Some(last_id) = current_touch_id {
                if last_id == id {
                    *current_touch_id = None;
                    events.push_front(Event::MouseUp {
                        button: MouseButton::Left,
                        x: *x as _,
                        y: *y as _,
                    });
                }
            }
        }
        _ => {}
    }
}
