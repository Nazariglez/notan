use crate::app::{AppBuilder, BackendSystem, SetupHandler};

#[cfg(not(feature = "backend"))]
use crate::app::empty::EmptyBackend as DefaultBackend;

#[cfg(feature = "backend")]
use notan_backend::DefaultBackend;
#[cfg(feature = "log")]
use notan_log::LogConfig;

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
    let builder = AppBuilder::new(setup, backend);
    #[cfg(feature = "log")]
    let builder = builder.add_config(LogConfig::default());
    builder
}
