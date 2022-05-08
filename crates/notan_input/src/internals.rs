// functions or utils meant to be used for another notan_Crates but not to be used by the users
// nothing here should be re-exported by notan or the preludes

use crate::keyboard::Keyboard;
use crate::mouse::Mouse;
use crate::touch::Touch;
use notan_core::events::Event;

#[inline]
pub fn clear_mouse(mouse: &mut Mouse) {
    mouse.clear();
}

#[inline]
pub fn process_mouse_events(mouse: &mut Mouse, event: &Event, delta: f32) {
    mouse.process_events(event, delta);
}

#[inline]
pub fn clear_keyboard(keyboard: &mut Keyboard) {
    keyboard.clear();
}

#[inline]
pub fn process_keyboard_events(keyboard: &mut Keyboard, event: &Event, delta: f32) {
    keyboard.process_events(event, delta);
}

#[inline]
pub fn clear_touch(touch: &mut Touch) {
    touch.clear();
}

#[inline]
pub fn process_touch_events(touch: &mut Touch, event: &Event, delta: f32) {
    touch.process_events(event, delta);
}
