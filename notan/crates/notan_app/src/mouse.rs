use crate::Event;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
/// Represents a button of a mouse
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Default)]
/// Represent the mouse data
pub struct Mouse {
    /// x position
    pub x: f32,
    /// y position
    pub y: f32,
    /// pressed buttons
    pub pressed: HashSet<MouseButton>,
    /// down buttons with delta time
    pub down: HashMap<MouseButton, f32>,
    /// released buttons
    pub released: HashSet<MouseButton>,
}

impl Mouse {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    /// Returns a tuple with the x and y position
    pub fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    #[inline]
    /// Returns true if the button was released on the last frame
    pub fn was_released(&self, btn: MouseButton) -> bool {
        self.released.contains(&btn)
    }

    #[inline]
    /// Returns true if the button is still down
    pub fn is_down(&self, btn: MouseButton) -> bool {
        self.down.contains_key(&btn)
    }

    #[inline]
    /// Returns the total time that this button is down
    pub fn down_delta(&self, btn: MouseButton) -> f32 {
        *self.down.get(&btn).unwrap_or(&0.0)
    }

    #[inline]
    /// Returns true if the button was pressed on the last feame
    pub fn was_pressed(&self, btn: MouseButton) -> bool {
        self.pressed.contains(&btn)
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }

    #[inline]
    pub(crate) fn process_events(&mut self, evt: &Event, delta: f32) {
        match evt {
            Event::MouseMove { x, y } => {
                self.x = *x as f32;
                self.y = *y as f32;
            }

            Event::MouseUp { x, y, button } => {
                self.x = *x as f32;
                self.y = *y as f32;

                self.down.remove(button);
                self.pressed.remove(button);
                self.released.insert(*button);
            }

            Event::MouseDown { x, y, button } => {
                self.x = *x as f32;
                self.y = *y as f32;

                if let Some(t) = self.down.get_mut(button) {
                    *t += delta;
                } else {
                    self.down.insert(*button, 0.0);
                    self.pressed.insert(*button);
                }
            }
            _ => {}
        }
    }
}

//TODO touch
//TODO pointers (mix of mouse and touch)
