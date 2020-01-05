use crate::res::{ResourceLoaderManager, ResourceParser};
use backend::*;
use nae_core::resources::*;
use nae_core::{BaseSystem, BuilderOpts};

/*TODO
    - Custom Error like Nae::NotFound, Nae::GraphicsX
    - math functions like random, random::seed() (crossplatform)
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

    pub fn delta(&self) -> f32 {
        1.0
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
}

impl<S> AppBuilder<S> {
    pub fn build(&mut self) -> Result<(), String> {
        let sys = System::new(BuilderOpts::default())?;

        let mut app = App {
            sys: sys,
            resources: ResourceLoaderManager::new(),
        };

        let mut state = (self.state_cb)(&mut app);
        let draw_cb = self.draw_callback.take().unwrap_or(|_, _| {});
        let update_cb = self.update_callback.take().unwrap_or(|_, _| {});
        let start_cb = self.start_callback.take().unwrap_or(|_, _| {});

        start_cb(&mut app, &mut state);
        backend::run(app, move |mut app| {
            try_load_resources(&mut app).unwrap();

            update_cb(&mut app, &mut state);
            draw_cb(&mut app, &mut state);
            app.system().swap_buffers();
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

    pub fn resource(&mut self, _cb: fn(&mut App, &mut S, res: &str)) -> &mut Self {
        //TODO call this every time a new resource is loaded
        self
    }

    pub fn update(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.update_callback = Some(cb);
        self
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
    AppBuilder {
        state_cb: |_| (),
        draw_callback: None,
        update_callback: None,
        start_callback: None,
    }
}

pub fn with_state<S>(cb: fn(&mut App) -> S) -> AppBuilder<S> {
    AppBuilder {
        state_cb: cb,
        draw_callback: None,
        update_callback: None,
        start_callback: None,
    }
}
