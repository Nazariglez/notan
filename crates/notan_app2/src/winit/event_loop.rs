use std::ops::Deref;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};

pub(crate) enum EventLoopPtr {
    Main(EventLoop<()>),
    Ptr(*const EventLoopWindowTarget<()>),
    None,
}

impl EventLoopPtr {
    pub fn new() -> Self {
        Self::Main(EventLoop::new())
    }

    pub fn take(&mut self) -> Option<EventLoop<()>> {
        if matches!(self, Self::Main(_)) {
            let value = std::mem::replace(self, Self::None);
            if let Self::Main(event_loop) = value {
                return Some(event_loop);
            }
        }

        None
    }

    pub fn inner(&self) -> Option<&EventLoopWindowTarget<()>> {
        match self {
            EventLoopPtr::Main(el) => Some(el.deref()),
            // SAFETY: if the enum is Ptr is because this is assigned inside the event_loop otherwise is None
            EventLoopPtr::Ptr(ptr) => unsafe { Some(cast_ptr(*ptr)) },
            EventLoopPtr::None => None,
        }
    }

    pub fn set(&mut self, event_loop: &EventLoopWindowTarget<()>) {
        *self = Self::Ptr(event_loop as *const _);
    }

    pub fn unset(&mut self) {
        *self = Self::None;
    }
}

unsafe fn cast_ptr<'a>(ptr: *const EventLoopWindowTarget<()>) -> &'a EventLoopWindowTarget<()> {
    &*ptr
}
