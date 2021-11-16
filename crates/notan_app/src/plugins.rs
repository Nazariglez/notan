use crate::app::App;
use crate::assets::Assets;
use crate::builder::AppBuilder;
use crate::events::Event;
use crate::Graphics;
use downcast_rs::{impl_downcast, Downcast};
use indexmap::IndexMap;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};

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

// helper trait to be able to downcast from Any to RefCell<T: Plugin> (traits doesn't have size, so cannot be downcasted to)
trait PluginCell
where
    Self: Any + Downcast,
{
    fn run_init(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String>;
    fn run_pre_frame(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String>;
    fn run_event(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        event: &Event,
    ) -> Result<AppFlow, String>;
    fn run_update(&mut self, app: &mut App, assets: &mut Assets) -> Result<AppFlow, String>;
    fn run_draw(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String>;
    fn run_post_frame(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String>;
}

impl<T: Plugin + 'static> PluginCell for RefCell<T> {
    #[inline(always)]
    fn run_init(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        self.borrow_mut().init(app, assets, gfx)
    }

    #[inline(always)]
    fn run_pre_frame(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        self.borrow_mut().pre_frame(app, assets, gfx)
    }

    #[inline(always)]
    fn run_event(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        event: &Event,
    ) -> Result<AppFlow, String> {
        self.borrow_mut().event(app, assets, event)
    }

    #[inline(always)]
    fn run_update(&mut self, app: &mut App, assets: &mut Assets) -> Result<AppFlow, String> {
        self.borrow_mut().update(app, assets)
    }

    #[inline(always)]
    fn run_draw(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        self.borrow_mut().draw(app, assets, gfx)
    }

    #[inline(always)]
    fn run_post_frame(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        self.borrow_mut().post_frame(app, assets, gfx)
    }
}

impl_downcast!(PluginCell);

/// A container of plugins that allow get them to use it
#[derive(Default)]
pub struct Plugins {
    map: IndexMap<TypeId, Box<dyn PluginCell>>,
}

impl Plugins {
    /// Adds a new plugin
    pub fn add<T: Plugin + 'static>(&mut self, value: T) {
        self.map
            .insert(TypeId::of::<T>(), Box::new(RefCell::new(value)));
    }

    /// Remove the plugin of the type passed
    pub fn remove<T: Plugin + 'static>(&mut self) {
        self.map.remove(&TypeId::of::<T>());
    }

    /// Returns the plugin of the type passed
    pub fn get<T: Plugin + 'static>(&self) -> Option<Ref<T>> {
        self.map
            .get(&TypeId::of::<T>())?
            .downcast_ref::<RefCell<T>>()
            .map(|value| value.borrow())
    }

    /// Returns the plugin of the type passed as mutable reference
    pub fn get_mut<T: Plugin + 'static>(&self) -> Option<RefMut<T>> {
        self.map
            .get(&TypeId::of::<T>())?
            .downcast_ref::<RefCell<T>>()
            .map(|value| value.borrow_mut())
    }

    #[inline]
    pub(crate) fn init(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.run_init(app, assets, gfx))
            .max()
            .unwrap_or_else(|| Ok(Default::default()))
    }

    #[inline]
    pub(crate) fn pre_frame(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.run_pre_frame(app, assets, gfx))
            .max()
            .unwrap_or_else(|| Ok(Default::default()))
    }

    #[inline]
    pub(crate) fn event(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        event: &Event,
    ) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.run_event(app, assets, event))
            .max()
            .unwrap_or_else(|| Ok(Default::default()))
    }

    #[inline]
    pub(crate) fn update(&mut self, app: &mut App, assets: &mut Assets) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.run_update(app, assets))
            .max()
            .unwrap_or_else(|| Ok(Default::default()))
    }

    #[inline]
    pub(crate) fn draw(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.run_draw(app, assets, gfx))
            .max()
            .unwrap_or_else(|| Ok(Default::default()))
    }

    #[inline]
    pub(crate) fn post_frame(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        self.map
            .iter_mut()
            .map(|(_, p)| p.run_post_frame(app, assets, gfx))
            .max()
            .unwrap_or_else(|| Ok(Default::default()))
    }
}

#[allow(unused_variables)]
/// A plugin allow the user to extend or alter the application
pub trait Plugin
where
    Self: Send + Sync,
{
    /// Executed before the application loop
    fn init(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed a the beginning of each iteration
    fn pre_frame(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed for each event received
    fn event(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        event: &Event,
    ) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed for each frame before the update method
    fn update(&mut self, app: &mut App, assets: &mut Assets) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed for each frame before the draw method
    fn draw(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed at the end of the frame
    fn post_frame(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        Ok(Default::default())
    }

    /// Executed when it's added to the builder
    fn build<S, B>(&mut self, builder: &mut AppBuilder<S, B>)
    where
        Self: Sized,
    {
    }
}
