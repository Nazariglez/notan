mod notan;
pub mod prelude;

pub use crate::notan::*;
#[doc(inline)]
pub use notan_app as app;
#[doc(inline)]
pub use notan_graphics as graphics;
#[doc(inline)]
pub use notan_input as input;
#[doc(inline)]
pub use notan_macro::*;
#[doc(inline)]
pub use notan_math as math;
#[doc(inline)]
pub use notan_utils as utils;

#[doc(inline)]
pub use notan_core::events::Event;

#[doc(inline)]
#[cfg(feature = "backend")]
pub use notan_backend as backend;

#[doc(inline)]
#[cfg(feature = "audio")]
pub use notan_audio as audio;

#[doc(inline)]
#[cfg(feature = "log")]
pub use notan_log as log;

#[doc(inline)]
#[cfg(feature = "glyph")]
pub use notan_glyph as glyph;

#[doc(inline)]
#[cfg(feature = "draw")]
pub use notan_draw as draw;

#[doc(inline)]
#[cfg(feature = "egui")]
pub use notan_egui as egui;

#[doc(inline)]
#[cfg(feature = "text")]
pub use notan_text as text;

#[doc(inline)]
#[cfg(feature = "extra")]
pub use notan_extra as extra;

#[doc(inline)]
#[cfg(feature = "random")]
pub use notan_random as random;
