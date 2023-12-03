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

// use std::collections::VecDeque;
//
// #[cfg(feature = "drop_files")]
// use std::path::PathBuf;
//
// use crate::keyboard::KeyCode;
// use crate::mouse::MouseButton;
//
// /// Application events usually received from the user
// #[derive(Debug, PartialEq, Clone)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub enum Event {
//     /// When the app is about to close
//     Exit,
//
//     /// Represents the current window's position after it was moved
//     WindowMove { x: i32, y: i32 },
//
//     /// Represents the current window's size after it was resized
//     WindowResize { width: u32, height: u32 },
//
//     /// Represents a change on the screen aspect ration
//     ScreenAspectChange { ratio: f64 },
//
//     /// Represents the current's mouse position after it was moved
//     MouseMove { x: i32, y: i32 },
//
//     /// A mouse button is down on this position
//     MouseDown { button: MouseButton, x: i32, y: i32 },
//
//     /// A mouse button was released on this position
//     MouseUp { button: MouseButton, x: i32, y: i32 },
//
//     /// The mouse wheel was moved with this delta
//     MouseWheel { delta_x: f32, delta_y: f32 },
//
//     /// Mouse cursor has entered the window's app
//     MouseEnter { x: i32, y: i32 },
//
//     /// Mouse cursor has left the window's app
//     MouseLeft { x: i32, y: i32 },
//
//     /// Mouse was moved with this delta
//     MouseMotion { delta: (f64, f64) },
//
//     /// Keyboard's key is down
//     KeyDown { key: KeyCode },
//
//     /// Keyboard's key was released
//     KeyUp { key: KeyCode },
//
//     /// User's touch the screen
//     TouchStart { id: u64, x: f32, y: f32 },
//
//     /// User0s move the touch
//     TouchMove { id: u64, x: f32, y: f32 },
//
//     /// Touch event ends
//     TouchEnd { id: u64, x: f32, y: f32 },
//
//     /// The System cancelled the touch
//     TouchCancel { id: u64, x: f32, y: f32 },
//
//     /// Unicode char pressed
//     ReceivedCharacter(char),
//
//     #[cfg(feature = "drop_files")]
//     /// The user is dragging a file over the window
//     DragEnter {
//         path: Option<PathBuf>,
//         name: Option<String>,
//         mime: String,
//     },
//
//     #[cfg(feature = "drop_files")]
//     /// The user stops dragging any file over the window
//     DragLeft,
//
//     #[cfg(feature = "drop_files")]
//     /// A file was dropped into the window
//     Drop(DroppedFile),
//
//     /// Text copied to the clipboard
//     Copy,
//
//     /// Text cut to the clipboard
//     Cut,
//
//     /// Text pasted from the clipboard
//     Paste(String),
// }
//
// #[cfg(feature = "drop_files")]
// #[derive(Default, Debug, PartialEq, Eq, Clone)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct DroppedFile {
//     pub path: Option<PathBuf>,
//     pub name: String,
//     pub mime: String,
//
//     #[cfg(target_arch = "wasm32")]
//     #[cfg_attr(feature = "serde", serde(skip))]
//     pub file: Option<web_sys::File>,
// }
//
// /// Event iterator queue
// #[derive(Debug, Clone, Default)]
// pub struct EventIterator(VecDeque<Event>);
//
// impl EventIterator {
//     pub fn new() -> Self {
//         Default::default()
//     }
//
//     /// Remove and return the first element on the queue
//     pub fn pop_front(&mut self) -> Option<Event> {
//         self.0.pop_front()
//     }
//
//     /// Add an event at the end of the list
//     pub fn push(&mut self, evt: Event) {
//         self.0.push_back(evt);
//     }
//
//     /// Add an event at the beginning of the list
//     pub fn push_front(&mut self, evt: Event) {
//         self.0.push_front(evt);
//     }
//
//     /// Return the events and clear the list
//     pub fn take_events(&mut self) -> EventIterator {
//         EventIterator(std::mem::take(&mut self.0))
//     }
// }
//
// impl Iterator for EventIterator {
//     type Item = Event;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.pop_front()
//     }
// }
