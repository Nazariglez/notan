use crate::events::{self, EventListener, EventMap};
use crate::handlers::{EventHandlerFn, EventHandlerFnOnce};
use crate::state::AppState;
use crate::storage::Storage;
use crate::window::WindowId;
use std::any::TypeId;

/// The core of the application, all the systems and backend interacts with it somehow
pub struct System<S: AppState + 'static> {
    pub(crate) storage: Storage<S>,
    pub(crate) event_handler: EventMap,
    pub(crate) initialized: bool,
    pub(crate) in_frame: bool,
    pub(crate) closed: bool,
}

impl<S: AppState> System<S> {
    /// Allows mutable access to a plugin stored
    pub fn get_mut_plugin<T: 'static>(&mut self) -> Option<&mut T> {
        self.storage.plugins.get_mut()
    }

    /// It's called when the backend is ready
    /// it dispatched the event `Init`
    pub fn init(&mut self) {
        if self.initialized {
            return;
        }

        self.initialized = true;
        self.event(events::InitEvent);
    }

    pub fn frame_start(&mut self) {
        if self.in_frame {
            return;
        }

        self.in_frame = true;
        self.event(events::FrameStartEvent);
    }

    pub fn frame_end(&mut self) {
        if !self.in_frame {
            return;
        }

        self.event(events::FrameEndEvent);
        self.in_frame = false;
    }

    fn exec_event_callback<E: Send + Sync + std::fmt::Debug + 'static>(
        &mut self,
        evt: &E,
        idx: usize,
    ) -> Result<bool, String> {
        let listener = self
            .event_handler
            .get_mut(&TypeId::of::<E>())
            .and_then(|list| list.get_mut(idx))
            .ok_or_else(|| {
                format!(
                    "Callback {} for event {:?} cannot be found",
                    idx,
                    TypeId::of::<E>()
                )
            })?;

        let mut needs_clean = false;
        match listener {
            EventListener::Once(_, cb_opt) => {
                if let Some(cb) = cb_opt.take() {
                    let cb = cb.downcast::<Box<EventHandlerFnOnce<E, S>>>();
                    if let Ok(cb) = cb {
                        cb(&mut self.storage, evt);
                        needs_clean = true;
                    }
                }
            }
            EventListener::Mut(_, cb) => {
                let cb = cb.downcast_mut::<Box<EventHandlerFn<E, S>>>();
                if let Some(cb) = cb {
                    cb(&mut self.storage, evt);
                }
            }
        }
        execute_queued_events(self);
        Ok(needs_clean)
    }

    /// Execute any listener set for the event passed in
    pub fn event<E: Send + Sync + std::fmt::Debug + 'static>(&mut self, evt: E) {
        println!("> Set EVENT {:?}", evt);
        if !self.initialized {
            return;
        }

        let len = self
            .event_handler
            .get(&TypeId::of::<E>())
            .map_or(0, |list| list.len());

        // There is a bad thing about this event system. There is a double indirection
        // because we need to fetch the list of events and then call id per callback
        // this is because we cannot get the list and execute the callbacks with a forloop
        // due borrow checker issues when pushing events inside event callbacks

        if len != 0 {
            // clean once events once all callback has been executed
            let mut needs_clean = false;

            for idx in 0..len {
                let once = match self.exec_event_callback(&evt, idx) {
                    Ok(needs_clean) => needs_clean,
                    Err(err) => {
                        log::error!("Error with event '{:?}': {}", evt, err);
                        false
                    }
                };

                if once {
                    needs_clean = true;
                }
            }

            if needs_clean {
                if let Some(list) = self.event_handler.get_mut(&TypeId::of::<E>()) {
                    list.retain(|listener| !listener.is_once());
                }
            }
        }
    }

    /// It's called each frame by the backend and it dispatches
    /// the event `Update`.
    pub fn update(&mut self) {
        let frame_running = self.initialized && self.in_frame;
        if !frame_running {
            return;
        }

        if self.closed {
            return;
        }

        self.event(events::UpdateEvent);
    }

    /// It's called when the backend/app is about to close
    /// it dispatched the events `RequestedClose` and `Close`
    pub fn close(&mut self) {
        if !self.initialized {
            return;
        }

        if self.closed {
            return;
        }

        self.event(events::RequestCloseEvent);
        self.closed = true;
        self.event(events::CloseEvent);
    }
}

#[inline]
fn execute_queued_events<S: AppState + 'static>(app: &mut System<S>) {
    while let Some(cb) = app.storage.take_event() {
        cb(app);
    }
}
