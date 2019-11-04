use super::{window, window::*};
use super::graphics::Context;
use super::res::*;

pub struct App<'a> {
    pub(crate) window: Window,
    pub(crate) graphics: Context,
    pub(crate) resources: ResourceManager<'a>,
}

impl<'a> App<'a> {
    pub fn draw(&mut self) -> &mut Context {
        &mut self.graphics
    }

    pub fn load<A>(&mut self, file: &str) -> Result<A, String>
        where
            A: ResourceConstructor + Resource + Clone + 'a,
    {
        self.resources.load(file)
    }
}

pub struct AppBuilder<S>
    where
        S: 'static,
{
    state: Option<S>,
    draw_callback: Option<fn(&mut App, &mut S)>,
    update_callback: Option<fn(&mut App, &mut S)>,
    start_callback: Option<fn(&mut App, &mut S)>,
}

impl<S> AppBuilder<S> {
    pub fn build(&mut self) -> Result<(), String> {
        let win = Window::new();
        let gfx = Context::new(win.window())?;

        let mut app = App {
            window: win,
            graphics: gfx,
            resources: ResourceManager::new(),
        };

        let mut state = self.state.take().unwrap();
        let mut draw_cb = self.draw_callback.take().unwrap_or(|_, _| {});
        let mut update_cb = self.update_callback.take().unwrap_or(|_, _| {});
        let mut start_cb = self.start_callback.take().unwrap_or(|_, _| {});

        start_cb(&mut app, &mut state);
        window::run(move || {
            app.resources.try_load().unwrap();

            update_cb(&mut app, &mut state);
            draw_cb(&mut app, &mut state);
        });
        Ok(())
    }

    pub fn draw(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.draw_callback = Some(cb);
        self
    }

    pub fn start(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.start_callback = Some(cb);
        self
    }

    pub fn resource(&mut self, cb: fn(&mut App, &mut S, res: &str)) -> &mut Self {
        //TODO call this every time a new resource is loaded
        self
    }

    pub fn update(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.update_callback = Some(cb);
        self
    }
}

pub fn init<S>(state: S) -> AppBuilder<S> {
    AppBuilder {
        state: Some(state),
        draw_callback: None,
        update_callback: None,
        start_callback: None,
    }
}

