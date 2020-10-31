use crate::plugins::*;
use crate::{App, Backend, BackendSystem};
use std::sync::Arc;

pub struct AppBuilder<S, B> {
    state: S,
    backend: B,
    plugins: Plugins,
    pub window: String,
    init_callback: Option<BuildCallback<S>>,
    update_callback: Option<BuildCallback<S>>,
    draw_callback: Option<BuildCallback<S>>,
    event_callback: Option<BuildCallback<S>>,
}

impl<S, B> AppBuilder<S, B>
where
    S: 'static,
    B: BackendSystem + 'static,
{
    pub fn new(state: S, backend: B) -> Self {
        AppBuilder {
            state,
            backend,
            plugins: Plugins::new(),
            window: "Yeah".to_string(),
            init_callback: None,
            update_callback: None,
            draw_callback: None,
            event_callback: None,
        }
    }
    //
    // pub fn set_config(mut self, config: &dyn AppConfig<S>) -> Self {
    //     config.apply(&mut self);
    //     self
    // }

    pub fn initialize<H, Params>(mut self, handler: H) -> Self
    where
        H: AppHandler<S, Params>,
    {
        self.init_callback = Some(handler.callback());
        self
    }

    pub fn update<H, Params>(mut self, handler: H) -> Self
    where
        H: AppHandler<S, Params>,
    {
        self.update_callback = Some(handler.callback());
        self
    }

    pub fn set_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        self.plugins.set(plugin);
        self
    }

    pub fn build(mut self) -> Result<(), String> {
        let AppBuilder {
            mut backend,
            mut state,
            mut plugins,
            init_callback,
            update_callback,
            draw_callback,
            event_callback,
            ..
        } = self;

        let initialize = backend.initialize()?;
        let mut app = App::new(Box::new(backend));

        plugins.init(&mut app);
        if let Some(cb) = &init_callback {
            cb.exec(&mut app, &mut plugins, &mut state);
        }

        //TODO manage plugin error
        initialize(app, state, move |mut app, mut state| {
            plugins.pre_frame(&mut app);
            app.tick();

            app.backend.events_iter().for_each(|evt| {
                app.mouse.process_events(&evt, app.delta);
                app.keyboard.process_events(&evt, app.delta);

                plugins.event(&mut app, evt);
                if let Some(cb) = &event_callback {
                    cb.exec(&mut app, &mut plugins, &mut state); //pass event
                }
            });

            plugins.update(&mut app);
            if let Some(cb) = &update_callback {
                cb.exec(&mut app, &mut plugins, &mut state);
            }

            //TODO check frame here?
            plugins.draw(&mut app);
            if let Some(cb) = &draw_callback {
                cb.exec(&mut app, &mut plugins, &mut state);
            }

            app.mouse.clear();
            app.keyboard.clear();
            plugins.post_frame(&mut app);
        })?;

        Ok(())
    }
}

pub enum BuildCallback<S> {
    Empty(Box<Fn()>),

    A(Box<Fn(&mut App)>),
    AS(Box<Fn(&mut App, &mut S)>),
    AP(Box<Fn(&mut App, &mut Plugins)>),
    APS(Box<Fn(&mut App, &mut Plugins, &mut S)>),

    P(Box<Fn(&mut Plugins)>),
    PS(Box<Fn(&mut Plugins, &mut S)>),

    S(Box<Fn(&mut S)>),
}

impl<State> BuildCallback<State> {
    fn exec(&self, app: &mut App, plugins: &mut Plugins, state: &mut State) {
        use BuildCallback::*;
        match &*self {
            Empty(cb) => cb(),
            A(cb) => cb(app),
            AS(cb) => cb(app, state),
            AP(cb) => cb(app, plugins),
            APS(cb) => cb(app, plugins, state),

            P(cb) => cb(plugins),
            PS(cb) => cb(plugins, state),

            S(cb) => cb(state),
        }
    }
}

pub trait AppState {}
//impl<F> AppState for F {}

pub trait AppHandler<S, Params> {
    fn callback(self) -> BuildCallback<S>;
}

impl<F, S> AppHandler<S, ()> for F
where
    F: Fn() + 'static,
{
    fn callback(self) -> BuildCallback<S> {
        BuildCallback::Empty(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut App)> for F
where
    F: Fn(&mut App) + 'static,
{
    fn callback(self) -> BuildCallback<S> {
        BuildCallback::A(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut App, &mut S)> for F
where
    F: Fn(&mut App, &mut S) + 'static,
    S: AppState,
{
    fn callback(self) -> BuildCallback<S> {
        BuildCallback::AS(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut App, &mut Plugins)> for F
where
    F: Fn(&mut App, &mut Plugins) + 'static,
{
    fn callback(self) -> BuildCallback<S> {
        BuildCallback::AP(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut App, &mut Plugins, &mut S)> for F
where
    F: Fn(&mut App, &mut Plugins, &mut S) + 'static,
    S: AppState,
{
    fn callback(self) -> BuildCallback<S> {
        BuildCallback::APS(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut Plugins)> for F
where
    F: Fn(&mut Plugins) + 'static,
{
    fn callback(self) -> BuildCallback<S> {
        BuildCallback::P(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut Plugins, &mut S)> for F
where
    F: Fn(&mut Plugins, &mut S) + 'static,
    S: AppState,
{
    fn callback(self) -> BuildCallback<S> {
        BuildCallback::PS(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut S)> for F
where
    F: Fn(&mut S) + 'static,
    S: AppState,
{
    fn callback(self) -> BuildCallback<S> {
        BuildCallback::S(Box::new(self))
    }
}
