use hashbrown::{HashMap, HashSet};
use notan_core::events::Event;

#[derive(Default)]
/// Represent the touches data
pub struct Touch {}

impl Touch {
    #[inline]
    pub(crate) fn clear(&mut self) {}

    #[inline]
    pub(crate) fn process_events(&mut self, evt: &Event, delta: f32) {
        match evt {
            Event::TouchStart { id, x, y } => {}
            Event::TouchMove { id, x, y } => {}
            Event::TouchEnd { id, x, y } => {}
            Event::TouchCancel { id, x, y } => {}
            _ => {}
        }
    }
}
