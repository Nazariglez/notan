use crate::app::{App, AppState};
use crate::assets::AssetManager;
use crate::events::Event;
use crate::graphics::Graphics;
use crate::plugins::Plugins;

//TODO generate this enum should be easy to do with a proc_macro or something...
pub enum SetupCallback<S> {
    Empty(Box<dyn Fn() -> S>),
    A(Box<dyn Fn(&mut App) -> S>),
    M(Box<dyn Fn(&mut AssetManager) -> S>),
    AM(Box<dyn Fn(&mut App, &mut AssetManager) -> S>),
    P(Box<dyn Fn(&mut Plugins) -> S>),
    AP(Box<dyn Fn(&mut App, &mut Plugins) -> S>),
    MP(Box<dyn Fn(&mut AssetManager, &mut Plugins) -> S>),
    AMP(Box<dyn Fn(&mut App, &mut AssetManager, &mut Plugins) -> S>),
    G(Box<dyn Fn(&mut Graphics) -> S>),
    AG(Box<dyn Fn(&mut App, &mut Graphics) -> S>),
    MG(Box<dyn Fn(&mut AssetManager, &mut Graphics) -> S>),
    AMG(Box<dyn Fn(&mut App, &mut AssetManager, &mut Graphics) -> S>),
    GP(Box<dyn Fn(&mut Graphics, &mut Plugins) -> S>),
    MGP(Box<dyn Fn(&mut AssetManager, &mut Graphics, &mut Plugins) -> S>),
    AMGP(Box<dyn Fn(&mut App, &mut AssetManager, &mut Graphics, &mut Plugins) -> S>),
}

impl<State> SetupCallback<State> {
    pub(crate) fn exec(
        &self,
        app: &mut App,
        manager: &mut AssetManager,
        graphics: &mut Graphics,
        plugins: &mut Plugins,
    ) -> State {
        use SetupCallback::*;
        match self {
            Empty(cb) => cb(),
            A(cb) => cb(app),
            AM(cb) => cb(app, manager),
            AP(cb) => cb(app, plugins),
            AMP(cb) => cb(app, manager, plugins),

            M(cb) => cb(manager),
            MP(cb) => cb(manager, plugins),

            P(cb) => cb(plugins),

            G(cb) => cb(graphics),
            AG(cb) => cb(app, graphics),
            MG(cb) => cb(manager, graphics),
            AMG(cb) => cb(app, manager, graphics),
            GP(cb) => cb(graphics, plugins),
            MGP(cb) => cb(manager, graphics, plugins),
            AMGP(cb) => cb(app, manager, graphics, plugins),
        }
    }
}

pub trait SetupHandler<S, Params> {
    fn callback(self) -> SetupCallback<S>;
}

macro_rules! setup_handler {
    ($variant:expr, $($param:ident),*) => {
        #[allow(unused_parens)]
        impl<F, S> SetupHandler<S, ($(&mut $param),*)> for F
        where
            F: Fn($(&mut $param),*) -> S + 'static,
            S: AppState
        {
            fn callback(self) -> SetupCallback<S> {
                $variant(Box::new(self))
            }
        }
    }
}

setup_handler!(SetupCallback::Empty,);
setup_handler!(SetupCallback::A, App);
setup_handler!(SetupCallback::AM, App, AssetManager);
setup_handler!(SetupCallback::AP, App, Plugins);
setup_handler!(SetupCallback::AMP, App, AssetManager, Plugins);
setup_handler!(SetupCallback::M, AssetManager);
setup_handler!(SetupCallback::MP, AssetManager, Plugins);
setup_handler!(SetupCallback::P, Plugins);
setup_handler!(SetupCallback::G, Graphics);
setup_handler!(SetupCallback::AG, App, Graphics);
setup_handler!(SetupCallback::MG, AssetManager, Graphics);
setup_handler!(SetupCallback::AMG, App, AssetManager, Graphics);
setup_handler!(SetupCallback::GP, Graphics, Plugins);
setup_handler!(SetupCallback::MGP, AssetManager, Graphics, Plugins);
setup_handler!(SetupCallback::AMGP, App, AssetManager, Graphics, Plugins);

