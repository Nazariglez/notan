use super::graphics::Context2d;
use super::res::*;
use super::{window, window::*};

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

pub struct App<'a> {
    pub(crate) window: Window,
    pub(crate) graphics: Context2d,
    resources: ResourceLoaderManager<'a>,
}

impl<'a> App<'a> {
    pub fn draw(&mut self) -> &mut Context2d {
        &mut self.graphics
    }

    pub fn load_file<A>(&mut self, file: &str) -> Result<A, String>
    where
        A: ResourceConstructor + Resource + Clone + 'a,
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
        let win = Window::new();
        let gfx = Context2d::new(win.window())?;

        let mut app = App {
            window: win,
            graphics: gfx,
            resources: ResourceLoaderManager::new(),
        };

        let mut state = (self.state_cb)(&mut app);
        let draw_cb = self.draw_callback.take().unwrap_or(|_, _| {});
        let update_cb = self.update_callback.take().unwrap_or(|_, _| {});
        let start_cb = self.start_callback.take().unwrap_or(|_, _| {});

        start_cb(&mut app, &mut state);
        window::run(move || {
            try_load_resources(&mut app).unwrap();

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
            if !asset.is_loaded() {
                asset.parse(app, data)?;
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
