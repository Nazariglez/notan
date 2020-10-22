use crate::mouse::Mouse;
use crate::{Backend, WindowBackend};

pub struct App<B: Backend> {
    pub backend: B,
    pub mouse: Mouse,
    pub delta: f32,
}

impl<B: Backend> App<B> {
    pub(crate) fn new(backend: B) -> Self {
        let mouse = Default::default();
        Self {
            backend,
            mouse,
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
    pub fn window(&mut self) -> &mut impl WindowBackend {
        self.backend.window()
    }
}
