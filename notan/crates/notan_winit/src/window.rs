use crate::messages::*;
use crossbeam_channel::{Receiver, Sender};
use notan_app::{App, Backend, InitializeFn, WindowBackend};
use winit::dpi::LogicalSize;
use winit::event::DeviceEvent::Button;
use winit::event::Event::DeviceEvent;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::macos::WindowExtMacOS;
use winit::window::{Fullscreen, Window, WindowBuilder};

pub struct WinitWindowBackend {
    pub(crate) sender: Sender<BackendMessages>,
    pub(crate) receiver: Receiver<BackendMessages>,
    pub(crate) is_fullscreen: bool,
    pub(crate) size: (i32, i32),
}

impl WindowBackend for WinitWindowBackend {
    fn set_size(&mut self, width: i32, height: i32) {
        if self
            .sender
            .send(BackendMessages::Size { width, height })
            .is_ok()
        {
            self.size = (width, height);
        }
    }

    fn size(&self) -> (i32, i32) {
        self.size
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        if self
            .sender
            .send(BackendMessages::FullscreenMode(enabled))
            .is_ok()
        {
            self.is_fullscreen = enabled;
        }
    }

    fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }
}
