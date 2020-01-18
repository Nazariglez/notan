use hashbrown::{HashMap, HashSet};
use nae_core::{Event, KeyCode, MouseButton};

pub struct Mouse {
    pub x: f32,
    pub y: f32,
    pub pressed: HashSet<MouseButton>,
    pub down: HashMap<MouseButton, f32>,
    pub released: HashSet<MouseButton>,
}

impl Mouse {
    pub(crate) fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            pressed: HashSet::new(),
            down: HashMap::new(),
            released: HashSet::new(),
        }
    }

    pub fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn was_released(&self, btn: MouseButton) -> bool {
        self.released.contains(&btn)
    }

    pub fn is_down(&self, btn: MouseButton) -> bool {
        self.down.contains_key(&btn)
    }

    pub fn down_delta(&self, btn: MouseButton) -> f32 {
        *self.down.get(&btn).unwrap_or(&0.0)
    }

    pub fn was_pressed(&self, btn: MouseButton) -> bool {
        self.pressed.contains(&btn)
    }

    pub(crate) fn clear(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }

    #[inline]
    pub(crate) fn process(&mut self, evt: &Event, delta: f32) {
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

pub struct Keyboard {
    pub pressed: HashSet<KeyCode>,
    pub down: HashMap<KeyCode, f32>,
    pub released: HashSet<KeyCode>,
}

impl Keyboard {
    pub(crate) fn new() -> Self {
        Self {
            pressed: HashSet::new(),
            down: HashMap::new(),
            released: HashSet::new(),
        }
    }

    pub fn was_released(&self, key: KeyCode) -> bool {
        self.released.contains(&key)
    }

    pub fn is_down(&self, key: KeyCode) -> bool {
        self.down.contains_key(&key)
    }

    pub fn down_delta(&self, key: KeyCode) -> f32 {
        *self.down.get(&key).unwrap_or(&0.0)
    }

    pub fn was_pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub(crate) fn clear(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }

    #[inline]
    pub(crate) fn process(&mut self, evt: &Event, delta: f32) {
        match evt {
            Event::KeyUp { key } => {
                self.down.remove(key);
                self.pressed.remove(key);
                self.released.insert(*key);
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
