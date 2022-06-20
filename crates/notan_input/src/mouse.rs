use hashbrown::{HashMap, HashSet};
use notan_core::events::Event;

pub use notan_core::mouse::MouseButton;
use notan_math::Vec2;

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
    /// wheel delta
    pub wheel_delta: Vec2,
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
    /// Returns a local position
    pub fn local_position(&self, m: notan_math::Mat3) -> (f32, f32) {
        let pos = notan_math::mat3_screen_to_local(self.x, self.y, m);
        (pos.x, pos.y)
    }

    #[inline]
    /// Returns true if the button was released on the last frame
    pub fn was_released(&self, btn: MouseButton) -> bool {
        self.released.contains(&btn)
    }

    #[inline(always)]
    /// Returns true if the left button was released on the last frame
    pub fn left_was_released(&self) -> bool {
        self.was_released(MouseButton::Left)
    }

    #[inline(always)]
    /// Returns true if the middle button was released on the last frame
    pub fn middle_was_released(&self) -> bool {
        self.was_released(MouseButton::Middle)
    }

    #[inline(always)]
    /// Returns true if the right button was released on the last frame
    pub fn right_was_released(&self) -> bool {
        self.was_released(MouseButton::Right)
    }

    #[inline]
    /// Returns true if the button is still down
    pub fn is_down(&self, btn: MouseButton) -> bool {
        self.down.contains_key(&btn)
    }

    #[inline(always)]
    /// Returns true if the left button is still down
    pub fn left_is_down(&self) -> bool {
        self.is_down(MouseButton::Left)
    }

    #[inline(always)]
    /// Returns true if the middle button is still down
    pub fn middle_is_down(&self) -> bool {
        self.is_down(MouseButton::Middle)
    }

    #[inline(always)]
    /// Returns true if the right button is still down
    pub fn right_is_down(&self) -> bool {
        self.is_down(MouseButton::Right)
    }

    #[inline]
    /// Returns the total time that this button is down
    pub fn down_delta(&self, btn: MouseButton) -> f32 {
        *self.down.get(&btn).unwrap_or(&0.0)
    }

    #[inline]
    /// Returns true if the button was pressed on the last frame
    pub fn was_pressed(&self, btn: MouseButton) -> bool {
        self.pressed.contains(&btn)
    }

    #[inline(always)]
    /// Returns true if the left button was pressed on the last frame
    pub fn left_was_pressed(&self) -> bool {
        self.was_pressed(MouseButton::Left)
    }

    #[inline(always)]
    /// Returns true if the middle button was pressed on the last frame
    pub fn middle_was_pressed(&self) -> bool {
        self.was_pressed(MouseButton::Middle)
    }

    #[inline(always)]
    /// Returns true if the right button was pressed on the last frame
    pub fn right_was_pressed(&self) -> bool {
        self.was_pressed(MouseButton::Right)
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }

    #[inline]
    pub(crate) fn process_events(&mut self, evt: &Event, delta: f32) {
        self.wheel_delta.x = 0.0;
        self.wheel_delta.y = 0.0;

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
            Event::MouseWheel { delta_x, delta_y } => {
                self.wheel_delta.x = *delta_x;
                self.wheel_delta.y = *delta_y;
            }
            _ => {}
        }
    }
}
