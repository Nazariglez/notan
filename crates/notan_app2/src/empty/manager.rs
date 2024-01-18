use super::window::Window;
use hashbrown::HashMap;
use notan_core::window::{CursorIcon, NotanApp, WindowAttributes, WindowId};

#[derive(Default)]
pub struct Manager {
    pub(crate) windows: HashMap<WindowId, Window>,
    pub(crate) request_exit: bool,
}

impl NotanApp<Window> for Manager {
    fn new() -> Self {
        Default::default()
    }

    fn create(&mut self, attrs: WindowAttributes) -> Result<WindowId, String> {
        let count = self.windows.len();
        let id: WindowId = (count as u64).into();
        let win = Window {
            id,
            size: attrs.size.unwrap_or((800, 600)),
            position: attrs.position.unwrap_or((0, 0)),
            title: attrs.title.clone(),
            cursor: CursorIcon::Default,
            resizable: attrs.resizable,
            min_size: None,
            max_size: None,
        };
        self.windows.insert(id, win);
        Ok(id)
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
