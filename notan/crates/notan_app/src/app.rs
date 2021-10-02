use crate::keyboard::Keyboard;
use crate::mouse::Mouse;
use crate::timer::AppTimer;
use crate::{Backend, WindowBackend};

/// Represents the state of the application, always accessible across the event's cycle
pub trait AppState {}
impl AppState for () {}

/// Represents the context of the application
pub struct App {
    pub backend: Box<dyn Backend>,
    pub mouse: Mouse,
    pub keyboard: Keyboard,
    pub timer: AppTimer,
    pub delta: f32,
    pub(crate) closed: bool,
}

impl App {
    pub(crate) fn new(backend: Box<dyn Backend>) -> Self {
        let mouse = Default::default();
        let keyboard = Default::default();
        Self {
            backend,
            mouse,
            keyboard,
            timer: AppTimer::default(),
            delta: 0.0,
            closed: false,
        }
    }

    #[inline]
    pub fn date_now(&self) -> u64 {
        self.backend.system_timestamp()
    }

    #[inline]
    pub fn exit(&mut self) {
        self.closed = true;
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
            .ok_or_else(|| "Invalid backend type.".to_string())
    }
}
