use crate::input::{Keyboard, Mouse};
use crate::res::{ResourceLoaderManager, ResourceParser};
use backend::*;
use nae_core::resources::*;
use nae_core::{BaseSystem, BuilderOpts, Event};
use std::collections::VecDeque;

/*TODO
    - Custom Error like Nae::NotFound, Nae::GraphicsX
    - use rayon when it's necessary for example processing the batch before draw
    -
*/

/*TODO avoid to skip the draw callback:
    returning from update: DrawState::Skip (to draw DrawState::Draw)
    or from a function on the app: app.skip_next_draw(); //app.resume_next_draw() to cancel?
    --
    This is useful for GUI systems, and mobile devices, to save battery.
*/

//TODO backend requirements for resvg https://github.com/RazrFalcon/resvg/blob/master/docs/backend_requirements.md

pub struct App {
    resources: ResourceLoaderManager<'static>,
    sys: System,
    fps: VecDeque<f64>,
    last_time: u64,

    pub delta: f32,
    pub mouse: Mouse,
    pub keyboard: Keyboard,
    pub time: f32,
}

impl BaseApp for App {
    type System = System;

    fn system(&mut self) -> &mut Self::System {
        &mut self.sys
    }
}

impl App {
    pub fn draw(&mut self) -> &mut Context2d {
        self.sys.ctx2()
    }

    pub fn load_file<A>(&mut self, file: &str) -> Result<A, String>
    where
        A: ResourceParser<App = Self> + Resource + Clone + 'static,
    {
        self.resources.add(file)
    }

    fn tick(&mut self) {
        let now = date_now();
        let elapsed = (now - self.last_time) as f64;
        self.last_time = now;
        self.delta = (elapsed / 1000.0) as f32;
        self.time += self.delta;
        self.fps.pop_front();
        self.fps.push_back(elapsed);
    }

    pub fn fps(&self) -> f64 {
        let average: f64 = self.fps.iter().sum::<f64>() / self.fps.len() as f64;
        1000.0 / average
    }
}

pub struct AppBuilder<S>
where
    S: 'static,
{
    state_cb: fn(&mut App) -> S,
    draw_callback: Option<fn(&mut App, &mut S)>,
    update_callback: Option<fn(&mut App, &mut S)>,
    start_callback: Option<fn(&mut App, &mut S)>,
    event_callback: Option<fn(&mut App, &mut S, event: Event)>,
    options: BuilderOpts,
}

impl<S> AppBuilder<S> {
    pub fn build(&mut self) -> Result<(), String> {
        let sys = System::new(self.options.clone())?;

        let mut fps = VecDeque::with_capacity(300);
        fps.resize(fps.capacity(), 1000.0 / 60.0);

        let mut app = App {
            sys: sys,
            resources: ResourceLoaderManager::new(),
            fps: fps,
            last_time: date_now(),
            delta: 0.0,
            time: 0.0,
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
        };

        let mut state = (self.state_cb)(&mut app);
        let draw_cb = self.draw_callback.take().unwrap_or(|_, _| {});
        let update_cb = self.update_callback.take().unwrap_or(|_, _| {});
        let start_cb = self.start_callback.take().unwrap_or(|_, _| {});
        let event_cb = self.event_callback.take().unwrap_or(|_, _, _| {});

        start_cb(&mut app, &mut state);
        backend::run(
            app,
            state,
            move |mut app, mut state| {
                app.tick();
                try_load_resources(&mut app).unwrap();
                process_events(app, state, event_cb);
                update_cb(&mut app, &mut state);
            },
            move |mut app, mut state| {
                draw_cb(&mut app, &mut state);
            },
        );

        Ok(())
    }

    pub fn size(&mut self, width: i32, height: i32) -> &mut Self {
        self.options.width = width;
        self.options.height = height;
        self
    }

    pub fn min_size(&mut self, width: i32, height: i32) -> &mut Self {
        self.options.min_size = Some((width, height));
        self
    }

    pub fn max_size(&mut self, width: i32, height: i32) -> &mut Self {
        self.options.max_size = Some((width, height));
        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.options.title = title.to_string();
        self
    }

    pub fn fullscreen(&mut self) -> &mut Self {
        self.options.fullscreen = true;
        self
    }

    pub fn maximized(&mut self) -> &mut Self {
        self.options.maximized = true;
        self
    }

    pub fn icon(&mut self) -> &mut Self {
        //TODO set window icon
        unimplemented!()
    }

    pub fn resizable(&mut self) -> &mut Self {
        self.options.resizable = true;
        self
    }

    pub fn draw(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.draw_callback = Some(cb);
        self
    }

    pub fn start(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.start_callback = Some(cb);
        self
    }

    pub fn update(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.update_callback = Some(cb);
        self
    }

    pub fn event(&mut self, cb: fn(&mut App, &mut S, event: Event)) -> &mut Self {
        self.event_callback = Some(cb);
        self
    }

    pub fn fps_target(&mut self, fps: i32) -> &mut Self {
        self.options.fps_target = Some(fps);
        self
    }
}

fn process_events<S>(app: &mut App, state: &mut S, cb: fn(&mut App, &mut S, Event)) {
    app.mouse.clear();
    app.keyboard.clear();
    let mut events = app.sys.events().take_events();
    for evt in events {
        app.mouse.process(&evt, app.delta);
        app.keyboard.process(&evt, app.delta);
        cb(app, state, evt);
    }
}

//TODO don't stop the loop, just return Vec<String> with the errors, and the user will decide what to do instead of stop the program
fn try_load_resources(app: &mut App) -> Result<(), String> {
    if let Some(mut assets_loaded) = app.resources.try_load()? {
        while let Some((data, mut asset)) = assets_loaded.pop() {
            if !asset.already_loaded() {
                asset.parse_res(app, data)?;
            }
        }
    }

    Ok(())
}

pub fn init() -> AppBuilder<()> {
    init_with(|_| ())
}

pub fn init_with<S>(cb: fn(&mut App) -> S) -> AppBuilder<S> {
    AppBuilder {
        state_cb: cb,
        draw_callback: None,
        update_callback: None,
        start_callback: None,
        event_callback: None,
        options: BuilderOpts::default(),
    }
}
