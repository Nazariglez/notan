use crate::app::App;
use crate::backend::*;
use crate::builder::AppBuilder;
use crate::events::Event;
use indexmap::IndexMap;
use std::any::{Any, TypeId};

/// Control the flow of the application
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum AppFlow {
    /// Keeps running as usual, calling the next callback
    /// This is the option by default
    Next = 0,

    /// Cancels the execution of the next callback
    Skip = 1,

    /// Cancels the whole frame execution
    SkipFrame = 2,
}

impl Default for AppFlow {
    fn default() -> AppFlow {
        AppFlow::Next
    }
}

/// A container of plugins that allow get them to use it
pub struct Plugins {
    pub(crate) map: IndexMap<TypeId, Box<Plugin>>,
}

impl Plugins {
    pub(crate) fn new() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }

    pub(crate) fn set<T: Plugin + 'static>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }

    /// Returns the plugin of the type passed
    pub fn get<T: Plugin + 'static + MyAny>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .map(|value| value.as_any().downcast_ref().unwrap())
    }

    /// Returns the plugin of the type passed as mutable reference
    pub fn get_mut<T: Plugin + 'static>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .map(|value| value.as_any_mut().downcast_mut().unwrap())
    }

    pub(crate) fn init(&mut self, app: &mut App) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.init(app))
            .max()
            .unwrap_or(Ok(Default::default()))
    }

    pub(crate) fn pre_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.pre_frame(app))
            .max()
            .unwrap_or(Ok(Default::default()))
    }

    pub(crate) fn event(&mut self, app: &mut App, event: &Event) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.event(app, event))
            .max()
            .unwrap_or(Ok(Default::default()))
    }

    pub(crate) fn update(&mut self, app: &mut App) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.update(app))
            .max()
            .unwrap_or(Ok(Default::default()))
    }

    pub(crate) fn draw(&mut self, app: &mut App) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.draw(app))
            .max()
            .unwrap_or(Ok(Default::default()))
    }

    pub(crate) fn post_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.post_frame(app))
            .max()
            .unwrap_or(Ok(Default::default()))
    }
}

/// A plugin allow the user to extend or alter the application
pub trait Plugin
where
    Self: Any + Send + Sync,
{
    /// Executed before the application loop
    fn init(&mut self, app: &mut App) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed a the beginning of each iteration
    fn pre_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed for each event received
    fn event(&mut self, app: &mut App, event: &Event) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed for each frame before the update method
    fn update(&mut self, app: &mut App) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed for each frame before the draw method
    fn draw(&mut self, app: &mut App) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed at the end of the frame
    fn post_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed when it's added to the builder
    fn build<S, B>(&mut self, builder: &mut AppBuilder<S, B>) where Self: Sized {}
}

pub trait MyAny: Any {
    fn as_any(&self) -> &(Any + '_);
    fn as_any_mut(&mut self) -> &mut (Any + '_);
}

impl<T: Any> MyAny for T {
    fn as_any(&self) -> &(Any + '_) {
        self
    }

    fn as_any_mut(&mut self) -> &mut (Any + '_) {
        self
    }
}