pub enum AppCallback<S> {
    Empty(Box<dyn Fn()>),

    A(Box<dyn Fn(&mut App)>),
    AS(Box<dyn Fn(&mut App, &mut S)>),
    AM(Box<dyn Fn(&mut App, &mut AssetManager)>),
    AP(Box<dyn Fn(&mut App, &mut Plugins)>),
    APS(Box<dyn Fn(&mut App, &mut Plugins, &mut S)>),
    AMS(Box<dyn Fn(&mut App, &mut AssetManager, &mut S)>),
    AMP(Box<dyn Fn(&mut App, &mut AssetManager, &mut Plugins)>),
    AMPS(Box<dyn Fn(&mut App, &mut AssetManager, &mut Plugins, &mut S)>),

    M(Box<dyn Fn(&mut AssetManager)>),
    MP(Box<dyn Fn(&mut AssetManager, &mut Plugins)>),
    MS(Box<dyn Fn(&mut AssetManager, &mut S)>),
    MPS(Box<dyn Fn(&mut AssetManager, &mut Plugins, &mut S)>),

    P(Box<dyn Fn(&mut Plugins)>),
    PS(Box<dyn Fn(&mut Plugins, &mut S)>),

    S(Box<dyn Fn(&mut S)>),
}

impl<State> AppCallback<State> {
    pub(crate) fn exec(
        &self,
        app: &mut App,
        manager: &mut AssetManager,
        plugins: &mut Plugins,
        state: &mut State,
    ) {
        use AppCallback::*;
        match self {
            Empty(cb) => cb(),
            A(cb) => cb(app),
            AS(cb) => cb(app, state),
            AM(cb) => cb(app, manager),
            AP(cb) => cb(app, plugins),
            APS(cb) => cb(app, plugins, state),
            AMS(cb) => cb(app, manager, state),
            AMP(cb) => cb(app, manager, plugins),
            AMPS(cb) => cb(app, manager, plugins, state),

            M(cb) => cb(manager),
            MP(cb) => cb(manager, plugins),
            MS(cb) => cb(manager, state),
            MPS(cb) => cb(manager, plugins, state),

            P(cb) => cb(plugins),
            PS(cb) => cb(plugins, state),

            S(cb) => cb(state),
        }
    }
}

pub trait AppHandler<S, Params> {
    fn callback(self) -> AppCallback<S>;
}

macro_rules! app_handler {
    ($variant:expr, $($param:ident),*) => {
        #[allow(unused_parens)]
        impl<F, S> AppHandler<S, ($(&mut $param),*)> for F
        where
            F: Fn($(&mut $param),*) + 'static,
            S: AppState
        {
            fn callback(self) -> AppCallback<S> {
                $variant(Box::new(self))
            }
        }
    }
}

app_handler!(AppCallback::Empty,);
app_handler!(AppCallback::A, App);
app_handler!(AppCallback::AS, App, S);
app_handler!(AppCallback::AM, App, AssetManager);
app_handler!(AppCallback::AP, App, Plugins);
app_handler!(AppCallback::APS, App, Plugins, S);
app_handler!(AppCallback::AMS, App, AssetManager, S);
app_handler!(AppCallback::AMP, App, AssetManager, Plugins);
app_handler!(AppCallback::AMPS, App, AssetManager, Plugins, S);
app_handler!(AppCallback::M, AssetManager);
app_handler!(AppCallback::MP, AssetManager, Plugins);
app_handler!(AppCallback::MS, AssetManager, S);
app_handler!(AppCallback::MPS, AssetManager, Plugins, S);
app_handler!(AppCallback::P, Plugins);
app_handler!(AppCallback::PS, Plugins, S);
app_handler!(AppCallback::S, S);

pub enum EventCallback<S> {
    E(Box<dyn Fn(Event)>),

