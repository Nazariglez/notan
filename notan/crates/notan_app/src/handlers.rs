use crate::app::{App, AppState};
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
