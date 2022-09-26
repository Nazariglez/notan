use hashbrown::{HashMap, HashSet};
use notan_core::events::Event;

pub use notan_core::keyboard::KeyCode;

#[derive(Default)]
/// Represent the keyboard data
pub struct Keyboard {
    /// pressed keys
    pub pressed: HashSet<KeyCode>,
    /// down keys
    pub down: HashMap<KeyCode, f32>,
    /// released keys
    pub released: HashSet<KeyCode>,
    /// last key release
    pub last_key_released: Option<KeyCode>,
}

impl Keyboard {
    /// returns the last key released
    pub fn last_key_released(&self) -> Option<KeyCode> {
        self.last_key_released
    }

    /// returns true if the key was released on the last frame
    pub fn was_released(&self, key: KeyCode) -> bool {
        self.released.contains(&key)
    }

    /// returns true if the key is still down
    pub fn is_down(&self, key: KeyCode) -> bool {
        self.down.contains_key(&key)
    }

    /// returns the total time that this key is down
    pub fn down_delta(&self, key: KeyCode) -> f32 {
        *self.down.get(&key).unwrap_or(&0.0)
    }

    /// returns true if the key was pressed on the last frame
    pub fn was_pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    #[inline]
    /// returns true if any control key is down
    pub fn ctrl(&self) -> bool {
        self.is_down(KeyCode::RControl) || self.is_down(KeyCode::LControl)
    }

    #[inline]
    /// returns true if any alt key is down
    pub fn alt(&self) -> bool {
        self.is_down(KeyCode::RAlt) || self.is_down(KeyCode::LAlt)
    }

    #[inline]
    /// returns true if any shift key is down
    pub fn shift(&self) -> bool {
        self.is_down(KeyCode::RShift) || self.is_down(KeyCode::LShift)
    }

    #[inline]
    /// returns true if any logo (win or command) key is down
    pub fn logo(&self) -> bool {
        self.is_down(KeyCode::RWin) || self.is_down(KeyCode::LWin)
    }

    pub(crate) fn clear(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }

    #[inline]
    pub(crate) fn process_events(&mut self, evt: &Event, delta: f32) {
        match evt {
            Event::KeyUp { key } => {
                self.down.remove(key);
                self.pressed.remove(key);
                self.released.insert(*key);
                self.last_key_released = Some(*key);
            }

            Event::KeyDown { key } => {
                if let Some(t) = self.down.get_mut(key) {
                    *t += delta;
                } else {
                    self.down.insert(*key, 0.0);
                    self.pressed.insert(*key);
                }
            }
            _ => {}
        }
    }
}