    AE(Box<dyn Fn(&mut App, Event)>),
    AME(Box<dyn Fn(&mut App, &mut AssetManager, Event)>),
    ASE(Box<dyn Fn(&mut App, &mut S, Event)>),
    APE(Box<dyn Fn(&mut App, &mut Plugins, Event)>),
    AMPE(Box<dyn Fn(&mut App, &mut AssetManager, &mut Plugins, Event)>),
    APSE(Box<dyn Fn(&mut App, &mut Plugins, &mut S, Event)>),
    AMPSE(Box<dyn Fn(&mut App, &mut AssetManager, &mut Plugins, &mut S, Event)>),

    ME(Box<dyn Fn(&mut AssetManager, Event)>),
    MSE(Box<dyn Fn(&mut AssetManager, &mut S, Event)>),
    MPE(Box<dyn Fn(&mut AssetManager, &mut Plugins, Event)>),
    MPSE(Box<dyn Fn(&mut AssetManager, &mut Plugins, &mut S, Event)>),

    PE(Box<dyn Fn(&mut Plugins, Event)>),
    PSE(Box<dyn Fn(&mut Plugins, &mut S, Event)>),

    SE(Box<dyn Fn(&mut S, Event)>),
}

impl<State> EventCallback<State> {
    pub(crate) fn exec(
        &self,
        app: &mut App,
        manager: &mut AssetManager,
        plugins: &mut Plugins,
        state: &mut State,
        event: Event,
    ) {
        use EventCallback::*;
        match self {
            E(cb) => cb(event),

            AE(cb) => cb(app, event),
            AME(cb) => cb(app, manager, event),
            ASE(cb) => cb(app, state, event),
            APE(cb) => cb(app, plugins, event),
            AMPE(cb) => cb(app, manager, plugins, event),
            APSE(cb) => cb(app, plugins, state, event),
            AMPSE(cb) => cb(app, manager, plugins, state, event),

            ME(cb) => cb(manager, event),
            MSE(cb) => cb(manager, state, event),
            MPE(cb) => cb(manager, plugins, event),
            MPSE(cb) => cb(manager, plugins, state, event),

            PE(cb) => cb(plugins, event),
            PSE(cb) => cb(plugins, state, event),

            SE(cb) => cb(state, event),
        }
    }
}

pub trait EventHandler<S, Params> {
    fn callback(self) -> EventCallback<S>;
}

macro_rules! event_handler {
    ($variant:expr, $($param:ident),*) => {
        #[allow(unused_parens)]
        impl<F, S> EventHandler<S, ($(&mut $param,)* Event)> for F
        where
            F: Fn($(&mut $param,)* Event) + 'static,
            S: AppState
        {
            fn callback(self) -> EventCallback<S> {
                $variant(Box::new(self))
            }
        }
    }
}

event_handler!(EventCallback::E,);
event_handler!(EventCallback::AE, App);
event_handler!(EventCallback::AME, App, AssetManager);
event_handler!(EventCallback::ASE, App, S);
event_handler!(EventCallback::APE, App, Plugins);
event_handler!(EventCallback::AMPE, App, AssetManager, Plugins);
event_handler!(EventCallback::APSE, App, Plugins, S);
event_handler!(EventCallback::AMPSE, App, AssetManager, Plugins, S);
event_handler!(EventCallback::ME, AssetManager);
event_handler!(EventCallback::MSE, AssetManager, S);
event_handler!(EventCallback::MPE, AssetManager, Plugins);
event_handler!(EventCallback::MPSE, AssetManager, Plugins, S);
event_handler!(EventCallback::PE, Plugins);
event_handler!(EventCallback::PSE, Plugins, S);
event_handler!(EventCallback::SE, S);

