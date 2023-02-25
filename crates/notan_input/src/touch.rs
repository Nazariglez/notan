use hashbrown::{HashMap, HashSet};
use notan_core::events::Event;

/// Represent a pointer event
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Pointer {
    /// Pointer index
    _index: u8,
    /// Platform id set by the backend
    _platform_id: u64,
    /// x position
    x: f32,
    /// y position
    y: f32,
}

/// Represent the touches data
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Touch {
    /// pressed touches
    pub pressed: HashSet<u8>,
    /// down touches with delta time
    pub down: HashMap<u8, f32>,
    /// released touches
    pub released: HashSet<u8>,

    pointers: [Option<Pointer>; 20],
    pointer_index: HashMap<u64, usize>,
    logged_max_error: bool,
}

impl Touch {
    /// Returns a tuple with the x and y position
    #[inline]
    pub fn position(&self, id: u8) -> Option<(f32, f32)> {
        self.pointers.get(id as usize)?.as_ref().map(|p| (p.x, p.y))
    }

    /// Returns true if the touch was released on the last frame
    #[inline]
    pub fn was_released(&self, id: u8) -> bool {
        self.released.contains(&id)
    }

    /// Returns true if the touch was pressed on the last frame
    #[inline]
    pub fn was_pressed(&self, id: u8) -> bool {
        self.pressed.contains(&id)
    }

    /// Returns true if the touch is still down
    #[inline]
    pub fn down(&self, id: u8) -> bool {
        self.down.contains_key(&id)
    }

    /// Returns the total time that the touch has been down
    #[inline]
    pub fn down_delta(&self, id: u8) -> f32 {
        *self.down.get(&id).unwrap_or(&0.0)
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }

    #[inline]
    pub(crate) fn process_events(&mut self, evt: &Event, delta: f32) {
        match evt {
            Event::TouchStart { id, x, y } => {
                let empty_index = self.pointers.iter().position(|s| s.is_none());
                match empty_index {
                    None => {
                        if !self.logged_max_error {
                            self.logged_max_error = true;
                            log::warn!(
                                "Touch manager cannot support more than {} pointers.",
                                self.pointers.len()
                            );
                        }
                    }
                    Some(index) => {
                        self.pointer_index.insert(*id, index);
                        self.pointers[index] = Some(Pointer {
                            _index: index as _,
                            _platform_id: *id,
                            x: *x,
                            y: *y,
                        });

                        self.pressed.insert(index as _);
                        self.down.insert(index as _, 0.0);
                    }
                }
            }
            Event::TouchMove { id, x, y } => {
                if let Some(&index) = self.pointer_index.get(id) {
                    if let Some(pointer) = &mut self.pointers[index] {
                        pointer.x = *x;
                        pointer.y = *y;
                    }
                }
            }
            Event::TouchEnd { id, .. } => {
                self.clean_id(*id);
            }
            Event::TouchCancel { id, .. } => {
                self.clean_id(*id);
            }
            _ => {}
        }

        self.down.values_mut().for_each(|time| *time += delta);
    }

    fn clean_id(&mut self, id: u64) {
        if let Some(index) = self.pointer_index.remove(&id) {
            let index_u8: u8 = index as _;
            self.down.remove(&index_u8);
            self.pressed.remove(&index_u8);
            self.released.insert(index_u8);
            self.pointers[index] = None;
        }
    }
}
