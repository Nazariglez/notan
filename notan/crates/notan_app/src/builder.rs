use crate::{App, AppConfig, Backend};

type BuildCallback<B, S> = fn(&mut App<B>, &mut S);

pub struct AppBuilder<S, B>
where
    B: Backend,
{
    state: S,
    backend: B,
    pub window: String,
    init_callback: Option<BuildCallback<B, S>>,
    update_callback: Option<BuildCallback<B, S>>,
    draw_callback: Option<BuildCallback<B, S>>,
    event_callback: Option<BuildCallback<B, S>>,
}

impl<S, B> AppBuilder<S, B>
where
    B: Backend + 'static,
    S: 'static,
{
    pub fn new(state: S, backend: B) -> Self {
        AppBuilder {
            state,
            backend,
            window: "Yeah".to_string(),
            init_callback: None,
            update_callback: None,
            draw_callback: None,
            event_callback: None,
        }
    }

    pub fn set_config(mut self, config: &dyn AppConfig<B, S>) -> Self {
        config.apply(&mut self);
        self
    }

    pub fn initialize(mut self, cb: BuildCallback<B, S>) -> Self {
        self.init_callback = Some(cb);
        self
    }

    pub fn update(mut self, cb: BuildCallback<B, S>) -> Self {
        self.update_callback = Some(cb);
        self
    }

    pub fn build(self) -> Result<(), String> {
        let AppBuilder {
            mut backend,
            mut state,
            init_callback,
            update_callback,
            draw_callback,
            event_callback,
            ..
        } = self;

        let initialize = backend.initialize()?;
        let mut app = App::new(backend);

        if let Some(cb) = init_callback {
            cb(&mut app, &mut state);
        }

        initialize(app, state, move |mut app, mut state| {
            app.tick();

            app.backend.events_iter().for_each(|evt| {
                app.mouse.process_events(&evt, app.delta);

                if let Some(cb) = event_callback {
                    cb(&mut app, &mut state);
                }
            });

            if let Some(cb) = update_callback {
                cb(&mut app, &mut state);
            }

            //TODO check frame here?
            if let Some(cb) = draw_callback {
                cb(&mut app, &mut state);
            }

            app.mouse.clear();
        })?;

        Ok(())
    }
}
