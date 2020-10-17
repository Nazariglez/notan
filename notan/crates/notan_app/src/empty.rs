use crate::{Backend, InitializeFn, App, WindowBackend};

#[derive(Default)]
pub struct EmptyWindowBackend {
    size: (i32, i32),
    is_fullscreen: bool,
}

impl WindowBackend for EmptyWindowBackend {
    fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
    }

    fn size(&self) -> (i32, i32) {
        self.size
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        self.is_fullscreen = enabled;
    }

    fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }
}

#[derive(Default)]
pub struct EmptyBackend {
    exit_requested: bool,
    window: EmptyWindowBackend,
}

impl Backend for EmptyBackend {
    type Impl = EmptyBackend;
    type Window = EmptyWindowBackend;

    fn get_impl(&mut self) -> &mut Self::Impl {
        self
    }

    fn initialize<B, S, R>(&mut self) -> Result<Box<InitializeFn<B, S, R>>, String>
        where
            B: Backend<Impl = Self::Impl> + 'static,
            S: 'static,
            R: FnMut(&mut App<B>, &mut S) + 'static,
    {
        Ok(Box::new(|mut app: App<B>, mut state: S, mut cb: R| {
            loop {
                cb(&mut app, &mut state);

                let backend = app.backend.get_impl();
                if backend.exit_requested {
                    break;
                }
            }
            Ok(())
        }))
    }

    fn window(&mut self) -> &mut Self::Window {
        &mut self.window
    }

    fn exit(&mut self) {
        self.exit_requested = true;
    }
}
