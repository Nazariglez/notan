use crate::keyboard::Keyboard;
use crate::mouse::Mouse;
use crate::timer::AppTimer;
use crate::{Backend, WindowBackend};

/// Represents the state of the application, always accessible across the event's cycle
pub trait AppState {}
impl AppState for () {}

/// Represents the context of the application
pub struct App {
    /// Backend implementation
    pub backend: Box<dyn Backend>,

    /// Mouse data
    pub mouse: Mouse,

    /// Keyboard data
    pub keyboard: Keyboard,

    /// System timer
    pub system_timer: AppTimer,

    /// App timer
    pub timer: AppTimer,

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
            system_timer: AppTimer::default(),
            timer: AppTimer::default(),
            closed: false,
        }
    }

    #[inline]
    #[cfg(feature = "links")]
    pub fn open_link(&self, url: &str) {
        self.backend.open_link(url, false);
    }

    #[inline]
    #[cfg(feature = "links")]
    pub fn open_link_new_tab(&self, url: &str) {
        self.backend.open_link(url, true);
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
