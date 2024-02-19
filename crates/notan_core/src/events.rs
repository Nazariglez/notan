use crate::option_usize_env;
use crate::state::AppState;
use crate::sys::System;
use crate::window::WindowId;

use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};

#[cfg(feature = "limited_events")]
const MAX_EVENT_LISTENERS: usize = option_usize_env!("NOTAN_LIMIT_EVENTS_TO", 32);

#[cfg(feature = "limited_events")]
pub(crate) type EventMap = HashMap<TypeId, arrayvec::ArrayVec<EventListener, MAX_EVENT_LISTENERS>>;

#[cfg(not(feature = "limited_events"))]
pub(crate) type EventMap = HashMap<TypeId, Vec<EventListener>>;

pub(crate) enum EventListener {
    Once(u64, Option<Box<dyn Any>>),
    Mut(u64, Box<dyn Any>),
}

impl EventListener {
    pub(crate) fn is_once(&self) -> bool {
        if let Self::Once(_, _) = self {
            return true;
        }

        false
    }
}

/// A list of events pushed by plugins to be processed
#[derive(Default)]
pub struct EventQueue<S: AppState + 'static> {
    pub(crate) events: VecDeque<Box<dyn FnOnce(&mut System<S>)>>,
}

impl<S: AppState + 'static> EventQueue<S> {
    pub(crate) fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    /// Add a new event to the queue
    pub fn queue<E: Send + Sync + std::fmt::Debug + 'static>(&mut self, event: E) {
        self.events.push_back(Box::new(move |app| app.event(event)));
    }

    /// Take the first event of the queue
    pub(crate) fn take_event(&mut self) -> Option<Box<dyn FnOnce(&mut System<S>)>> {
        self.events.pop_front()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct InitEvent;

#[derive(Debug, Copy, Clone)]
pub struct FrameStartEvent;

#[derive(Debug, Copy, Clone)]
pub struct UpdateEvent;

#[derive(Debug, Copy, Clone)]
pub struct DrawEvent {
    pub window_id: WindowId,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct FrameEndEvent;

#[derive(Debug, Copy, Clone)]
pub struct RequestCloseEvent;

#[derive(Debug, Copy, Clone)]
pub struct CloseEvent;
