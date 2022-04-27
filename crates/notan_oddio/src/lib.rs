mod backend;
mod decoder;

#[cfg(target_arch = "wasm32")]
mod dummy;

pub use backend::OddioBackend;
