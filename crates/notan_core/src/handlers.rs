#![allow(unused)]
use crate::plugin::Plugin;
use crate::state::AppState;
use crate::storage::{FromPlugins, FromStorage, Plugins, Storage};
use crate::sys::System;

pub(crate) type RunnerHandlerFn<S> = dyn FnMut(System<S>) -> Result<(), String>;
pub(crate) type SetupHandlerFn<S> = dyn FnOnce(&mut Plugins) -> Result<S, String>;
pub(crate) type PluginHandlerFn<P> = dyn FnOnce(&mut Plugins) -> Result<P, String>;
pub(crate) type UpdateHandlerFn<S> = dyn FnMut(&mut Storage<S>);
pub(crate) type EventHandlerFn<E, S> = dyn FnMut(&mut Storage<S>, &E);
pub(crate) type EventHandlerFnOnce<E, S> = dyn FnOnce(&mut Storage<S>, &E);

/// Represent an update's handler
/// It allow to use as parameter the App's State
/// or any App's plugin
pub trait Handler<S: AppState, T> {
    fn call(&mut self, app: &mut Storage<S>);
}

// Safe for notan because the map will never change
// once it's created it will not have new register or removed ones
// Doing this we got interior mutability for the components but not the map
// because is never exposes
macro_rules! fn_handler ({ $($param:ident)* } => {
    impl<S, Fun, $($param,)*> Handler<S, ($($param,)*)> for Fun
    where
        S: AppState + 'static,
        Fun: FnMut($(&mut $param),*),
        $($param:FromStorage<S> + 'static),*
    {
        fn call(&mut self, storage: &mut Storage<S>) {
            // Look for duplicated parameters and panic
            #[cfg(debug_assertions)]
            {
                use std::collections::HashSet;
                use std::any::TypeId;
                let mut h_set:HashSet<TypeId> = Default::default();

                $(
                    if !h_set.insert(TypeId::of::<$param>()) {
                        panic!("Application handlers cannot contains duplicated parameters.");
                    }
                )*
            }


            // Safety. //TODO
            paste::paste! {
                let ($([<$param:lower _v>],)*) = unsafe {
                    $(let [<$param:lower _v>] = $param::from_storage(storage) as *mut _;)*
                    ($(&mut *[<$param:lower _v>],)*)
                };
                (self)($([<$param:lower _v>],)*);
            }
        }
    }
});

fn_handler! {}
fn_handler! { A }
fn_handler! { A B }
fn_handler! { A B C }
fn_handler! { A B C D }
fn_handler! { A B C D E }
fn_handler! { A B C D E F }
fn_handler! { A B C D E F G }
fn_handler! { A B C D E F G H }
fn_handler! { A B C D E F G H I }
fn_handler! { A B C D E F G H I J }

/// Represent a setuos's handler
/// It allow to use as parameter any app's plugin
pub trait SetupHandler<S: AppState, T> {
    fn call(self, storage: &mut Plugins) -> Result<S, String>;
}

// Safe for notan because the map will never change
// once it's created it will not have new register or removed ones
// Doing this we got interior mutability for the components but not the map
// because is never exposes
macro_rules! fn_setup_handler ({ $($param:ident)* } => {
    impl<S, Fun, $($param,)*> SetupHandler<S, ($($param,)*)> for Fun
    where
        S: AppState + 'static,
        Fun: FnOnce($(&mut $param),*) -> Result<S, String>,
        $($param:FromPlugins + 'static),*
    {
        fn call(mut self, plugins: &mut Plugins) -> Result<S, String> {
            // Look for duplicated parameters and panic
            #[cfg(debug_assertions)]
            {
                use std::collections::HashSet;
                use std::any::TypeId;
                let mut h_set:HashSet<TypeId> = Default::default();

                $(
                    if !h_set.insert(TypeId::of::<$param>()) {
                        panic!("Application handlers cannot contains duplicated parameters.");
                    }
                )*
            }


            // Safety. //TODO
            paste::paste! {
                let ($([<$param:lower _v>],)*) = unsafe {
                    $(let [<$param:lower _v>] = $param::from_plugins(plugins) as *mut _;)*
                    ($(&mut *[<$param:lower _v>],)*)
                };
                return (self)($([<$param:lower _v>],)*);
            }
        }
    }
});

fn_setup_handler! {}
fn_setup_handler! { A }
fn_setup_handler! { A B }
fn_setup_handler! { A B C }
fn_setup_handler! { A B C D }
fn_setup_handler! { A B C D E }
fn_setup_handler! { A B C D E F }
fn_setup_handler! { A B C D E F G }
fn_setup_handler! { A B C D E F G H }
fn_setup_handler! { A B C D E F G H I }
fn_setup_handler! { A B C D E F G H I J }

/// Represent a plugin's handler
/// It allow to use as parameter any app's plugin
pub trait PluginHandler<P: Plugin, T> {
    fn call(self, storage: &mut Plugins) -> Result<P, String>;
}

// Safe for notan because the map will never change
// once it's created it will not have new register or removed ones
// Doing this we got interior mutability for the components but not the map
// because is never exposes
macro_rules! fn_plugin_handler ({ $($param:ident)* } => {
    impl<P, Fun, $($param,)*> PluginHandler<P, ($($param,)*)> for Fun
    where
        P: Plugin + 'static,
        Fun: FnOnce($(&mut $param),*) -> Result<P, String>,
        $($param:FromPlugins + 'static),*
    {
        fn call(mut self, plugins: &mut Plugins) -> Result<P, String> {
            // Look for duplicated parameters and panic
            #[cfg(debug_assertions)]
            {
                use std::collections::HashSet;
                use std::any::TypeId;
                let mut h_set:HashSet<TypeId> = Default::default();

                $(
                    if !h_set.insert(TypeId::of::<$param>()) {
                        panic!("Application handlers cannot contains duplicated parameters.");
                    }
                )*
            }


            // Safety. //TODO
            paste::paste! {
                let ($([<$param:lower _v>],)*) = unsafe {
                    $(let [<$param:lower _v>] = $param::from_plugins(plugins) as *mut _;)*
                    ($(&mut *[<$param:lower _v>],)*)
                };
                return (self)($([<$param:lower _v>],)*);
            }
        }
    }
});

fn_plugin_handler! {}
fn_plugin_handler! { A }
fn_plugin_handler! { A B }
fn_plugin_handler! { A B C }
fn_plugin_handler! { A B C D }
fn_plugin_handler! { A B C D E }
fn_plugin_handler! { A B C D E F }
fn_plugin_handler! { A B C D E F G }
fn_plugin_handler! { A B C D E F G H }
fn_plugin_handler! { A B C D E F G H I }
fn_plugin_handler! { A B C D E F G H I J }

/// Represent a event's handler
/// It allow to use as parameter the App's State
/// or any App's plugin
pub trait EventHandler<Evt, S: AppState, T> {
    fn call(&mut self, app: &mut Storage<S>, evt: &Evt);
}

// Safe for notan because the map will never change
// once it's created it will not have new register or removed ones
// Doing this we got interior mutability for the components but not the map
// because is never exposes
macro_rules! fn_event_handler ({ $($param:ident)* } => {
    impl<Evt, S, Fun, $($param,)*> EventHandler<Evt, S, ($($param,)*)> for Fun
    where
        S: AppState + 'static,
        Fun: FnMut(&Evt, $(&mut $param),*),
        $($param:FromStorage<S> + 'static),*
    {
        fn call(&mut self, storage: &mut Storage<S>, evt: &Evt) {
            // Look for duplicated parameters and panic
            #[cfg(debug_assertions)]
            {
                use std::collections::HashSet;
                use std::any::TypeId;
                let mut h_set:HashSet<TypeId> = Default::default();

                $(
                    if !h_set.insert(TypeId::of::<$param>()) {
                        panic!("Application handlers cannot contains duplicated parameters.");
                    }
                )*
            }


            // Safety. //TODO
            paste::paste! {
                let ($([<$param:lower _v>],)*) = unsafe {
                    $(let [<$param:lower _v>] = $param::from_storage(storage) as *mut _;)*
                    ($(&mut *[<$param:lower _v>],)*)
                };
                (self)(evt, $([<$param:lower _v>],)*);
            }
        }
    }
});

fn_event_handler! {}
fn_event_handler! { A }
fn_event_handler! { A B }
fn_event_handler! { A B C }
fn_event_handler! { A B C D }
fn_event_handler! { A B C D E }
fn_event_handler! { A B C D E F }
fn_event_handler! { A B C D E F G }
fn_event_handler! { A B C D E F G H }
fn_event_handler! { A B C D E F G H I }
fn_event_handler! { A B C D E F G H I J }

/// Represent a event's handler
/// It allow to use as parameter the App's State
/// or any App's plugin
pub trait EventHandlerOnce<Evt, S: AppState, T> {
    fn call(self, app: &mut Storage<S>, evt: &Evt);
}

// Safe for notan because the map will never change
// once it's created it will not have new register or removed ones
// Doing this we got interior mutability for the components but not the map
// because is never exposes
macro_rules! fn_event_once_handler ({ $($param:ident)* } => {
    impl<Evt, S, Fun, $($param,)*> EventHandlerOnce<Evt, S, ($($param,)*)> for Fun
    where
        S: AppState + 'static,
        Fun: FnOnce(&Evt, $(&mut $param),*),
        $($param:FromStorage<S> + 'static),*
    {
        fn call(mut self, storage: &mut Storage<S>, evt: &Evt) {
            // Look for duplicated parameters and panic
            #[cfg(debug_assertions)]
            {
                use std::collections::HashSet;
                use std::any::TypeId;
                let mut h_set:HashSet<TypeId> = Default::default();

                $(
                    if !h_set.insert(TypeId::of::<$param>()) {
                        panic!("Application handlers cannot contains duplicated parameters.");
                    }
                )*
            }


            // Safety. //TODO
            paste::paste! {
                let ($([<$param:lower _v>],)*) = unsafe {
                    $(let [<$param:lower _v>] = $param::from_storage(storage) as *mut _;)*
                    ($(&mut *[<$param:lower _v>],)*)
                };
                (self)(evt, $([<$param:lower _v>],)*);
            }
        }
    }
});

fn_event_once_handler! {}
fn_event_once_handler! { A }
fn_event_once_handler! { A B }
fn_event_once_handler! { A B C }
fn_event_once_handler! { A B C D }
fn_event_once_handler! { A B C D E }
fn_event_once_handler! { A B C D E F }
fn_event_once_handler! { A B C D E F G }
fn_event_once_handler! { A B C D E F G H }
fn_event_once_handler! { A B C D E F G H I }
fn_event_once_handler! { A B C D E F G H I J }
