use crate::keyboard::Keyboard;
use crate::mouse::Mouse;
use crate::{Backend, LoadFileFn, WindowBackend};
use std::path::Path;
use std::sync::Arc;

/// Represents the state of the application, always accessible across the event's cycle
pub trait AppState {}

/// Represents the context of the application
pub struct App {
    pub backend: Box<Backend>,
    pub mouse: Mouse,
    pub keyboard: Keyboard,
    pub delta: f32,

    load_file: LoadFileFn,
}

impl App {
    pub(crate) fn new(backend: Box<Backend>, load_file: LoadFileFn) -> Self {
        let mouse = Default::default();
        let keyboard = Default::default();
        Self {
            backend,
            mouse,
            keyboard,
            delta: 0.0,
            load_file,
        }
    }

    pub fn tick(&mut self) {
        //TODO
    }

    #[inline]
    pub fn exit(&mut self) {
        self.backend.exit();
    }

    #[inline]
    pub fn window(&mut self) -> &mut WindowBackend {
        self.backend.window()
    }
}
