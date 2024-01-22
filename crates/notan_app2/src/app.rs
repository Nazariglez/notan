use crate::{Manager, PlatformConfig, Window};
use hashbrown::hash_map::{Values, ValuesMut};
use notan_core::window::{NotanApp, WindowAttributes, WindowId};
use notan_core::Plugin;

pub struct App {
    pub manager: Manager,
    main_window: Option<WindowId>,
    window_ids: Vec<WindowId>,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            manager: Manager::new()?,
            main_window: None,
            window_ids: vec![],
        })
    }

    pub fn config() -> PlatformConfig {
        PlatformConfig::default()
    }

    pub fn create_window(&mut self, attrs: WindowAttributes) -> Result<WindowId, String> {
        let id = self.manager.create(attrs)?;
        self.window_ids.push(id);

        // set this as main windows if there is no one
        if self.main_window.is_none() {
            self.main_window = Some(id);
        }

        Ok(id)
    }

    pub fn window(&mut self, id: WindowId) -> Option<&mut Window> {
        self.manager.window(id)
    }

    pub fn main_window(&mut self) -> Option<&mut Window> {
        self.main_window.and_then(|id| self.window(id))
    }

    pub fn set_main_window(&mut self, win_id: WindowId) {
        self.main_window = Some(win_id);
    }

    pub fn window_ids(&self) -> &[WindowId] {
        &self.window_ids
    }

    pub fn windows(&self) -> Values<'_, WindowId, Window> {
        self.manager.windows.values()
    }

    pub fn windows_mut(&mut self) -> ValuesMut<'_, WindowId, Window> {
        self.manager.windows.values_mut()
    }

    pub fn close(&mut self, id: WindowId) {
        let closed = self.manager.close(id);
        if closed {
            // remove from the window_id list
            let pos = self
                .window_ids
                .iter()
                .position(|stored_id| *stored_id == id);

            if let Some(pos) = pos {
                self.window_ids.remove(pos);
            }

            // set main window as none if this was the main window
            if self.main_window == Some(id) {
                self.main_window = None;
            }
        }
    }

    pub fn exit(&mut self) {
        self.manager.exit();
    }
}

impl Plugin for App {}
