use super::event_loop::EventLoopPtr;
use super::window::Window;
use hashbrown::HashMap;
use notan_core::window::{NotanApp, NotanWindow, WindowConfig, WindowId};
use notan_core::Plugin;
pub use winit::event_loop::EventLoopWindowTarget;

pub struct Manager {
    pub windows: HashMap<WindowId, Window>,
    pub(crate) event_loop: EventLoopPtr,
    pub(crate) request_exit: bool,
}

impl Plugin for Manager {}

impl NotanApp<Window> for Manager {
    fn new() -> Result<Self, String> {
        Ok(Self {
            windows: HashMap::default(),
            event_loop: EventLoopPtr::new()?,
            request_exit: false,
        })
    }

    fn create(&mut self, attrs: WindowConfig) -> Result<WindowId, String> {
        // SAFETY: if it's `Some` means that we're inside the event's loop and this is available
        let event_loop = self.event_loop.inner();
        match event_loop {
            Some(event_loop) => {
                let win = Window::new(event_loop, attrs)?;
                let id = win.id();
                self.windows.insert(id, win);
                Ok(id)
            }
            None => Err("Cannot create window because EventLoop is not initialized".to_string()),
        }
    }

    fn window(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    fn close(&mut self, id: WindowId) -> bool {
        self.windows.remove(&id).is_some()
    }

    fn exit(&mut self) {
        self.request_exit = true;
    }
}
