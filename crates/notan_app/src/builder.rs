#![allow(clippy::type_complexity)]

use crate::assets::{AssetLoader, Assets};
use crate::graphics::Graphics;
use crate::handlers::{
    AppCallback, AppHandler, DrawCallback, DrawHandler, EventCallback, EventHandler,
    ExtensionHandler, InitCallback, InitHandler, PluginHandler, SetupCallback,
};
use crate::parsers::*;
use crate::plugins::*;
use crate::{config::*, AppLoader, AppRunner};
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

struct Runner<S> {
    app: App,
    graphics: Graphics,
    plugins: Plugins,
    assets: Assets,
    state: S,
    current_touch_id: Option<u64>,
    event_callback: Option<EventCallback<S>>,
    update_callback: Option<AppCallback<S>>,
    draw_callback: Option<DrawCallback<S>>,
    first_loop: bool,
}

impl<S: 'static, B: BackendSystem + 'static> AppLoader for AppBuilder<S, B> {
    fn backend(&mut self) -> &mut dyn Backend {
        &mut self.backend
    }

    fn load(self: Box<Self>) -> Result<Box<dyn AppRunner>, String> {
        let AppBuilder {
            backend,
            setup_callback,
            mut plugins,
            mut assets,

            init_callback,
            update_callback,
            draw_callback,
            event_callback,
            mut plugin_callbacks,
            mut extension_callbacks,
            use_touch_as_mouse,
            ..
        } = *self;

        let mut graphics = Graphics::new(backend.get_graphics_backend())?;

        #[cfg(feature = "audio")]
        let audio = Audio::new(backend.get_audio_backend())?;
        #[cfg(feature = "audio")]
        let mut app = App::new(Box::new(backend), audio);

        #[cfg(not(feature = "audio"))]
        let mut app = App::new(Box::new(backend));

        app.window().set_touch_as_mouse(use_touch_as_mouse);

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

        Ok(Box::new(Runner {
            app,
            graphics,
            plugins,
            assets,
            state,
            current_touch_id: None,
            event_callback,
            update_callback,
            draw_callback,
            first_loop: true,
        }))
    }
}

impl<S> AppRunner for Runner<S> {
    fn run(&mut self) -> Result<FrameState, String> {
        self.app.system_timer.update();

        let win_size = self.app.window().size();
        if self.graphics.size() != win_size {
            let (width, height) = win_size;
            self.graphics.set_size(width, height);
        }

        let win_dpi = self.app.window().dpi();
        if (self.graphics.dpi() - win_dpi).abs() > f64::EPSILON {
            self.graphics.set_dpi(win_dpi);
        }

        // Manage pre frame events
        if let AppFlow::SkipFrame =
            self.plugins
                .pre_frame(&mut self.app, &mut self.assets, &mut self.graphics)?
        {
            return Ok(FrameState::Skip);
        }

        // update delta time and fps here
        self.app.timer.update();

        self.assets.tick((
            &mut self.app,
            &mut self.graphics,
            &mut self.plugins,
            &mut self.state,
        ))?;

        let delta = self.app.timer.delta_f32();

        let use_touch_as_mouse = self.app.window().touch_as_mouse();

        // Manage each event
        let mut events = self.app.backend.events_iter();
        while let Some(evt) = events.next() {
            if use_touch_as_mouse {
                touch_as_mouse(&mut self.current_touch_id, &mut events, &evt);
            }

            process_keyboard_events(&mut self.app.keyboard, &evt, delta);
            process_mouse_events(&mut self.app.mouse, &evt, delta);
            process_touch_events(&mut self.app.touch, &evt, delta);

            match self.plugins.event(&mut self.app, &mut self.assets, &evt)? {
                AppFlow::Skip => {}
                AppFlow::Next => {
                    if let Some(cb) = &self.event_callback {
                        cb.exec(
                            &mut self.app,
                            &mut self.assets,
                            &mut self.plugins,
                            &mut self.state,
                            evt,
                        );
                    }
                }
                AppFlow::SkipFrame => return Ok(FrameState::Skip),
            }
        }

        // Manage update callback
        match self.plugins.update(&mut self.app, &mut self.assets)? {
            AppFlow::Skip => {}
            AppFlow::Next => {
                if let Some(cb) = &self.update_callback {
                    cb.exec(
                        &mut self.app,
                        &mut self.assets,
                        &mut self.plugins,
                        &mut self.state,
                    );
                }
            }
            AppFlow::SkipFrame => return Ok(FrameState::Skip),
        }

        // Manage draw callback
        match self
            .plugins
            .draw(&mut self.app, &mut self.assets, &mut self.graphics)?
        {
            AppFlow::Skip => {}
            AppFlow::Next => {
                if let Some(cb) = &self.draw_callback {
                    cb.exec(
                        &mut self.app,
                        &mut self.assets,
                        &mut self.graphics,
                        &mut self.plugins,
                        &mut self.state,
                    );
                }
            }
            AppFlow::SkipFrame => return Ok(FrameState::Skip),
        }

        // call next frame in lazy mode if user is pressing mouse or keyboard
        if self.app.window().lazy_loop() {
            let mouse_down = !self.app.mouse.down.is_empty();
            let key_down = !self.app.keyboard.down.is_empty();
            if mouse_down || key_down {
                self.app.window().request_frame();
            }
        }

        clear_mouse(&mut self.app.mouse);
        clear_keyboard(&mut self.app.keyboard);

        // Manage post frame event
        let _ = self
            .plugins
            .post_frame(&mut self.app, &mut self.assets, &mut self.graphics)?;

        // Clean possible dropped resources on the backend
        self.graphics.clean();
        #[cfg(feature = "audio")]
        self.app.audio.clean();

        // dispatch Event::Exit before close the app
        if self.app.closed {
            let evt = Event::Exit;
            let _ = self.plugins.event(&mut self.app, &mut self.assets, &evt)?;
            if let Some(cb) = &self.event_callback {
                cb.exec(
                    &mut self.app,
                    &mut self.assets,
                    &mut self.plugins,
                    &mut self.state,
                    evt,
                );
            }
        }

        // Using lazy loop we need to draw 2 frames at the beginning to avoid
        // a blank window when the buffer is swapped
        if !self.app.closed && self.app.window().lazy_loop() && self.first_loop {
            self.first_loop = false;
            self.app.window().request_frame();
        }

        Ok(FrameState::End)
    }

    fn app(&self) -> &App {
        &self.app
    }

    fn app_mut(&mut self) -> &mut App {
        &mut self.app
    }
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

        let mut runner = builder.backend.runner();
        let window_config = builder.window.clone();
        runner.run(Box::new(builder), window_config)
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
