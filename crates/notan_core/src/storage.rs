use crate::events::EventQueue;
use crate::plugin::Plugin;
use crate::state::AppState;
use crate::sys::System;
use anymap::AnyMap;

pub struct Storage<S: AppState + 'static> {
    pub state: S,
    pub plugins: Plugins,
    pub events: EventQueue<S>,
}

impl<S: AppState + 'static> Storage<S> {
    pub fn take_event(&mut self) -> Option<Box<dyn FnOnce(&mut System<S>)>> {
        self.events.take_event()
    }
}

pub(crate) struct Plugins {
    map: AnyMap,
}

impl Plugins {
    pub(crate) fn new() -> Self {
        Self { map: AnyMap::new() }
    }

    pub(crate) fn add<T: 'static>(&mut self, plugin: T) {
        self.map.insert(plugin);
    }

    pub(crate) fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map.get_mut()
    }
}

pub trait FromPlugins {
    fn from_plugins(storage: &mut Plugins) -> &mut Self;
}

impl<T: 'static> FromPlugins for T {
    fn from_plugins(storage: &mut Plugins) -> &mut Self {
        storage.map.get_mut::<Self>().unwrap()
    }
}

pub trait FromStorage<S: AppState> {
    fn from_storage<'state>(app: &'state mut Storage<S>) -> &'state mut Self;
}

impl<S: AppState, T: Plugin + 'static> FromStorage<S> for T {
    fn from_storage(storage: &mut Storage<S>) -> &mut Self {
        storage.plugins.map.get_mut::<Self>().unwrap()
    }
}

impl<S: AppState + 'static> FromStorage<S> for EventQueue<S> {
    fn from_storage(storage: &mut Storage<S>) -> &mut Self {
        &mut storage.events
    }
}
