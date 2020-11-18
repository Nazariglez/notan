use crate::app::assets::AssetManager;
use crate::app::{App, AppBuilder, BackendSystem};

#[cfg(not(feature = "default_backend"))]
use crate::app::empty::EmptyBackend as DefaultBackend;

#[cfg(feature = "default_backend")]
use notan_default_backend::DefaultBackend;

/// Initialize the app with the default backend and with an empty state
pub fn init() -> AppBuilder<(), DefaultBackend> {
    init_with(|_, _| {})
}

/// Initialize the app with a custom state and the default backend
pub fn init_with<S>(state: fn(&mut App, &mut AssetManager) -> S) -> AppBuilder<S, DefaultBackend>
where
    S: 'static,
{
    init_with_backend(state, DefaultBackend::new().unwrap())
}

/// Initialize the app using a custom state and a custom backend implementation
pub fn init_with_backend<S, B>(
    state: fn(&mut App, &mut AssetManager) -> S,
    backend: B,
) -> AppBuilder<S, B>
where
    S: 'static,
    B: BackendSystem + 'static,
{
    AppBuilder::new(state, backend)
}
