use crate::config::BuildConfig;
use crate::events::{EventListener, EventMap, EventQueue};
use crate::handlers::{
    EventHandler, EventHandlerFn, EventHandlerFnOnce, EventHandlerOnce, PluginHandler,
    RunnerHandlerFn, SetupHandler, SetupHandlerFn,
};
use crate::plugin::Plugin;
use crate::runner::default_runner;
use crate::state::AppState;
use crate::storage::{Plugins, Storage};
use crate::sys::System;
use indexmap::IndexMap;
use std::any::TypeId;
use std::collections::HashMap;

pub struct AppBuilder<S: AppState + 'static> {
    plugins: Plugins,
    runner: Box<RunnerHandlerFn<S>>,
    setup_handler: Box<SetupHandlerFn<S>>,
    event_handler: EventMap,
    late_configs: Option<IndexMap<TypeId, Box<dyn BuildConfig<S>>>>,
    event_ids: u64,
}

impl AppState for () {}

impl AppBuilder<()> {
    pub fn init() -> Self {
        Self::init_with(|| Ok(()))
    }
}

impl<S: AppState> AppBuilder<S> {
    pub fn init_with<T, H>(handler: H) -> Self
    where
        H: SetupHandler<S, T> + 'static,
    {
        let plugins = Plugins::new();
        let runner = Box::new(default_runner);
        let setup_handler: Box<SetupHandlerFn<S>> = Box::new(|plugins| handler.call(plugins));
        let event_handler = HashMap::default();
        let late_configs = Some(Default::default());

        Self {
            plugins,
            runner,
            setup_handler,
            event_handler,
            late_configs,
            event_ids: 0,
        }
    }

    pub fn add_config<C>(mut self, mut config: C) -> Result<Self, String>
    where
        C: BuildConfig<S> + 'static,
    {
        if config.late_evaluation() {
            if let Some(late_configs) = &mut self.late_configs {
                let typ = TypeId::of::<C>();
                late_configs.insert(typ, Box::new(config));
            }

            return Ok(self);
        }

        config.apply(self)
    }

    pub fn on<E, T, H>(mut self, mut handler: H) -> Self
    where
        E: 'static,
        H: EventHandler<E, S, T> + 'static,
    {
        let k = TypeId::of::<E>();
        let cb: Box<EventHandlerFn<E, S>> =
            Box::new(move |s: &mut Storage<S>, e: &E| handler.call(s, e));
        self.event_handler
            .entry(k)
            .or_default()
            .push(EventListener::Mut(self.event_ids, Box::new(cb)));
        self.event_ids += 1;
        self
    }

    pub fn once<E, T, H>(mut self, handler: H) -> Self
    where
        E: 'static,
        H: EventHandlerOnce<E, S, T> + 'static,
    {
        let k = TypeId::of::<E>();
        let cb: Box<EventHandlerFnOnce<E, S>> =
            Box::new(move |s: &mut Storage<S>, e: &E| handler.call(s, e));
        self.event_handler
            .entry(k)
            .or_default()
            .push(EventListener::Once(self.event_ids, Some(Box::new(cb))));
        self.event_ids += 1;
        self
    }

    pub fn with_runner<F: FnMut(System<S>) -> Result<(), String> + 'static>(
        mut self,
        runner: F,
    ) -> Self {
        self.runner = Box::new(runner);
        self
    }

    pub fn add_plugin<T: Plugin + 'static>(mut self, plugin: T) -> Self {
        self.plugins.add(plugin);
        self
    }

    pub fn add_plugin_with<T, P, H>(mut self, handler: H) -> Result<Self, String>
    where
        T: 'static,
        P: Plugin + 'static,
        H: PluginHandler<P, T> + 'static,
    {
        let plugin = handler.call(&mut self.plugins)?;
        Ok(self.add_plugin(plugin))
    }

    pub fn build(mut self) -> Result<(), String> {
        if let Some(late_configs) = self.late_configs.take() {
            for (_, mut config) in late_configs {
                self = config.apply(self)?;
            }
        }

        let Self {
            mut plugins,
            mut runner,
            setup_handler,
            event_handler,
            ..
        } = self;

        let state = (setup_handler)(&mut plugins)?;
        let storage = Storage {
            plugins,
            state,
            events: EventQueue::new(),
        };

        let app = System {
            storage,
            event_handler,
            initialized: false,
            in_frame: false,
            closed: false,
        };

        (runner)(app)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::state::AppState;
    use crate::storage::FromStorage;

    // helper to set states
    macro_rules! app_state {
        ($struct_name:ident) => {
            impl AppState for $struct_name {}
            impl FromStorage<$struct_name> for $struct_name {
                fn from_storage<'state>(
                    storage: &'state mut Storage<$struct_name>,
                ) -> &'state mut Self {
                    &mut storage.state
                }
            }
        };
    }

    #[test]
    fn empty_state_default_to_void() {
        let res = AppBuilder::init()
            .with_runner(|app| {
                assert_eq!(app.storage.state, ());
                Ok(())
            })
            .build();

        assert!(res.is_ok());
    }

    #[test]
    fn custom_state() {
        #[derive(Debug, Eq, PartialEq)]
        struct MyState(u64);
        app_state!(MyState);

        let res = AppBuilder::init_with(|| Ok(MyState(9999)))
            .with_runner(|app| {
                assert_eq!(app.storage.state, MyState(9999));
                Ok(())
            })
            .build();

        assert!(res.is_ok());
    }

    #[test]
    fn once_event_executed_once() {
        #[derive(Debug)]
        struct MyEvent;

        #[derive(Debug)]
        struct MyCount(usize);
        app_state!(MyCount);

        let res = AppBuilder::init_with(|| Ok(MyCount(0)))
            .once(|_: &MyEvent, count: &mut MyCount| count.0 += 1)
            .with_runner(|mut app| {
                app.init();

                for _ in 0..10 {
                    app.event(MyEvent);
                }

                assert_eq!(app.storage.state.0, 1);
                Ok(())
            })
            .build();

        assert!(res.is_ok());
    }

    #[test]
    fn on_event_executed_repeat() {
        #[derive(Debug)]
        struct MyEvent;

        #[derive(Debug)]
        struct MyCount(usize);
        app_state!(MyCount);

        const COUNT: usize = 10;

        let res = AppBuilder::init_with(|| Ok(MyCount(0)))
            .on(|_: &MyEvent, count: &mut MyCount| count.0 += 1)
            .with_runner(|mut app| {
                app.init();

                for _ in 0..COUNT {
                    app.event(MyEvent);
                }

                assert_eq!(app.storage.state.0, COUNT);
                Ok(())
            })
            .build();

        assert!(res.is_ok());
    }

    // TODO config and plugin
}
