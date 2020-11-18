use crate::keyboard::Keyboard;
use crate::mouse::Mouse;
use crate::{Backend, WindowBackend};

//TODO: looks like an interesting API to do the draw2d module https://github.com/RazrFalcon/tiny-skia/tree/master/examples

/// Represents the state of the application, always accessible across the event's cycle
pub trait AppState {}
impl AppState for () {}

/// Represents the context of the application
pub struct App {
    pub backend: Box<dyn Backend>,
    pub mouse: Mouse,
    pub keyboard: Keyboard,
    pub delta: f32,
}

impl App {
    pub(crate) fn new(backend: Box<dyn Backend>) -> Self {
        let mouse = Default::default();
        let keyboard = Default::default();
        Self {
            backend,
            mouse,
            keyboard,
            delta: 0.0,
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
    pub fn window(&mut self) -> &mut dyn WindowBackend {
        self.backend.window()
    }

    #[inline]
    /// Returns the backend downcasted to the real type (useful for custom backends)
    pub fn backend<T: Backend>(&mut self) -> Result<&mut T, String> {
        self.backend
            .downcast_mut::<T>()
            .ok_or("Invalid backend type.".to_string())
    }
}
