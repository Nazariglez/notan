use crate::app::{App, AppState};
use crate::assets::AssetManager;
use crate::events::Event;
use crate::graphics::Graphics;
use crate::plugins::Plugins;

#[cfg(feature = "glyphs")]
use notan_glyph::GlyphManager;

// Order of params App, AssetManager, Graphics, GlyphManager, Plugins, S, Event

#[cfg(not(feature = "glyphs"))]
notan_macro::handler!(Setup<&mut App, &mut AssetManager, &mut Graphics, &mut Plugins> -> S);
#[cfg(not(feature = "glyphs"))]
notan_macro::handler!(App<&mut App, &mut AssetManager, &mut Plugins, &mut S>);
#[cfg(not(feature = "glyphs"))]
notan_macro::handler!(Event<&mut App, &mut AssetManager, &mut Plugins, &mut S, Event>);
#[cfg(not(feature = "glyphs"))]
notan_macro::handler!(Draw<&mut App, &mut AssetManager, &mut Graphics, &mut Plugins, &mut S>);

#[cfg(feature = "glyphs")]
notan_macro::handler!(Setup<&mut App, &mut AssetManager, &mut Graphics, &mut GlyphManager, &mut Plugins> -> S);
#[cfg(feature = "glyphs")]
notan_macro::handler!(App<&mut App, &mut AssetManager, &mut GlyphManager, &mut Plugins, &mut S>);
#[cfg(feature = "glyphs")]
notan_macro::handler!(Event<&mut App, &mut AssetManager, &mut GlyphManager, &mut Plugins, &mut S, Event>);
#[cfg(feature = "glyphs")]
notan_macro::handler!(Draw<&mut App, &mut AssetManager, &mut Graphics, &mut GlyphManager, &mut Plugins, &mut S>);
