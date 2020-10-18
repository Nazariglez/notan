use crate::{Backend, WindowBackend};

pub struct App<B: Backend> {
    pub backend: B,
}

impl<B: Backend> App<B> {
    pub(crate) fn new(backend: B) -> Self {
        Self { backend }
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
