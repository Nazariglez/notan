use crate::keyboard::*;
use crate::mouse::*;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
/// Application events usually received from the user
pub enum Event {
    /// When the app is about to close
    Exit,

    /// Reoresents the current window's position after it was moved
    WindowMove { x: i32, y: i32 },

    /// Represents the current window's size after it was resized
    WindowResize { width: i32, height: i32 },

    /// Represents a change on the screen aspect ration
    ScreenAspectChange { ratio: f64 },

    /// Represents the current's mouse position after it was moved
    MouseMove { x: i32, y: i32 },

    /// A mouse button is down on this position
    MouseDown { button: MouseButton, x: i32, y: i32 },

    /// A mouse button was released on this position
    MouseUp { button: MouseButton, x: i32, y: i32 },

    /// The mouse wheel was moved with this delta
    MouseWheel { delta_x: f32, delta_y: f32 },

    /// Mouse cursor has entered the window's app
    MouseEnter { x: i32, y: i32 },

    /// Mouse cursos has left the window's app
    MouseLeft { x: i32, y: i32 },

    /// Keyboard's key is down
    KeyDown { key: KeyCode },

    /// Keyboard's key was released
    KeyUp { key: KeyCode },

    /// Unicode char pressed
    ReceivedCharacter(char),
}

#[derive(Debug, Clone, Default)]
/// Event iterator queue
pub struct EventIterator(VecDeque<Event>);

impl EventIterator {
    pub fn new() -> Self {
        Default::default()
    }

    /// Remove and return the last element on the queue
    pub fn pop(&mut self) -> Option<Event> {
        self.0.pop_front()
    }

    /// Add an event at the beginning of the list
    pub fn push(&mut self, evt: Event) {
        self.0.push_back(evt);
    }

    /// Return the events and clear the list
    pub fn take_events(&mut self) -> EventIterator {
        EventIterator(std::mem::take(&mut self.0))
    }
}

impl Iterator for EventIterator {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
