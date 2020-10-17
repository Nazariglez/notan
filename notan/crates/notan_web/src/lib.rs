use notan_app::{Backend, App, WindowBackend, InitializeFn};

pub struct WebWindowBackend {

}

impl WindowBackend for WebWindowBackend {
    fn set_size(&mut self, width: i32, height: i32) {
        // unimplemented!()
    }

    fn size(&self) -> (i32, i32) {
        // unimplemented!()
        (800, 600)
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        // unimplemented!()
    }

    fn is_fullscreen(&self) -> bool {
        // unimplemented!()
        false
    }
}

pub struct WebBackend {
    window: WebWindowBackend,
    exit_requested: bool,
}

impl WebBackend {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            window: WebWindowBackend {},
            exit_requested: false,
        })
    }
}

impl Backend for WebBackend {
    type Impl = WebBackend;
    type Window = WebWindowBackend;

    fn get_impl(&mut self) -> &mut Self::Impl {
        self
    }

    fn initialize<B, S, R>(&mut self) -> Result<Box<InitializeFn<B, S, R>>, String> where
        B: Backend<Impl=Self::Impl> + 'static,
        S: 'static,
        R: FnMut(&mut App<B>, &mut S) + 'static {
        // unimplemented!()
        Ok(Box::new(|app, state, cb| {
            Ok(())
        }))
    }

    fn window(&mut self) -> &mut Self::Window {
        &mut self.window
    }

    fn exit(&mut self) {
        // unimplemented!()
    }
}