use crate::app::{AppBuilder, BackendSystem, SetupHandler};

#[cfg(not(feature = "backend"))]
use crate::app::empty::EmptyBackend as DefaultBackend;

#[cfg(feature = "backend")]
use notan_backend::DefaultBackend;

/// Initialize the app with the default backend and with an empty state
pub fn init() -> AppBuilder<(), DefaultBackend> {
    init_with::<(), fn() -> (), ()>(|| {})
}

/// Initialize the app with a custom state and the default backend
pub fn init_with<S, H, Params>(setup: H) -> AppBuilder<S, DefaultBackend>
where
    S: 'static,
    H: SetupHandler<S, Params>,
{
    init_with_backend(setup, DefaultBackend::new().unwrap())
}

/// Initialize the app using a custom state and a custom backend implementation
pub fn init_with_backend<S, B, H, Params>(setup: H, backend: B) -> AppBuilder<S, B>
where
    S: 'static,
    B: BackendSystem + 'static,
    H: SetupHandler<S, Params>,
{
    #[cfg(feature = "log_enabled")]
    {
        #[cfg(debug_assertions)]
        crate::log::init();

        #[cfg(not(debug_assertions))]
        crate::log::init_with_level(crate::log::Level::Warn);
    }

    AppBuilder::new(setup, backend)
}
