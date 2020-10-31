use crate::app::App;
use crate::backend::*;
use crate::builder::AppBuilder;
use crate::events::Event;
use indexmap::IndexMap;
use std::any::{Any, TypeId};

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

    pub(crate) fn init(&mut self, app: &mut App) -> Result<(), String> {
        for (_, p) in &mut self.map {
            p.init(app)?;
        }

        Ok(())
    }

    pub(crate) fn pre_frame(&mut self, app: &mut App) -> Result<(), String> {
        for (_, p) in &mut self.map {
            p.pre_frame(app)?;
        }

        Ok(())
    }

    pub(crate) fn event(&mut self, app: &mut App, event: Event) -> Result<(), String> {
        for (_, p) in &mut self.map {
            p.event(app, event)?;
        }

        Ok(())
    }

    pub(crate) fn update(&mut self, app: &mut App) -> Result<(), String> {
        for (_, p) in &mut self.map {
            p.update(app)?;
        }

        Ok(())
    }

    pub(crate) fn draw(&mut self, app: &mut App) -> Result<(), String> {
        for (_, p) in &mut self.map {
            p.draw(app)?;
        }

        Ok(())
    }

    pub(crate) fn post_frame(&mut self, app: &mut App) -> Result<(), String> {
        for (_, p) in &mut self.map {
            p.post_frame(app)?;
        }

        Ok(())
    }
}

/// A plugin allow the user to extend the application
pub trait Plugin
where
    Self: Any + Send + Sync,
{
    /// Executed before the application loop
    fn init(&mut self, app: &mut App) -> Result<(), String> {
        Ok(())
    }

    /// Executed a the beginning of each iteration
    fn pre_frame(&mut self, app: &mut App) -> Result<(), String> {
        Ok(())
    }

    /// Executed for each event received
    fn event(&mut self, app: &mut App, event: Event) -> Result<(), String> {
        Ok(())
    }

    /// Executed for each frame before the update method
    fn update(&mut self, app: &mut App) -> Result<(), String> {
        Ok(())
    }

    /// Executed for each frame before the draw method
    fn draw(&mut self, app: &mut App) -> Result<(), String> {
        Ok(())
    }

    /// Executed at the end of the frame
    fn post_frame(&mut self, app: &mut App) -> Result<(), String> {
        Ok(())
    }
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
