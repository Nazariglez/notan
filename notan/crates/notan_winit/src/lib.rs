use notan_app::{App, Backend};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct WinitBackend {
    window: Window,
    event_loop: Option<EventLoop<()>>,
}

impl WinitBackend {
    pub fn new() -> Result<Self, String> {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("yeah")
            .build(&event_loop)
            .map_err(|e| format!("{:?}", e))?;

        Ok(Self { window, event_loop: Some(event_loop) })
    }
}

impl Backend for WinitBackend {
    fn runner<B: Backend, S, R: FnMut(&mut App<B>, &mut S)>(
        &mut self,
    ) -> Box<Fn(App<B>, S, R) -> Result<(), String>> {
        Box::new(move |mut app: App<B>, mut state: S, mut cb: R| {
            loop {
                cb(&mut app, &mut state);

                if app.exit_was_requested() {
                    break;
                }
            }
            Ok(())
        })
    }
}
