use crate::app::{App, AppState};
use crate::events::Event;
use crate::plugins::{Plugin, Plugins};

pub enum AppCallback<S> {
    Empty(Box<Fn()>),

    A(Box<Fn(&mut App)>),
    AS(Box<Fn(&mut App, &mut S)>),
    AP(Box<Fn(&mut App, &mut Plugins)>),
    APS(Box<Fn(&mut App, &mut Plugins, &mut S)>),

    P(Box<Fn(&mut Plugins)>),
    PS(Box<Fn(&mut Plugins, &mut S)>),

    S(Box<Fn(&mut S)>),
}

impl<State> AppCallback<State> {
    pub(crate) fn exec(&self, app: &mut App, plugins: &mut Plugins, state: &mut State) {
        use AppCallback::*;
        match self {
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

pub trait AppHandler<S, Params> {
    fn callback(self) -> AppCallback<S>;
}

impl<F, S> AppHandler<S, ()> for F
where
    F: Fn() + 'static,
{
    fn callback(self) -> AppCallback<S> {
        AppCallback::Empty(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut App)> for F
where
    F: Fn(&mut App) + 'static,
{
    fn callback(self) -> AppCallback<S> {
        AppCallback::A(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut App, &mut S)> for F
where
    F: Fn(&mut App, &mut S) + 'static,
    S: AppState,
{
    fn callback(self) -> AppCallback<S> {
        AppCallback::AS(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut App, &mut Plugins)> for F
where
    F: Fn(&mut App, &mut Plugins) + 'static,
{
    fn callback(self) -> AppCallback<S> {
        AppCallback::AP(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut App, &mut Plugins, &mut S)> for F
where
    F: Fn(&mut App, &mut Plugins, &mut S) + 'static,
    S: AppState,
{
    fn callback(self) -> AppCallback<S> {
        AppCallback::APS(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut Plugins)> for F
where
    F: Fn(&mut Plugins) + 'static,
{
    fn callback(self) -> AppCallback<S> {
        AppCallback::P(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut Plugins, &mut S)> for F
where
    F: Fn(&mut Plugins, &mut S) + 'static,
    S: AppState,
{
    fn callback(self) -> AppCallback<S> {
        AppCallback::PS(Box::new(self))
    }
}

impl<F, S> AppHandler<S, (&mut S)> for F
where
    F: Fn(&mut S) + 'static,
    S: AppState,
{
    fn callback(self) -> AppCallback<S> {
        AppCallback::S(Box::new(self))
    }
}

pub enum EventCallback<S> {
    E(Box<Fn(Event)>),

    AE(Box<Fn(&mut App, Event)>),
    ASE(Box<Fn(&mut App, &mut S, Event)>),
    APE(Box<Fn(&mut App, &mut Plugins, Event)>),
    APSE(Box<Fn(&mut App, &mut Plugins, &mut S, Event)>),

    PE(Box<Fn(&mut Plugins, Event)>),
    PSE(Box<Fn(&mut Plugins, &mut S, Event)>),

    SE(Box<Fn(&mut S, Event)>),
}

impl<State> EventCallback<State> {
    pub(crate) fn exec(
        &self,
        app: &mut App,
        plugins: &mut Plugins,
        state: &mut State,
        event: Event,
    ) {
        use EventCallback::*;
        match self {
            E(cb) => cb(event),

            AE(cb) => cb(app, event),
            ASE(cb) => cb(app, state, event),
            APE(cb) => cb(app, plugins, event),
            APSE(cb) => cb(app, plugins, state, event),

            PE(cb) => cb(plugins, event),
            PSE(cb) => cb(plugins, state, event),

            SE(cb) => cb(state, event),
        }
    }
}

pub trait EventHandler<S, Params> {
    fn callback(self) -> EventCallback<S>;
}

impl<F, S> EventHandler<S, (Event)> for F
where
    F: Fn(Event) + 'static,
{
    fn callback(self) -> EventCallback<S> {
        EventCallback::E(Box::new(self))
    }
}

impl<F, S> EventHandler<S, (&mut App, Event)> for F
where
    F: Fn(&mut App, Event) + 'static,
{
    fn callback(self) -> EventCallback<S> {
        EventCallback::AE(Box::new(self))
    }
}

impl<F, S> EventHandler<S, (&mut App, &mut S, Event)> for F
where
    F: Fn(&mut App, &mut S, Event) + 'static,
    S: AppState,
{
    fn callback(self) -> EventCallback<S> {
        EventCallback::ASE(Box::new(self))
    }
}

impl<F, S> EventHandler<S, (&mut App, &mut Plugins, Event)> for F
where
    F: Fn(&mut App, &mut Plugins, Event) + 'static,
{
    fn callback(self) -> EventCallback<S> {
        EventCallback::APE(Box::new(self))
    }
}

impl<F, S> EventHandler<S, (&mut App, &mut Plugins, &mut S, Event)> for F
where
    F: Fn(&mut App, &mut Plugins, &mut S, Event) + 'static,
    S: AppState,
{
    fn callback(self) -> EventCallback<S> {
        EventCallback::APSE(Box::new(self))
    }
}

impl<F, S> EventHandler<S, (&mut Plugins, Event)> for F
where
    F: Fn(&mut Plugins, Event) + 'static,
{
    fn callback(self) -> EventCallback<S> {
        EventCallback::PE(Box::new(self))
    }
}

impl<F, S> EventHandler<S, (&mut Plugins, &mut S, Event)> for F
where
    F: Fn(&mut Plugins, &mut S, Event) + 'static,
    S: AppState,
{
    fn callback(self) -> EventCallback<S> {
        EventCallback::PSE(Box::new(self))
    }
}

impl<F, S> EventHandler<S, (&mut S, Event)> for F
where
    F: Fn(&mut S, Event) + 'static,
    S: AppState,
{
    fn callback(self) -> EventCallback<S> {
        EventCallback::SE(Box::new(self))
    }
}
