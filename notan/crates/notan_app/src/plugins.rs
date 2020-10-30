use crate::app::App;
use crate::backend::*;
use crate::builder::AppBuilder;
use indexmap::IndexMap;
use std::any::{Any, TypeId};

pub struct Plugins {
    pub(crate) map: IndexMap<TypeId, Box<Plugin>>,
}

impl Plugins {
    pub fn new() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }

    pub fn set<T: Plugin + 'static>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: Plugin + 'static + MyAny>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .map(|value| value.as_any().downcast_ref().unwrap())
    }

    // pub fn get_mut<T: Plugin + 'static>(&mut self) -> Option<&mut T> {
    //     self.map.get_mut(&TypeId::of::<T>())
    //         .map(|value| value.as_any().downcast_mut().unwrap())
    // }
}

pub trait Plugin
where
    Self: Any + Send + Sync,
{
    fn init(&mut self, app: &mut App) -> Result<(), String> {
        Ok(())
    }
    fn pre_frame(&mut self, app: &mut App) -> Result<(), String> {
        Ok(())
    }
    fn event(&mut self, app: &mut App) -> Result<(), String> {
        Ok(())
    }
    fn update(&mut self, app: &mut App) -> Result<(), String> {
        Ok(())
    }
    fn draw(&mut self, app: &mut App) -> Result<(), String> {
        Ok(())
    }
    fn post_frame(&mut self, app: &mut App) -> Result<(), String> {
        Ok(())
    }
}

pub trait MyAny: Any {
    fn as_any(&self) -> &(Any + '_);
}

impl<T: Any> MyAny for T {
    fn as_any(&self) -> &(Any + '_) {
        self
    }
}