pub enum DrawCallback<S> {
    G(Box<dyn Fn(&mut Graphics)>),
    GS(Box<dyn Fn(&mut Graphics, &mut S)>),
    // AE(Box<dyn Fn(&mut App, Event)>),
    // AME(Box<dyn Fn(&mut App, &mut AssetManager, Event)>),
    // ASE(Box<dyn Fn(&mut App, &mut S, Event)>),
    // APE(Box<dyn Fn(&mut App, &mut Plugins, Event)>),
    // AMPE(Box<dyn Fn(&mut App, &mut AssetManager, &mut Plugins, Event)>),
    // APSE(Box<dyn Fn(&mut App, &mut Plugins, &mut S, Event)>),
    // AMPSE(Box<dyn Fn(&mut App, &mut AssetManager, &mut Plugins, &mut S, Event)>),
    //
    // ME(Box<dyn Fn(&mut AssetManager, Event)>),
    // MSE(Box<dyn Fn(&mut AssetManager, &mut S, Event)>),
    // MPE(Box<dyn Fn(&mut AssetManager, &mut Plugins, Event)>),
    // MPSE(Box<dyn Fn(&mut AssetManager, &mut Plugins, &mut S, Event)>),
    //
    // PE(Box<dyn Fn(&mut Plugins, Event)>),
    // PSE(Box<dyn Fn(&mut Plugins, &mut S, Event)>),
    //
    // SE(Box<dyn Fn(&mut S, Event)>),
}

impl<State> DrawCallback<State> {
    pub(crate) fn exec(
        &self,
        app: &mut App,
        manager: &mut AssetManager,
        graphics: &mut Graphics,
        plugins: &mut Plugins,
        state: &mut State,
    ) {
        use DrawCallback::*;
        match self {
            G(cb) => cb(graphics),
            GS(cb) => cb(graphics, state),
            // AE(cb) => cb(app, event),
            // AME(cb) => cb(app, manager, event),
            // ASE(cb) => cb(app, state, event),
            // APE(cb) => cb(app, plugins, event),
            // AMPE(cb) => cb(app, manager, plugins, event),
            // APSE(cb) => cb(app, plugins, state, event),
            // AMPSE(cb) => cb(app, manager, plugins, state, event),
            //
            // ME(cb) => cb(manager, event),
            // MSE(cb) => cb(manager, state, event),
            // MPE(cb) => cb(manager, plugins, event),
            // MPSE(cb) => cb(manager, plugins, state, event),
            //
            // PE(cb) => cb(plugins, event),
            // PSE(cb) => cb(plugins, state, event),
            //
            // SE(cb) => cb(state, event),
        }
    }
}

pub trait DrawHandler<S, Params> {
    fn callback(self) -> DrawCallback<S>;
}

macro_rules! draw_handler {
    ($variant:expr, $($param:ident),*) => {
        #[allow(unused_parens)]
        impl<F, S> DrawHandler<S, ($(&mut $param),*)> for F
        where
            F: Fn($(&mut $param),*) + 'static,
            S: AppState
        {
            fn callback(self) -> DrawCallback<S> {
                $variant(Box::new(self))
            }
        }
    }
}

draw_handler!(DrawCallback::G, Graphics);
draw_handler!(DrawCallback::GS, Graphics, S);
// event_handler!(EventCallback::AE, App);
// event_handler!(EventCallback::AME, App, AssetManager);
// event_handler!(EventCallback::ASE, App, S);
// event_handler!(EventCallback::APE, App, Plugins);
// event_handler!(EventCallback::AMPE, App, AssetManager, Plugins);
// event_handler!(EventCallback::APSE, App, Plugins, S);
// event_handler!(EventCallback::AMPSE, App, AssetManager, Plugins, S);
// event_handler!(EventCallback::ME, AssetManager);
// event_handler!(EventCallback::MSE, AssetManager, S);
// event_handler!(EventCallback::MPE, AssetManager, Plugins);
// event_handler!(EventCallback::MPSE, AssetManager, Plugins, S);
// event_handler!(EventCallback::PE, Plugins);
// event_handler!(EventCallback::PSE, Plugins, S);
// event_handler!(EventCallback::SE, S);
